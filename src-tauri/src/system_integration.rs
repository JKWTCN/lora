//! 系统集成模块
//! 
//! 此模块包含与操作系统集成的功能，包括：
//! - Windows 开机自启动设置
//! - 系统托盘管理
//! - 全局快捷键处理
//! - 系统事件处理

use std::process::Command;
use std::os::windows::process::CommandExt;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::data::{load_app_settings, save_app_settings};
use crate::models::AppSettings;

/// Windows 开机自启动实现
#[cfg(target_os = "windows")]
pub fn set_auto_start_windows(enable: bool) -> Result<(), String> {
    // 获取当前可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;
    let exe_path_str = exe_path.to_str().ok_or("可执行文件路径包含无效字符")?;

    // 注册表键路径
    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    let app_name = "Lora";

    if enable {
        // 添加到注册表
        let script = format!(
            r#"
            try {{
                New-ItemProperty -Path "HKCU:\{}" -Name "{}" -Value '"{}"' -PropertyType String -Force
                Write-Output "SUCCESS"
            }} catch {{
                Write-Error $_.Exception.Message
                exit 1
            }}
            "#,
            reg_key,
            app_name,
            exe_path_str.replace("'", "''")
        );

        let output = Command::new("powershell")
            .args([
                "-WindowStyle",
                "Hidden",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &script,
            ])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| format!("PowerShell执行失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("设置开机自启动失败: {}", stderr));
        }
    } else {
        // 从注册表中删除
        let script = format!(
            r#"
            try {{
                Remove-ItemProperty -Path "HKCU:\{}" -Name "{}" -ErrorAction SilentlyContinue
                Write-Output "SUCCESS"
            }} catch {{
                Write-Error $_.Exception.Message
                exit 1
            }}
            "#,
            reg_key, app_name
        );

        let output = Command::new("powershell")
            .args([
                "-WindowStyle",
                "Hidden",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &script,
            ])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| format!("PowerShell执行失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("取消开机自启动失败: {}", stderr));
        }
    }

    Ok(())
}

/// 更新托盘菜单项
#[tauri::command]
pub async fn update_tray_menu(
    app: AppHandle,
    prevent_auto_hide: bool,
) -> Result<String, String> {
    // 重新创建菜单项
    let prevent_auto_hide_text = if prevent_auto_hide {
        "✓ 阻止自动隐藏"
    } else {
        "○ 阻止自动隐藏"
    };

    let prevent_auto_hide_item =
        MenuItemBuilder::with_id("prevent_auto_hide", prevent_auto_hide_text)
            .build(&app)
            .map_err(|e| format!("创建菜单项失败: {}", e))?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置")
        .build(&app)
        .map_err(|e| format!("创建菜单项失败: {}", e))?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出")
        .build(&app)
        .map_err(|e| format!("创建菜单项失败: {}", e))?;

    let menu = MenuBuilder::new(&app)
        .items(&[&prevent_auto_hide_item, &settings_item, &quit_item])
        .build()
        .map_err(|e| format!("创建菜单失败: {}", e))?;

    // 更新托盘菜单
    if let Some(tray) = app.tray_by_id("main_tray") {
        tray.set_menu(Some(menu))
            .map_err(|e| format!("更新托盘菜单失败: {}", e))?;
    }

    Ok("托盘菜单已更新".to_string())
}

/// 初始化系统托盘
pub fn initialize_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 先加载设置以获取当前状态
    let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());

    // 创建托盘菜单
    let prevent_auto_hide_text = if settings.prevent_auto_hide {
        "✓ 阻止自动隐藏"
    } else {
        "○ 阻止自动隐藏"
    };

    let prevent_auto_hide =
        MenuItemBuilder::with_id("prevent_auto_hide", prevent_auto_hide_text).build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&prevent_auto_hide, &settings_item, &quit])
        .build()?;

    // 创建托盘图标
    let _tray = TrayIconBuilder::with_id("main_tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Lora Launcher")
        .on_menu_event(move |app, event| {
            handle_tray_menu_event(app, event);
        })
        .on_tray_icon_event(|tray, event| {
            handle_tray_icon_event(tray, event);
        })
        .build(app)?;

    Ok(())
}

/// 处理托盘菜单事件
fn handle_tray_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "prevent_auto_hide" => {
            // 切换阻止自动隐藏设置
            let current_settings =
                load_app_settings().unwrap_or_else(|_| get_default_settings());

            let new_value = !current_settings.prevent_auto_hide;

            // 更新设置
            let mut updated_settings = current_settings;
            updated_settings.prevent_auto_hide = new_value;

            if let Err(e) = save_app_settings(updated_settings) {
                eprintln!("保存设置失败: {}", e);
                return;
            }

            // 更新托盘菜单
            let prevent_auto_hide_text = if new_value {
                "✓ 阻止自动隐藏"
            } else {
                "○ 阻止自动隐藏"
            };

            if let Ok(prevent_auto_hide_item) = MenuItemBuilder::with_id(
                "prevent_auto_hide",
                prevent_auto_hide_text,
            )
            .build(app)
            {
                if let Ok(settings_item) =
                    MenuItemBuilder::with_id("settings", "设置").build(app)
                {
                    if let Ok(quit_item) =
                        MenuItemBuilder::with_id("quit", "退出").build(app)
                    {
                        if let Ok(menu) = MenuBuilder::new(app)
                            .items(&[
                                &prevent_auto_hide_item,
                                &settings_item,
                                &quit_item,
                            ])
                            .build()
                        {
                            if let Some(tray) = app.tray_by_id("main_tray") {
                                let _ = tray.set_menu(Some(menu));
                            }
                        }
                    }
                }
            }

            // 通知前端更新状态
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("prevent-auto-hide-changed", new_value);
            }
        }
        "settings" => {
            println!("托盘菜单：设置按钮被点击");
            // 显示设置窗口
            match app.get_webview_window("settings") {
                Some(settings_window) => {
                    println!("找到设置窗口，显示并聚焦");
                    if let Err(e) = settings_window.show() {
                        eprintln!("显示设置窗口失败: {}", e);
                    } else if let Err(e) = settings_window.set_focus() {
                        eprintln!("聚焦设置窗口失败: {}", e);
                    }
                }
                None => {
                    println!("设置窗口不存在，尝试创建新的设置窗口");
                    // 如果设置窗口不存在，尝试创建一个
                    // 这里我们不能直接调用 open_settings_window，因为需要 AppState
                    // 所以改为显示主窗口作为后备方案
                    if let Some(main_window) = app.get_webview_window("main") {
                        println!("显示主窗口作为后备方案");
                        if let Err(e) = main_window.show() {
                            eprintln!("显示主窗口失败: {}", e);
                        } else if let Err(e) = main_window.set_focus() {
                            eprintln!("聚焦主窗口失败: {}", e);
                        } else {
                            // 窗口显示时，隐藏任务栏图标
                            if let Err(e) = main_window.set_skip_taskbar(true) {
                                eprintln!("设置隐藏任务栏图标失败: {}", e);
                            }
                        }
                    } else {
                        eprintln!("无法找到主窗口");
                    }
                }
            }
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

/// 处理托盘图标事件
fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: tauri::tray::TrayIconEvent) {
    match event {
        tauri::tray::TrayIconEvent::Click { button, .. } => {
            if button == tauri::tray::MouseButton::Left {
                println!("托盘图标左键点击");
                // 左键点击只显示窗口，不隐藏（与设置菜单行为一致）
                if let Some(main_window) =
                    tray.app_handle().get_webview_window("main")
                {
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
            }
        }
        _ => {}
    }
}

/// 初始化全局快捷键
pub fn initialize_global_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 先加载设置以获取当前状态
    let settings = load_app_settings().unwrap_or_else(|_| get_default_settings());

    // 注册全局快捷键
    if settings.global_hotkey.unwrap_or(true) {
        if let Some(hotkey) = &settings.toggle_hotkey {
            if !hotkey.is_empty() {
                if let Ok(shortcut) = hotkey.parse::<Shortcut>() {
                    if let Err(e) = app.global_shortcut().register(shortcut.clone()) {
                        eprintln!("注册全局快捷键失败: {}", e);
                    } else {
                        println!("已注册全局快捷键: {}", hotkey);
                    }
                } else {
                    eprintln!("快捷键格式无效: {}", hotkey);
                }
            }
        }
    }

    Ok(())
}

/// 创建全局快捷键处理器
pub fn create_global_shortcut_handler() -> impl Fn(&AppHandle, &Shortcut, tauri_plugin_global_shortcut::ShortcutEvent) {
    |app, shortcut, event| {
        // 只处理按键按下事件，忽略松开事件
        if event.state() != ShortcutState::Pressed {
            return;
        }

        println!("快捷键按下触发: {:?}, 事件: {:?}", shortcut, event);

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
        theme: Some("auto".to_string()),
        icon_size: Some(64),
        sidebar_width: Some(180),
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
        // 界面状态记录默认值
        active_tab: Some("about".to_string()),
        last_selected_category: None,
        window_position_x: None,
        window_position_y: None,
        last_search_query: None,
        grid_view_enabled: Some(false),
        sort_order: Some("name".to_string()),
        show_hidden_files: Some(false),
    }
}