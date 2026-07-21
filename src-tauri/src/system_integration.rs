//! 系统集成模块
//!
//! 此模块包含与操作系统集成的功能，包括：
//! - Windows 开机自启动设置
//! - 系统托盘管理
//! - 全局快捷键处理
//! - 系统事件处理

use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder},
    AppHandle, Emitter, Manager, PhysicalPosition, Position,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::data::{load_app_data, load_app_settings};
use crate::models::{AppData, AppSettings};

#[cfg(target_os = "windows")]
mod mouse_invocation {
    use std::os::raw::c_int;
    use std::ptr::null_mut;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    };

    use tauri::AppHandle;
    use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
    use winapi::um::winuser::{
        CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
        UnhookWindowsHookEx, HC_ACTION, MSG, WH_MOUSE_LL, WM_MBUTTONDOWN,
    };

    static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
    static ENABLED: AtomicBool = AtomicBool::new(false);
    static HOOK_STARTED: AtomicBool = AtomicBool::new(false);

    unsafe extern "system" fn mouse_hook(
        code: c_int,
        message: WPARAM,
        event_data: LPARAM,
    ) -> LRESULT {
        if code == HC_ACTION
            && message == WM_MBUTTONDOWN as WPARAM
            && ENABLED.load(Ordering::Relaxed)
        {
            if let Some(app) = APP_HANDLE.get().cloned() {
                tauri::async_runtime::spawn(async move {
                    if let Err(error) = crate::window_manager::toggle_window_visibility(app).await {
                        eprintln!("鼠标中键切换窗口失败: {}", error);
                    }
                });
            }
        }

        CallNextHookEx(null_mut(), code, message, event_data)
    }

    pub fn initialize(app: &AppHandle, enabled: bool) {
        let _ = APP_HANDLE.set(app.clone());
        ENABLED.store(enabled, Ordering::Relaxed);

        if HOOK_STARTED.swap(true, Ordering::AcqRel) {
            return;
        }

        std::thread::spawn(|| unsafe {
            let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), null_mut(), 0);
            if hook.is_null() {
                HOOK_STARTED.store(false, Ordering::Release);
                eprintln!("安装全局鼠标钩子失败");
                return;
            }

            let mut message: MSG = std::mem::zeroed();
            while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }

            UnhookWindowsHookEx(hook);
            HOOK_STARTED.store(false, Ordering::Release);
        });
    }

    pub fn set_enabled(enabled: bool) {
        ENABLED.store(enabled, Ordering::Relaxed);
    }
}

/// 初始化可选的全局鼠标中键呼出功能。
pub fn initialize_mouse_invocation(app: &AppHandle) {
    let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());
    let enabled = settings.middle_mouse_toggle.unwrap_or(false);

    #[cfg(target_os = "windows")]
    mouse_invocation::initialize(app, enabled);

    #[cfg(not(target_os = "windows"))]
    let _ = (app, enabled);
}

/// 立即更新鼠标中键呼出的运行时状态，无需重启应用。
pub fn set_middle_mouse_toggle_enabled(enabled: bool) {
    #[cfg(target_os = "windows")]
    mouse_invocation::set_enabled(enabled);

    #[cfg(not(target_os = "windows"))]
    let _ = enabled;
}

/// Windows 开机自启动实现
#[cfg(target_os = "windows")]
pub fn set_auto_start_windows(enable: bool) -> Result<(), String> {
    // 获取当前可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;
    let exe_path_str = exe_path.to_str().ok_or("可执行文件路径包含无效字符")?;

    crate::win_native::set_auto_start("Lora", exe_path_str, enable)
}

/// 更新托盘菜单项
#[tauri::command]
pub async fn update_tray_menu(_app: AppHandle, _prevent_auto_hide: bool) -> Result<String, String> {
    // 托盘菜单已改为前端自绘窗口，状态由 tray-menu 页面直接读取设置。
    Ok("托盘菜单已更新".to_string())
}

/// 初始化系统托盘
pub fn initialize_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建托盘图标
    let _tray = TrayIconBuilder::with_id("main_tray")
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(false)
        .tooltip("Lora Launcher")
        .on_tray_icon_event(|tray, event| {
            handle_tray_icon_event(tray, event);
        })
        .build(app)?;

    Ok(())
}

/// 处理托盘图标事件
fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: tauri::tray::TrayIconEvent) {
    match event {
        tauri::tray::TrayIconEvent::Click {
            button,
            button_state,
            position,
            ..
        } => {
            if button_state != MouseButtonState::Down {
                return;
            }

            if button == MouseButton::Left {
                println!("托盘图标左键点击");
                // 左键点击只显示窗口，不隐藏（与设置菜单行为一致）
                if let Some(main_window) = tray.app_handle().get_webview_window("main") {
                    println!("找到主窗口，显示并聚焦");
                    if let Err(e) = main_window.show() {
                        eprintln!("显示主窗口失败: {}", e);
                    } else if let Err(e) = main_window.set_focus() {
                        eprintln!("聚焦主窗口失败: {}", e);
                    } else {
                        // 窗口显示时，隐藏任务栏图标
                        if let Err(e) = main_window.set_skip_taskbar(true) {
                            eprintln!("设置隐藏任务栏图标失败: {}", e);
                        } else {
                            println!("托盘左键点击：主窗口已显示并聚焦");
                        }
                    }
                } else {
                    eprintln!("托盘左键点击：无法找到主窗口");
                }
            } else if button == MouseButton::Right {
                show_custom_tray_menu(tray.app_handle(), position);
            }
        }
        _ => {}
    }
}

fn show_custom_tray_menu(app: &AppHandle, position: PhysicalPosition<f64>) {
    let Some(menu_window) = app.get_webview_window("tray-menu") else {
        eprintln!("托盘菜单窗口不存在");
        return;
    };

    const MENU_WIDTH: i32 = 164;
    const MENU_HEIGHT: i32 = 104;
    const GAP: i32 = 8;

    let mut x = position.x.round() as i32 - MENU_WIDTH + 20;
    let mut y = position.y.round() as i32 - MENU_HEIGHT - GAP;

    if let Ok(Some(monitor)) = menu_window.monitor_from_point(position.x, position.y) {
        let work_area = monitor.work_area();
        let min_x = work_area.position.x;
        let min_y = work_area.position.y;
        let max_x = work_area.position.x + work_area.size.width as i32 - MENU_WIDTH;
        let max_y = work_area.position.y + work_area.size.height as i32 - MENU_HEIGHT;

        x = x.clamp(min_x, max_x);
        y = y.clamp(min_y, max_y);
    }

    let _ = menu_window.set_position(Position::Physical(PhysicalPosition::new(x, y)));
    let _ = menu_window.emit("tray-menu-refresh", {});
    if let Err(e) = menu_window.show() {
        eprintln!("显示托盘菜单失败: {}", e);
    } else if let Err(e) = menu_window.set_focus() {
        eprintln!("聚焦托盘菜单失败: {}", e);
    }
}

/// 初始化全局快捷键
pub fn initialize_global_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    refresh_global_shortcuts(app).map_err(|error| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, error))
            as Box<dyn std::error::Error>
    })
}

/// 重新注册窗口呼出与所有单项目快捷键。
pub fn refresh_global_shortcuts(app: &AppHandle) -> Result<(), String> {
    let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());
    let storage = load_app_data()?;
    let mut registrations: Vec<(Shortcut, String)> = Vec::new();

    if settings.global_hotkey.unwrap_or(true) {
        if let Some(hotkey) = &settings.toggle_hotkey {
            if !hotkey.is_empty() {
                let shortcut = hotkey
                    .parse::<Shortcut>()
                    .map_err(|error| format!("窗口呼出快捷键格式无效: {}", error))?;
                registrations.push((shortcut, "窗口呼出".to_string()));
            }
        }
    }

    for project in storage.apps {
        let Some(hotkey) = project
            .shortcut_hotkey
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };

        let shortcut = hotkey
            .parse::<Shortcut>()
            .map_err(|error| format!("项目“{}”的快捷键格式无效: {}", project.name, error))?;
        if shortcut.mods.is_empty() {
            return Err(format!("项目“{}”的快捷键缺少修饰键", project.name));
        }
        registrations.push((shortcut, format!("项目“{}”", project.name)));
    }

    let mut registered_ids = std::collections::HashSet::new();
    for (shortcut, label) in &registrations {
        if !registered_ids.insert(shortcut.id()) {
            return Err(format!("{}的快捷键与其他绑定冲突", label));
        }
    }

    app.global_shortcut()
        .unregister_all()
        .map_err(|error| format!("清理全局快捷键失败: {}", error))?;

    for (shortcut, label) in registrations {
        app.global_shortcut()
            .register(shortcut)
            .map_err(|error| format!("注册{}快捷键失败: {}", label, error))?;
    }

    Ok(())
}

#[tauri::command]
pub fn refresh_shortcut_registrations(app: AppHandle) -> Result<String, String> {
    refresh_global_shortcuts(&app)?;
    Ok("快捷键注册已刷新".to_string())
}

fn shortcut_id(value: &str) -> Option<u32> {
    value.parse::<Shortcut>().ok().map(|shortcut| shortcut.id())
}

fn launch_project_from_shortcut(app: &AppHandle, project: &AppData) {
    let target_path = if project.path.to_lowercase().ends_with(".lnk")
        || project.path.to_lowercase().ends_with(".url")
    {
        project.path.clone()
    } else {
        project
            .target_path
            .clone()
            .unwrap_or_else(|| project.path.clone())
    };
    let launch_args = project.launch_args.clone();

    let result = if project.target_type.as_deref() == Some("url") {
        crate::system::open_url(target_path, launch_args)
    } else if project.target_type.as_deref() == Some("folder") && !project.run_as_admin {
        crate::system::open_folder(target_path, launch_args)
    } else {
        crate::app_launcher::launch_app(target_path, launch_args, Some(project.run_as_admin))
    };

    match result {
        Ok(_) => {
            if let Err(error) = crate::data::increment_app_usage(project.id) {
                eprintln!("更新项目快捷键使用次数失败: {}", error);
            }
            let _ = app.emit("data-updated", {});
        }
        Err(error) => eprintln!("项目快捷键启动“{}”失败: {}", project.name, error),
    }
}

/// 创建全局快捷键处理器
pub fn create_global_shortcut_handler(
) -> impl Fn(&AppHandle, &Shortcut, tauri_plugin_global_shortcut::ShortcutEvent) {
    |app, shortcut, event| {
        // 只处理按键按下事件，忽略松开事件
        if event.state() != ShortcutState::Pressed {
            return;
        }

        println!("快捷键按下触发: {:?}, 事件: {:?}", shortcut, event);

        let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());
        let is_window_toggle = settings.global_hotkey.unwrap_or(true)
            && settings.toggle_hotkey.as_deref().and_then(shortcut_id) == Some(shortcut.id());

        if !is_window_toggle {
            let project = load_app_data().ok().and_then(|storage| {
                storage.apps.into_iter().find(|project| {
                    project.shortcut_hotkey.as_deref().and_then(shortcut_id) == Some(shortcut.id())
                })
            });

            if let Some(project) = project {
                launch_project_from_shortcut(app, &project);
            }
            return;
        }

        // 尝试多种方式获取主窗口
        let window = app.get_webview_window("main").or_else(|| {
            println!("主窗口'main'未找到，尝试查找其他窗口");
            // 获取所有窗口并打印
            let windows = app.webview_windows();
            println!("当前可用窗口数量: {}", windows.len());
            for (label, _) in &windows {
                println!("发现窗口: {}", label);
            }
            // 尝试获取第一个窗口
            windows.into_iter().next().map(|(_, window)| window)
        });

        if let Some(window) = window {
            println!("找到窗口，当前状态检查...");
            match window.is_visible() {
                Ok(visible) => {
                    println!("窗口可见状态: {}", visible);
                    if visible {
                        println!("隐藏窗口...");
                        if let Err(e) = window.hide() {
                            eprintln!("隐藏窗口失败: {}", e);
                        } else {
                            println!("窗口已隐藏");
                            // 窗口隐藏时，显示任务栏图标
                            if let Err(e) = window.set_skip_taskbar(false) {
                                eprintln!("设置显示任务栏图标失败: {}", e);
                            } else {
                                println!("已设置显示任务栏图标");
                            }
                        }
                    } else {
                        println!("显示并聚焦窗口...");
                        if let Err(e) = window.show() {
                            eprintln!("显示窗口失败: {}", e);
                        } else {
                            println!("窗口已显示");
                            if let Err(e) = window.set_focus() {
                                eprintln!("聚焦窗口失败: {}", e);
                            } else {
                                println!("窗口已聚焦");
                                // 窗口显示时，隐藏任务栏图标
                                if let Err(e) = window.set_skip_taskbar(true) {
                                    eprintln!("设置隐藏任务栏图标失败: {}", e);
                                } else {
                                    println!("已设置隐藏任务栏图标");
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("检查窗口可见状态失败: {}", e);
                }
            }
        } else {
            eprintln!("无法找到任何窗口");
            // 列出所有窗口以便调试
            let windows = app.webview_windows();
            println!("当前应用中的窗口:");
            for (label, window) in &windows {
                println!("  - 标签: {}, 窗口: {:?}", label, window);
            }
        }
    }
}

/// 获取默认设置
fn get_default_settings() -> AppSettings {
    AppSettings {
        prevent_auto_hide: false,
        window_width: Some(800),
        window_height: Some(600),
        settings_window_width: Some(800),
        settings_window_height: Some(600),
        new_project_window_width: Some(600),
        new_project_window_height: Some(500),
        window_layout: Some("horizontal".to_string()),
        layout_locked: Some(false),
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
        middle_mouse_toggle: Some(false),
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
