//! Desktop workspaces, following PowerToys' capture → launch → arrange approach.
//! References: microsoft/PowerToys/src/modules/Workspaces/WorkspacesWindowArranger/
//! WindowArranger.cpp. Uses documented Win32 APIs; does not recreate Shell Snap Groups.
use crate::data::{load_app_data, write_app_storage};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub move_existing_windows: bool,
    pub windows: Vec<WorkspaceWindow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceWindow {
    pub name: String,
    pub title: String,
    pub path: String,
    pub launch_args: String,
    pub run_as_admin: bool,
    #[serde(default)]
    pub elevated: bool,
    pub monitor: String,
    #[serde(default)]
    pub monitor_device: String,
    // Fractions of the monitor's work area, adapting to resolution and DPI changes.
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
    pub minimized: bool,
}

#[derive(Serialize)]
pub struct CapturedWindow {
    pub key: String,
    pub window: WorkspaceWindow,
}

#[derive(Serialize)]
pub struct LaunchResult {
    pub name: String,
    pub status: String,
    pub error: Option<String>,
    pub error_code: Option<String>,
}

static LAUNCHING: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn list_workspaces() -> Result<Vec<Workspace>, String> {
    Ok(load_app_data()?.workspaces)
}

pub(crate) fn validate_workspaces(workspaces: &[Workspace]) -> Result<(), String> {
    let mut ids = std::collections::HashSet::new();
    for workspace in workspaces {
        if workspace.id.is_empty() || !ids.insert(&workspace.id) {
            return Err("工作区 ID 无效或重复".into());
        }
        if workspace.name.trim().is_empty() || workspace.name.chars().count() > 100 {
            return Err("工作区名称须为 1–100 个字符".into());
        }
        if workspace.windows.is_empty() || workspace.windows.len() > 50 {
            return Err("工作区须包含 1–50 个窗口".into());
        }
        for window in &workspace.windows {
            if window.path.is_empty()
                || window.path.contains('\0')
                || window.launch_args.contains('\0')
            {
                return Err("工作区应用路径或参数无效".into());
            }
            if ![window.x, window.y, window.width, window.height]
                .iter()
                .all(|n| n.is_finite())
                || window.width <= 0.0
                || window.height <= 0.0
                || window.width > 4.0
                || window.height > 4.0
                || window.x.abs() > 4.0
                || window.y.abs() > 4.0
            {
                return Err("工作区窗口位置或尺寸无效".into());
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn save_workspace(workspace: Workspace) -> Result<(), String> {
    validate_workspaces(std::slice::from_ref(&workspace))?;
    let mut storage = load_app_data()?;
    if let Some(existing) = storage.workspaces.iter_mut().find(|w| w.id == workspace.id) {
        *existing = workspace;
    } else {
        storage.workspaces.push(workspace);
    }
    write_app_storage(&storage)?;
    Ok(())
}

#[tauri::command]
pub fn delete_workspace(workspace_id: String) -> Result<(), String> {
    let mut storage = load_app_data()?;
    storage.workspaces.retain(|w| w.id != workspace_id);
    write_app_storage(&storage)?;
    Ok(())
}

#[tauri::command]
pub async fn capture_workspace_windows() -> Result<Vec<CapturedWindow>, String> {
    #[cfg(target_os = "windows")]
    return tauri::async_runtime::spawn_blocking(native::capture)
        .await
        .map_err(|e| e.to_string())?;
    #[cfg(not(target_os = "windows"))]
    Err("工作区窗口管理仅支持 Windows".into())
}

#[tauri::command]
pub async fn launch_workspace(
    app: tauri::AppHandle,
    workspace_id: String,
) -> Result<Vec<LaunchResult>, String> {
    let workspace = load_app_data()?
        .workspaces
        .into_iter()
        .find(|w| w.id == workspace_id)
        .ok_or("工作区不存在")?;
    validate_workspaces(std::slice::from_ref(&workspace))?;
    if LAUNCHING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Err("另一个工作区正在启动，请稍后重试".into());
    }
    // Guard belongs to the worker: dropping an invoke future must not allow two arrangers.
    struct LaunchGuard;
    impl Drop for LaunchGuard {
        fn drop(&mut self) {
            LAUNCHING.store(false, Ordering::SeqCst);
        }
    }
    let guard = LaunchGuard;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        #[cfg(target_os = "windows")]
        return native::launch(workspace, app);
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (workspace, app);
            Err("工作区窗口管理仅支持 Windows".into())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(target_os = "windows")]
mod native {
    use super::*;
    use std::{
        collections::HashSet,
        mem, ptr,
        time::{Duration, Instant},
    };
    use tauri::Emitter;
    use winapi::{
        shared::{
            minwindef::{BOOL, DWORD, LPARAM, TRUE},
            ntdef::HANDLE,
            windef::{
                DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, HDC, HMONITOR,
                HWND, RECT,
            },
        },
        um::{
            dwmapi::{DwmGetWindowAttribute, DWMWA_CLOAKED},
            handleapi::CloseHandle,
            processthreadsapi::{GetCurrentProcess, OpenProcess, OpenProcessToken},
            securitybaseapi::GetTokenInformation,
            winbase::QueryFullProcessImageNameW,
            wingdi::DISPLAY_DEVICEW,
            winnt::{
                TokenElevation, PROCESS_QUERY_LIMITED_INFORMATION, TOKEN_ELEVATION, TOKEN_QUERY,
            },
            winuser::*,
        },
    };

    struct Monitor {
        id: String,
        device: String,
        bounds: RECT,
        work: RECT,
    }
    struct LiveWindow {
        hwnd: HWND,
        data: WorkspaceWindow,
    }

    // Token queries are advisory. If access is denied, still let the window API report its error.
    fn process_elevated(process: HANDLE) -> Option<bool> {
        unsafe {
            let mut token = ptr::null_mut();
            if OpenProcessToken(process, TOKEN_QUERY, &mut token) == 0 {
                return None;
            }
            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut length = 0;
            let ok = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as _,
                mem::size_of::<TOKEN_ELEVATION>() as DWORD,
                &mut length,
            );
            CloseHandle(token);
            if ok == 0 {
                None
            } else {
                Some(elevation.TokenIsElevated != 0)
            }
        }
    }

    struct ArrangeError {
        message: String,
        code: Option<&'static str>,
    }
    impl ArrangeError {
        fn permission_denied() -> Self {
            Self { message: "权限不足，无法恢复此窗口的位置。请退出 Lora，以管理员身份运行 Lora 后重试。仅勾选应用的“以管理员身份启动”不能提升 Lora 的权限。".into(), code: Some("permission_denied") }
        }
        fn window_api(operation: &str) -> Self {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() == Some(5) {
                Self::permission_denied()
            } else {
                Self {
                    message: format!("{}: {}", operation, error),
                    code: None,
                }
            }
        }
        fn record(self, result: &mut LaunchResult) {
            result.status = "failed".into();
            result.error = Some(self.message);
            result.error_code = self.code.map(str::to_string);
        }
    }

    struct DpiGuard(DPI_AWARENESS_CONTEXT);
    impl DpiGuard {
        fn enter() -> Self {
            Self(unsafe {
                SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
            })
        }
    }
    impl Drop for DpiGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    SetThreadDpiAwarenessContext(self.0);
                }
            }
        }
    }

    fn wide(buffer: &[u16]) -> String {
        String::from_utf16_lossy(
            &buffer[..buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len())],
        )
    }

    fn monitor_info(handle: HMONITOR) -> Option<Monitor> {
        let mut info: MONITORINFOEXW = unsafe { mem::zeroed() };
        info.cbSize = mem::size_of::<MONITORINFOEXW>() as DWORD;
        if unsafe { GetMonitorInfoW(handle, &mut info as *mut _ as _) } == 0 {
            return None;
        }
        let mut display: DISPLAY_DEVICEW = unsafe { mem::zeroed() };
        display.cb = mem::size_of::<DISPLAY_DEVICEW>() as DWORD;
        let device = if unsafe {
            EnumDisplayDevicesW(
                info.szDevice.as_ptr(),
                0,
                &mut display,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
        } != 0
        {
            wide(&display.DeviceID)
        } else {
            String::new()
        };
        Some(Monitor {
            id: wide(&info.szDevice),
            device,
            bounds: info.rcMonitor,
            work: info.rcWork,
        })
    }

    unsafe extern "system" fn collect_monitor(
        handle: HMONITOR,
        _: HDC,
        _: *mut RECT,
        param: LPARAM,
    ) -> BOOL {
        if let Some(monitor) = monitor_info(handle) {
            (&mut *(param as *mut Vec<Monitor>)).push(monitor);
        }
        TRUE
    }

    fn monitors() -> Result<Vec<Monitor>, String> {
        let mut result = Vec::new();
        if unsafe {
            EnumDisplayMonitors(
                ptr::null_mut(),
                ptr::null(),
                Some(collect_monitor),
                &mut result as *mut _ as LPARAM,
            )
        } == 0
        {
            return Err("无法枚举显示器".into());
        }
        if result.is_empty() {
            return Err("没有可用的显示器".into());
        }
        Ok(result)
    }

    fn read_window(hwnd: HWND) -> Option<WorkspaceWindow> {
        unsafe {
            if IsWindowVisible(hwnd) == 0
                || !GetWindow(hwnd, GW_OWNER).is_null()
                || GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_TOOLWINDOW != 0
            {
                return None;
            }
            let mut cloaked: DWORD = 0;
            DwmGetWindowAttribute(
                hwnd,
                DWMWA_CLOAKED,
                &mut cloaked as *mut _ as _,
                mem::size_of::<DWORD>() as DWORD,
            );
            if cloaked != 0 {
                return None;
            }
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            if pid == 0 || pid == std::process::id() {
                return None;
            }
            let length = GetWindowTextLengthW(hwnd);
            if length <= 0 {
                return None;
            }
            let mut title = vec![0u16; length as usize + 1];
            GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);
            let mut class_name = [0u16; 256];
            GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
            if matches!(
                wide(&class_name).as_str(),
                "Progman" | "WorkerW" | "Shell_TrayWnd" | "Shell_SecondaryTrayWnd"
            ) {
                return None;
            }
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if process.is_null() {
                return None;
            }
            let mut path = vec![0u16; 32768];
            let mut size = path.len() as DWORD;
            let ok = QueryFullProcessImageNameW(process, 0, path.as_mut_ptr(), &mut size);
            let elevated = process_elevated(process).unwrap_or(false);
            CloseHandle(process);
            if ok == 0 {
                return None;
            }
            let path = wide(&path[..size as usize]);
            // A host executable cannot relaunch the packaged app it happens to display.
            if path.to_lowercase().ends_with("\\applicationframehost.exe") {
                return None;
            }
            let monitor = monitor_info(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST))?;
            let mut placement: WINDOWPLACEMENT = mem::zeroed();
            placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
            if GetWindowPlacement(hwnd, &mut placement) == 0 {
                return None;
            }
            let minimized = IsIconic(hwnd) != 0;
            let maximized =
                IsZoomed(hwnd) != 0 || (minimized && placement.flags & WPF_RESTORETOMAXIMIZED != 0);
            let mut rect = placement.rcNormalPosition;
            if !minimized && !maximized {
                if GetWindowRect(hwnd, &mut rect) == 0 {
                    return None;
                }
            } else {
                // WINDOWPLACEMENT uses workspace coordinates, GetWindowRect screen coordinates.
                let dx = monitor.work.left - monitor.bounds.left;
                let dy = monitor.work.top - monitor.bounds.top;
                rect.left += dx;
                rect.right += dx;
                rect.top += dy;
                rect.bottom += dy;
            }
            let w = f64::from(monitor.work.right - monitor.work.left);
            let h = f64::from(monitor.work.bottom - monitor.work.top);
            if w <= 0.0 || h <= 0.0 || rect.right <= rect.left || rect.bottom <= rect.top {
                return None;
            }
            let name = std::path::Path::new(&path)
                .file_stem()?
                .to_string_lossy()
                .into_owned();
            Some(WorkspaceWindow {
                name,
                title: wide(&title),
                path,
                launch_args: String::new(),
                run_as_admin: false,
                elevated,
                monitor: monitor.id,
                monitor_device: monitor.device,
                x: f64::from(rect.left - monitor.work.left) / w,
                y: f64::from(rect.top - monitor.work.top) / h,
                width: f64::from(rect.right - rect.left) / w,
                height: f64::from(rect.bottom - rect.top) / h,
                maximized,
                minimized,
            })
        }
    }

    unsafe extern "system" fn collect_window(hwnd: HWND, param: LPARAM) -> BOOL {
        if let Some(data) = read_window(hwnd) {
            (&mut *(param as *mut Vec<LiveWindow>)).push(LiveWindow { hwnd, data });
        }
        TRUE
    }

    fn windows() -> Result<Vec<LiveWindow>, String> {
        let mut result = Vec::new();
        if unsafe { EnumWindows(Some(collect_window), &mut result as *mut _ as LPARAM) } == 0 {
            return Err("无法枚举应用窗口".into());
        }
        Ok(result)
    }

    pub(super) fn capture() -> Result<Vec<CapturedWindow>, String> {
        let _dpi = DpiGuard::enter();
        let apps = load_app_data()?.apps;
        Ok(windows()?
            .into_iter()
            .map(|mut live| {
                // Preserve launcher arguments, shortcut working directory, and elevation preferences.
                if let Some(app) = apps.iter().find(|app| {
                    app.target_path
                        .as_deref()
                        .unwrap_or(&app.path)
                        .eq_ignore_ascii_case(&live.data.path)
                }) {
                    live.data.name = app.name.clone();
                    live.data.path = app.path.clone();
                    live.data.launch_args = app.launch_args.clone().unwrap_or_default();
                    live.data.run_as_admin = app.run_as_admin;
                }
                CapturedWindow {
                    key: format!("{:x}", live.hwnd as usize),
                    window: live.data,
                }
            })
            .collect())
    }

    fn executable(window: &WorkspaceWindow) -> String {
        if window.path.to_lowercase().ends_with(".lnk") {
            crate::helpers::resolve_shortcut_target(&window.path)
                .unwrap_or_else(|| window.path.clone())
        } else {
            window.path.clone()
        }
    }

    fn find_window<'a>(
        saved: &WorkspaceWindow,
        path: &str,
        live: &'a [LiveWindow],
        used: &HashSet<usize>,
        excluded: &HashSet<usize>,
    ) -> Option<&'a LiveWindow> {
        live.iter()
            .filter(|w| {
                !used.contains(&(w.hwnd as usize))
                    && !excluded.contains(&(w.hwnd as usize))
                    && w.data.path.eq_ignore_ascii_case(path)
            })
            .min_by_key(|w| {
                let distance = (w.data.x - saved.x).abs()
                    + (w.data.y - saved.y).abs()
                    + (w.data.width - saved.width).abs()
                    + (w.data.height - saved.height).abs();
                (
                    w.data.title != saved.title,
                    w.data.monitor != saved.monitor,
                    (distance * 10000.0) as u64,
                )
            })
    }

    fn arrange(
        hwnd: HWND,
        saved: &WorkspaceWindow,
        monitors: &[Monitor],
    ) -> Result<(), ArrangeError> {
        // Existing workspaces may predate elevation metadata, so check the actual live process.
        unsafe {
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if !process.is_null() {
                let target_elevated = process_elevated(process);
                CloseHandle(process);
                if target_elevated == Some(true)
                    && process_elevated(GetCurrentProcess()) == Some(false)
                {
                    return Err(ArrangeError::permission_denied());
                }
            }
        }
        let current = monitor_info(unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) });
        let monitor = monitors
            .iter()
            .find(|m| !saved.monitor_device.is_empty() && m.device == saved.monitor_device)
            .or_else(|| monitors.iter().find(|m| m.id == saved.monitor))
            .or_else(|| {
                current
                    .as_ref()
                    .and_then(|c| monitors.iter().find(|m| m.id == c.id))
            })
            .unwrap_or(&monitors[0]);
        let w = monitor.work.right - monitor.work.left;
        let h = monitor.work.bottom - monitor.work.top;
        let width = (saved.width * f64::from(w))
            .round()
            .clamp(100.0, f64::from(w.max(100))) as i32;
        let height = (saved.height * f64::from(h))
            .round()
            .clamp(80.0, f64::from(h.max(80))) as i32;
        let left = monitor.work.left
            + (saved.x * f64::from(w))
                .round()
                .clamp(0.0, f64::from((w - width).max(0))) as i32;
        let top = monitor.work.top
            + (saved.y * f64::from(h))
                .round()
                .clamp(0.0, f64::from((h - height).max(0))) as i32;
        let dx = monitor.work.left - monitor.bounds.left;
        let dy = monitor.work.top - monitor.bounds.top;
        unsafe {
            let mut placement: WINDOWPLACEMENT = mem::zeroed();
            placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
            if GetWindowPlacement(hwnd, &mut placement) == 0 {
                return Err(ArrangeError::window_api("读取窗口状态失败"));
            }
            placement.rcNormalPosition = RECT {
                left: left - dx,
                top: top - dy,
                right: left + width - dx,
                bottom: top + height - dy,
            };
            placement.flags = WPF_ASYNCWINDOWPLACEMENT;
            placement.showCmd = SW_RESTORE as u32;
            if SetWindowPlacement(hwnd, &placement) == 0 {
                return Err(ArrangeError::window_api("调整窗口失败"));
            }
            // PowerToys places on the target monitor before maximizing, then applies again for DPI.
            if saved.minimized {
                placement.showCmd = SW_SHOWMINIMIZED as u32;
                if saved.maximized {
                    placement.flags |= WPF_RESTORETOMAXIMIZED;
                }
            } else if saved.maximized {
                placement.showCmd = SW_SHOWMAXIMIZED as u32;
            }
            if SetWindowPlacement(hwnd, &placement) == 0 {
                return Err(ArrangeError::window_api("恢复窗口状态失败"));
            }
        }
        Ok(())
    }

    pub(super) fn launch(
        workspace: Workspace,
        app: tauri::AppHandle,
    ) -> Result<Vec<LaunchResult>, String> {
        let _dpi = DpiGuard::enter();
        let progress = |index: usize, result: &LaunchResult| {
            let _ = app.emit(
                "workspace-launch-progress",
                serde_json::json!({
                    "workspace_id": workspace.id, "index": index, "result": result
                }),
            );
        };
        let monitors = monitors()?;
        let before = windows()?;
        let excluded: HashSet<usize> = if workspace.move_existing_windows {
            HashSet::new()
        } else {
            before.iter().map(|w| w.hwnd as usize).collect()
        };
        let mut used = HashSet::new();
        let paths: Vec<_> = workspace.windows.iter().map(executable).collect();
        let mut results: Vec<_> = workspace
            .windows
            .iter()
            .map(|w| LaunchResult {
                name: w.name.clone(),
                status: "waiting".into(),
                error: None,
                error_code: None,
            })
            .collect();
        let mut pending = Vec::new();
        let mut settling: Vec<(usize, usize, Instant)> = Vec::new();
        // Reserve exact title matches first, so a fallback cannot take another project's window.
        let mut launch_order: Vec<usize> = (0..workspace.windows.len()).collect();
        launch_order.sort_by_key(|index| {
            !before.iter().any(|live| {
                live.data.path.eq_ignore_ascii_case(&paths[*index])
                    && live.data.title == workspace.windows[*index].title
            })
        });
        for index in launch_order {
            let saved = &workspace.windows[index];
            if let Some(live) = find_window(saved, &paths[index], &before, &used, &excluded) {
                used.insert(live.hwnd as usize);
                match arrange(live.hwnd, saved, &monitors) {
                    Ok(()) => results[index].status = "restored".into(),
                    Err(error) => error.record(&mut results[index]),
                }
            } else {
                results[index].status = "launching".into();
                progress(index, &results[index]);
                match crate::app_launcher::launch_app(
                    saved.path.clone(),
                    Some(saved.launch_args.clone()),
                    Some(saved.run_as_admin),
                ) {
                    Ok(_) => pending.push(index),
                    Err(error) => {
                        results[index].status = "failed".into();
                        results[index].error = Some(error);
                    }
                }
            }
            progress(index, &results[index]);
        }
        // Launch all missing apps first; wait for actual windows instead of a fixed startup sleep.
        let deadline = Instant::now() + Duration::from_secs(30);
        while (!pending.is_empty() || !settling.is_empty()) && Instant::now() < deadline {
            let live = windows()?;
            // Splash windows can disappear. Re-match their app rather than treating them as done.
            settling.retain(|(index, handle, since)| {
                let window = live.iter().find(|w| {
                    w.hwnd as usize == *handle && w.data.path.eq_ignore_ascii_case(&paths[*index])
                });
                let Some(window) = window else {
                    used.remove(handle);
                    pending.push(*index);
                    results[*index].status = "launching".into();
                    progress(*index, &results[*index]);
                    return false;
                };
                if since.elapsed() < Duration::from_millis(1500) {
                    return true;
                }
                // Apply once more after startup, as applications often restore their own bounds.
                match arrange(window.hwnd, &workspace.windows[*index], &monitors) {
                    Ok(()) => results[*index].status = "restored".into(),
                    Err(error) => error.record(&mut results[*index]),
                }
                progress(*index, &results[*index]);
                false
            });
            pending.retain(|index| {
                let saved = &workspace.windows[*index];
                if let Some(window) = find_window(saved, &paths[*index], &live, &used, &excluded) {
                    used.insert(window.hwnd as usize);
                    match arrange(window.hwnd, saved, &monitors) {
                        Ok(()) => {
                            results[*index].status = "arranging".into();
                            settling.push((*index, window.hwnd as usize, Instant::now()));
                        }
                        Err(error) => error.record(&mut results[*index]),
                    }
                    progress(*index, &results[*index]);
                    false
                } else {
                    true
                }
            });
            if !pending.is_empty() || !settling.is_empty() {
                std::thread::sleep(Duration::from_millis(300));
            }
        }
        for index in pending
            .into_iter()
            .chain(settling.into_iter().map(|(index, _, _)| index))
        {
            results[index].status = "failed".into();
            results[index].error =
                Some("等待应用窗口超时；请检查启动参数、权限，或启用复用已有窗口".into());
            progress(index, &results[index]);
        }
        Ok(results)
    }
}
