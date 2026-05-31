//! 应用启动器模块
//!
//! 此模块包含与应用启动相关的功能，包括：
//! - 获取文件信息
//! - 启动应用程序
//! - 自动隐藏窗口后启动应用
//! - 提取应用图标

use std::fs;
use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::process::Command;
use std::time::Duration;
use tauri::{AppHandle, Manager};

// 导入项目内部模块
use crate::data::load_app_settings;
use crate::helpers::{extract_file_icon, resolve_shortcut_target};
use crate::models::AppSettings;

fn collect_shortcuts(dir: &Path, shortcuts: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_shortcuts(&path, shortcuts);
            continue;
        }

        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
            .to_lowercase();

        if matches!(extension.as_str(), "lnk" | "url") {
            shortcuts.push(path.to_string_lossy().to_string());
        }
    }
}

#[tauri::command]
pub fn list_start_menu_items() -> Result<Vec<serde_json::Value>, String> {
    let mut roots = Vec::new();

    if let Ok(appdata) = std::env::var("APPDATA") {
        roots.push(Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs"));
    }

    if let Ok(program_data) = std::env::var("ProgramData") {
        roots.push(Path::new(&program_data).join("Microsoft\\Windows\\Start Menu\\Programs"));
    }

    let mut shortcuts = Vec::new();
    for root in roots {
        collect_shortcuts(&root, &mut shortcuts);
    }

    shortcuts.sort();
    shortcuts.dedup();

    let mut items: Vec<serde_json::Value> = shortcuts
        .into_iter()
        .filter_map(|shortcut_path| {
            let path = Path::new(&shortcut_path);
            let name = path.file_stem()?.to_string_lossy().to_string();

            Some(serde_json::json!({
                "id": shortcut_path,
                "name": name,
                "icon": "",
                "path": shortcut_path,
                "target_path": null,
                "target_type": "file",
                "launch_args": ""
            }))
        })
        .collect();

    items.sort_by(|a, b| {
        a.get("name")
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .to_lowercase()
            .cmp(
                &b.get("name")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_lowercase(),
            )
    });

    Ok(items)
}

#[tauri::command]
pub fn get_shell_file_icon(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use base64::engine::general_purpose;
        use base64::Engine;
        use image::{ImageBuffer, ImageEncoder, Rgba};
        use std::mem;
        use std::os::windows::ffi::OsStrExt;
        use winapi::shared::windef::HICON;
        use winapi::um::shellapi::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
        use winapi::um::wingdi::{DeleteObject, GetBitmapBits, GetObjectW, BITMAP};
        use winapi::um::winuser::{DestroyIcon, GetIconInfo, ICONINFO};

        let path_w: Vec<u16> = std::ffi::OsStr::new(&file_path)
            .encode_wide()
            .chain(Some(0))
            .collect();
        let mut file_info: SHFILEINFOW = unsafe { mem::zeroed() };
        let result = unsafe {
            SHGetFileInfoW(
                path_w.as_ptr(),
                0,
                &mut file_info,
                mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_LARGEICON,
            )
        };

        if result == 0 || file_info.hIcon.is_null() {
            return Err("无法获取 Shell 图标".to_string());
        }

        let icon: HICON = file_info.hIcon;
        let icon_result = (|| {
            let mut icon_info: ICONINFO = unsafe { mem::zeroed() };
            if unsafe { GetIconInfo(icon, &mut icon_info) } == 0 {
                return Err("读取图标信息失败".to_string());
            }

            let mut bitmap: BITMAP = unsafe { mem::zeroed() };
            if unsafe {
                GetObjectW(
                    icon_info.hbmColor as _,
                    mem::size_of::<BITMAP>() as i32,
                    &mut bitmap as *mut _ as _,
                )
            } == 0
            {
                unsafe {
                    DeleteObject(icon_info.hbmColor as _);
                    DeleteObject(icon_info.hbmMask as _);
                }
                return Err("读取图标位图失败".to_string());
            }

            let width = bitmap.bmWidth;
            let height = bitmap.bmHeight;
            let stride = bitmap.bmWidthBytes as usize;
            let buffer_size = stride
                .checked_mul(height as usize)
                .ok_or_else(|| "图标位图大小无效".to_string())?;
            let mut buffer = vec![0u8; buffer_size];
            let copied = unsafe {
                GetBitmapBits(
                    icon_info.hbmColor as _,
                    buffer_size as i32,
                    buffer.as_mut_ptr() as _,
                )
            };

            unsafe {
                DeleteObject(icon_info.hbmColor as _);
                DeleteObject(icon_info.hbmMask as _);
            }

            if copied != buffer_size as i32 {
                return Err("复制图标位图失败".to_string());
            }

            let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
            for y in 0..height {
                for x in 0..width {
                    let offset = (y * stride as i32 + x * 4) as usize;
                    if offset + 3 < buffer.len() {
                        rgba_data.push(buffer[offset + 2]);
                        rgba_data.push(buffer[offset + 1]);
                        rgba_data.push(buffer[offset]);
                        rgba_data.push(buffer[offset + 3]);
                    }
                }
            }

            let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, rgba_data)
                .ok_or_else(|| "构建图标图像失败".to_string())?;
            let mut png_bytes = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
            encoder
                .write_image(
                    image.as_raw(),
                    width as u32,
                    height as u32,
                    image::ColorType::Rgba8,
                )
                .map_err(|e| format!("编码图标失败: {}", e))?;

            Ok(format!(
                "data:image/png;base64,{}",
                general_purpose::STANDARD.encode(png_bytes)
            ))
        })();

        unsafe {
            DestroyIcon(icon);
        }

        icon_result
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持 Shell 图标".to_string())
    }
}

/// 获取文件信息
///
/// 此函数用于获取指定文件的详细信息，包括文件名、扩展名、大小等。
/// 对于快捷方式文件，会尝试解析其目标路径并获取目标文件的信息。
#[tauri::command]
pub fn get_file_info(file_path: String) -> Result<serde_json::Value, String> {
    println!("get_file_info called with path: {}", file_path);
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    // 获取文件扩展名
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_shortcut = extension == "lnk" || extension == "url" || extension == "desktop";
    let mut target_path = None;
    let mut actual_path = file_path.clone();

    // 如果是快捷方式，尝试解析目标路径
    if is_shortcut {
        if let Some(resolved_path) = resolve_shortcut_target(&file_path) {
            target_path = Some(resolved_path.clone());
            // 对于快捷方式，使用目标路径来获取文件信息
            if Path::new(&resolved_path).exists() {
                actual_path = resolved_path;
                println!("使用快捷方式目标路径: {}", actual_path);
            }
        }
    }

    // 使用实际路径（对于快捷方式是目标路径，对于普通文件是原路径）来获取信息
    let actual_path_obj = Path::new(&actual_path);

    // 获取文件名（不包含扩展名）
    // 如果是快捷方式，应返回快捷方式文件本身的名字，而不是它指向的目标名字
    let name = if is_shortcut {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("未知应用")
            .to_string()
    } else {
        actual_path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("未知应用")
            .to_string()
    };

    // 获取实际文件的扩展名（对于快捷方式，这将是目标文件的扩展名）
    let actual_extension = actual_path_obj
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 不再限制文件类型，所有文件都可以添加
    let is_executable = true; // 允许所有文件类型

    // 获取文件大小（使用原始文件的大小）
    let metadata = fs::metadata(&path).map_err(|e| format!("获取文件信息失败: {}", e))?;
    let size = metadata.len();

    // 使用 Windows Shell 图标缓存获取图标，避免自行解析 PE 资源造成卡顿。
    let icon_base64 = get_app_icon(file_path.clone())
        .ok()
        .or_else(|| extract_file_icon(&file_path));

    // 生成唯一ID（使用时间戳）
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    Ok(serde_json::json!({
        "id": id,
        "name": name,
        "path": file_path,  // 始终返回原始路径（快捷方式路径）
        "extension": if is_shortcut { actual_extension } else { extension },  // 对于快捷方式返回目标文件的扩展名
        "size": size,
        "is_executable": is_executable,
        "is_shortcut": is_shortcut,
        "target_path": target_path,  // 快捷方式的目标路径
        "icon": icon_base64
    }))
}

/// 启动应用程序
///
/// 此函数用于启动指定的应用程序，支持传递启动参数。
/// 对于快捷方式文件，会先解析其目标路径，然后启动目标应用程序。
#[tauri::command]
pub fn launch_app(app_path: String, launch_args: Option<String>) -> Result<String, String> {
    let path = Path::new(&app_path);

    if !path.exists() {
        return Err("应用文件不存在".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // 检查文件扩展名，如果是快捷方式(.lnk)，解析目标路径
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let actual_path = if extension == "lnk" {
            // 解析快捷方式的目标路径
            if let Some(target) = resolve_shortcut_target(&app_path) {
                println!("解析快捷方式目标路径: {}", target);
                target
            } else {
                return Err("无法解析快捷方式目标路径".to_string());
            }
        } else if extension == "url" {
            // 对于 .url 文件(Internet 快捷方式)，使用系统默认方式打开
            if let Some(url) = resolve_shortcut_target(&app_path) {
                return crate::system::open_url(url, launch_args);
            } else {
                return Err("无法解析URL快捷方式".to_string());
            }
        } else {
            app_path.clone()
        };

        let work_dir = Path::new(&actual_path).parent().and_then(|p| p.to_str());
        let params = launch_args
            .as_deref()
            .filter(|args| !args.trim().is_empty());
        crate::win_native::shell_execute(&actual_path, params, work_dir, Some("open"))
            .map(|_| "应用启动成功".to_string())
            .map_err(|e| format!("启动应用失败: {}", e))
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = Command::new(&app_path);

        // 重定向输出和错误，防止子进程崩溃影响父进程
        cmd.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                cmd.args(&split_args);
            }
        }

        let result = cmd.spawn();

        match result {
            Ok(_) => Ok("应用启动成功".to_string()),
            Err(e) => Err(format!("启动应用失败: {}", e)),
        }
    }
}

/// 启动应用并检查是否需要自动隐藏窗口
///
/// 此函数先启动应用程序，然后根据设置决定是否自动隐藏主窗口。
/// 如果启用了自动隐藏功能，会在应用启动后延迟一段时间再隐藏窗口。
#[tauri::command]
pub async fn launch_app_with_auto_hide(
    app: AppHandle,
    app_path: String,
    launch_args: Option<String>,
) -> Result<String, String> {
    // 先启动应用
    let launch_result = launch_app(app_path, launch_args)?;

    // 检查是否启用了自动隐藏功能
    let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());
    if settings.auto_hide_after_launch.unwrap_or(false) {
        // 延迟一段时间后隐藏窗口，让用户能看到应用已启动
        if let Some(window) = app.get_webview_window("main") {
            // 使用 tokio 的延迟功能
            tokio::time::sleep(Duration::from_millis(300)).await;
            let _: Result<(), _> = window.hide();
        }
    }

    Ok(launch_result)
}

/// 获取应用图标
///
/// 此函数用于从可执行文件中提取图标，并将其转换为 base64 编码的 PNG 图像。
/// 主要用于 Windows 平台，使用 Windows API 提取图标。
#[tauri::command]
pub fn get_app_icon(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // 如果传入的是 URL（网站），尝试使用 helpers::fetch_favicon
        if file_path.starts_with("http://") || file_path.starts_with("https://") {
            return crate::helpers::fetch_favicon(&file_path);
        }

        if !Path::new(&file_path).exists() {
            return Err("文件不存在，无法提取图标".to_string());
        }

        get_shell_file_icon(file_path)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持图标提取".to_string())
    }
}

/// 获取默认设置
///
/// 此函数返回应用程序的默认设置，用于在无法加载设置时提供后备值。
fn get_default_settings() -> AppSettings {
    AppSettings {
        prevent_auto_hide: false,
        window_width: Some(800),
        window_height: Some(600),
        settings_window_width: Some(800),
        settings_window_height: Some(600),
        window_layout: Some("horizontal".to_string()),
        theme: Some("auto".to_string()),
        icon_size: Some(88),
        project_name_position: Some("bottom".to_string()),
        sidebar_width: Some(0),
        enable_animations: Some(true),
        animation_speed: Some("normal".to_string()),
        start_with_system: Some(false),
        start_minimized: Some(false),
        auto_hide_after_launch: Some(false),
        toggle_hotkey: Some("Ctrl+Space".to_string()),
        global_hotkey: Some(true),
        fuzzy_search: Some(true),
        search_in_path: Some(false),
        max_search_results: Some(20),
        auto_backup: Some(true),
        backup_interval: Some("weekly".to_string()),
        last_backup_time: None,
        next_backup_time: None,
        // 界面状态记录默认值
        active_tab: Some("about".to_string()),
        last_selected_category: None,
        window_position_x: None,
        window_position_y: None,
        last_search_query: None,
        grid_view_enabled: Some(false),
        sort_order: Some("manual".to_string()),
        show_hidden_files: Some(false),
    }
}
