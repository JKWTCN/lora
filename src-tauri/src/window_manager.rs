//! 窗口管理模块
//! 
//! 此模块包含所有与窗口管理相关的功能，包括：
//! - 主窗口的显示/隐藏切换
//! - 设置窗口的打开/关闭
//! - 新建项目窗口的打开/关闭
//! - 编辑项目窗口的打开/关闭
//! - 窗口状态检查
//! - 主窗口大小获取

use tauri::{Manager, State};

use crate::models::AppState;

/// 显示/隐藏主窗口
#[tauri::command]
pub async fn toggle_window_visibility(app: tauri::AppHandle) -> Result<String, String> {
    println!("切换窗口可见性...");

    if let Some(window) = app.get_webview_window("main") {
        println!("找到主窗口，检查当前可见状态");
        match window.is_visible() {
            Ok(visible) => {
                println!("窗口当前可见状态: {}", visible);
                if visible {
                    println!("尝试隐藏窗口...");
                    if let Err(e) = window.hide() {
                        let error_msg = format!("隐藏窗口失败: {}", e);
                        eprintln!("{}", error_msg);
                        return Err(error_msg);
                    } else {
                        println!("窗口已隐藏");
                        // 窗口隐藏时，显示任务栏图标
                        if let Err(e) = window.set_skip_taskbar(false) {
                            eprintln!("设置显示任务栏图标失败: {}", e);
                        } else {
                            println!("已设置显示任务栏图标");
                        }
                        Ok("窗口已隐藏".to_string())
                    }
                } else {
                    println!("尝试显示并聚焦窗口...");
                    if let Err(e) = window.show() {
                        let error_msg = format!("显示窗口失败: {}", e);
                        eprintln!("{}", error_msg);
                        return Err(error_msg);
                    } else {
                        println!("窗口已显示");
                        if let Err(e) = window.set_focus() {
                            let error_msg = format!("聚焦窗口失败: {}", e);
                            eprintln!("{}", error_msg);
                            return Err(error_msg);
                        } else {
                            println!("窗口已聚焦");
                            // 窗口显示时，隐藏任务栏图标
                            if let Err(e) = window.set_skip_taskbar(true) {
                                eprintln!("设置隐藏任务栏图标失败: {}", e);
                            } else {
                                println!("已设置隐藏任务栏图标");
                            }
                            Ok("窗口已显示".to_string())
                        }
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("检查窗口可见状态失败: {}", e);
                eprintln!("{}", error_msg);
                Err(error_msg)
            }
        }
    } else {
        let error_msg = "找不到主窗口".to_string();
        eprintln!("{}", error_msg);
        // 列出所有窗口以便调试
        let windows = app.webview_windows();
        println!("当前应用中的窗口:");
        for (label, window) in &windows {
            println!("  - 标签: {}, 窗口: {:?}", label, window);
        }
        Err(error_msg)
    }
}

/// 打开设置窗口
#[tauri::command]
pub async fn open_settings_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    println!("open_settings_window: 尝试打开设置窗口...");

    // 首先检查所有窗口，确保没有重复的设置窗口
    let windows = app.webview_windows();
    let mut settings_window_count = 0;

    for (label, window) in &windows {
        if label == "settings" {
            settings_window_count += 1;
            println!("open_settings_window: 发现现有设置窗口: {:?}", window);
        }
    }

    if settings_window_count > 1 {
        eprintln!(
            "open_settings_window: 警告 - 发现多个设置窗口实例！数量: {}",
            settings_window_count
        );
        // 尝试关闭多余的窗口，只保留一个
        let mut count = 0;
        for (label, window) in &windows {
            if label == "settings" {
                count += 1;
                if count > 1 {
                    println!("open_settings_window: 关闭多余的设置窗口");
                    let _ = window.close();
                }
            }
        }
    }

    // 检查设置窗口是否已经存在
    if let Some(settings_window) = app.get_webview_window("settings") {
        println!("open_settings_window: 设置窗口已存在，显示并聚焦");
        // 如果窗口已存在，则显示并聚焦
        settings_window
            .show()
            .map_err(|e| format!("显示设置窗口失败: {}", e))?;
        settings_window
            .set_focus()
            .map_err(|e| format!("聚焦设置窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut settings_open) = state.settings_window_open.lock() {
            *settings_open = true;
        }

        return Ok("设置窗口已打开".to_string());
    }

    // 检查状态是否与实际窗口状态不一致
    if let Ok(settings_open) = state.settings_window_open.lock() {
        if *settings_open {
            println!(
                "open_settings_window: 警告 - 状态显示设置窗口已打开，但实际找不到窗口，重置状态"
            );
        }
    }

    // 克隆状态以便在闭包中使用
    let state_clone = state.settings_window_open.clone();

    println!("open_settings_window: 创建新的设置窗口...");
    // 创建新的设置窗口
    let settings_window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("设置")
    .inner_size(800.0, 600.0)
    .min_inner_size(600.0, 450.0)
    .center()
    .resizable(true)
    .decorations(true)
    .always_on_top(false)
    .build()
    .map_err(|e| format!("创建设置窗口失败: {}", e))?;

    println!("open_settings_window: 设置窗口创建成功，设置事件监听器");
    // 监听窗口关闭事件
    settings_window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            println!("open_settings_window: 设置窗口关闭事件触发");
            // 窗口关闭时更新状态
            if let Ok(mut settings_open) = state_clone.lock() {
                *settings_open = false;
            }
        }
    });

    // 更新状态
    if let Ok(mut settings_open) = state.settings_window_open.lock() {
        *settings_open = true;
    }

    Ok("设置窗口已创建".to_string())
}

/// 检查设置窗口是否打开
#[tauri::command]
pub fn is_settings_window_open(state: State<'_, AppState>) -> bool {
    match state.settings_window_open.lock() {
        Ok(settings_open) => *settings_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

/// 关闭设置窗口
#[tauri::command]
pub async fn close_settings_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    if let Some(settings_window) = app.get_webview_window("settings") {
        settings_window
            .close()
            .map_err(|e| format!("关闭设置窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut settings_open) = state.settings_window_open.lock() {
            *settings_open = false;
        }

        Ok("设置窗口已关闭".to_string())
    } else {
        Err("设置窗口不存在".to_string())
    }
}

/// 打开新建项目窗口
#[tauri::command]
pub async fn open_new_project_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    println!("open_new_project_window: 尝试打开新建项目窗口...");

    // 首先检查所有窗口，确保没有重复的新建项目窗口
    let windows = app.webview_windows();
    let mut new_project_window_count = 0;

    for (label, window) in &windows {
        if label == "new_project" {
            new_project_window_count += 1;
            println!(
                "open_new_project_window: 发现现有新建项目窗口: {:?}",
                window
            );
        }
    }

    if new_project_window_count > 1 {
        eprintln!(
            "open_new_project_window: 警告 - 发现多个新建项目窗口实例！数量: {}",
            new_project_window_count
        );
        // 尝试关闭多余的窗口，只保留一个
        let mut count = 0;
        for (label, window) in &windows {
            if label == "new_project" {
                count += 1;
                if count > 1 {
                    println!("open_new_project_window: 关闭多余的新建项目窗口");
                    let _ = window.close();
                }
            }
        }
    }

    // 检查新建项目窗口是否已经存在
    if let Some(new_project_window) = app.get_webview_window("new_project") {
        println!("open_new_project_window: 新建项目窗口已存在，显示并聚焦");
        // 如果窗口已存在，则显示并聚焦
        new_project_window
            .show()
            .map_err(|e| format!("显示新建项目窗口失败: {}", e))?;
        new_project_window
            .set_focus()
            .map_err(|e| format!("聚焦新建项目窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut new_project_open) = state.new_project_window_open.lock() {
            *new_project_open = true;
        }

        return Ok("新建项目窗口已打开".to_string());
    }

    // 检查状态是否与实际窗口状态不一致
    if let Ok(new_project_open) = state.new_project_window_open.lock() {
        if *new_project_open {
            println!("open_new_project_window: 警告 - 状态显示新建项目窗口已打开，但实际找不到窗口，重置状态");
        }
    }

    // 克隆状态以便在闭包中使用
    let state_clone = state.new_project_window_open.clone();

    println!("open_new_project_window: 创建新的新建项目窗口...");
    // 创建新的新建项目窗口
    let new_project_window = tauri::WebviewWindowBuilder::new(
        &app,
        "new_project",
        tauri::WebviewUrl::App("new-project.html".into()),
    )
    .title("新建项目")
    .inner_size(600.0, 500.0)
    .min_inner_size(500.0, 400.0)
    .center()
    .resizable(true)
    .decorations(true)
    .always_on_top(false)
    .build()
    .map_err(|e| format!("创建新建项目窗口失败: {}", e))?;

    println!("open_new_project_window: 新建项目窗口创建成功，设置事件监听器");
    // 监听窗口关闭事件
    new_project_window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            println!("open_new_project_window: 新建项目窗口关闭事件触发");
            // 窗口关闭时更新状态
            if let Ok(mut new_project_open) = state_clone.lock() {
                *new_project_open = false;
            }
        }
    });

    // 更新状态
    if let Ok(mut new_project_open) = state.new_project_window_open.lock() {
        *new_project_open = true;
    }

    Ok("新建项目窗口已创建".to_string())
}

/// 检查新建项目窗口是否打开
#[tauri::command]
pub fn is_new_project_window_open(state: State<'_, AppState>) -> bool {
    match state.new_project_window_open.lock() {
        Ok(new_project_open) => *new_project_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

/// 关闭新建项目窗口
#[tauri::command]
pub async fn close_new_project_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    if let Some(new_project_window) = app.get_webview_window("new_project") {
        new_project_window
            .close()
            .map_err(|e| format!("关闭新建项目窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut new_project_open) = state.new_project_window_open.lock() {
            *new_project_open = false;
        }

        Ok("新建项目窗口已关闭".to_string())
    } else {
        Err("新建项目窗口不存在".to_string())
    }
}

/// 打开编辑项目窗口
#[tauri::command]
pub async fn open_edit_project_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    app_id: i64,
) -> Result<String, String> {
    println!(
        "open_edit_project_window: 尝试打开编辑项目窗口，应用ID: {}",
        app_id
    );

    // 首先检查所有窗口，确保没有重复的编辑项目窗口
    let windows = app.webview_windows();
    let mut edit_project_window_count = 0;

    for (label, window) in &windows {
        if label == "edit_project" {
            edit_project_window_count += 1;
            println!(
                "open_edit_project_window: 发现现有编辑项目窗口: {:?}",
                window
            );
        }
    }

    if edit_project_window_count > 1 {
        eprintln!(
            "open_edit_project_window: 警告 - 发现多个编辑项目窗口实例！数量: {}",
            edit_project_window_count
        );
        // 尝试关闭多余的窗口，只保留一个
        let mut count = 0;
        for (label, window) in &windows {
            if label == "edit_project" {
                count += 1;
                if count > 1 {
                    println!("open_edit_project_window: 关闭多余的编辑项目窗口");
                    let _ = window.close();
                }
            }
        }
    }

    // 检查编辑项目窗口是否已经存在
    if let Some(edit_project_window) = app.get_webview_window("edit_project") {
        println!("open_edit_project_window: 编辑项目窗口已存在，显示并聚焦");
        // 如果窗口已存在，则显示并聚焦
        edit_project_window
            .show()
            .map_err(|e| format!("显示编辑项目窗口失败: {}", e))?;
        edit_project_window
            .set_focus()
            .map_err(|e| format!("聚焦编辑项目窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut edit_project_open) = state.edit_project_window_open.lock() {
            *edit_project_open = true;
        }

        return Ok("编辑项目窗口已打开".to_string());
    }

    // 检查状态是否与实际窗口状态不一致
    if let Ok(edit_project_open) = state.edit_project_window_open.lock() {
        if *edit_project_open {
            println!("open_edit_project_window: 警告 - 状态显示编辑项目窗口已打开，但实际找不到窗口，重置状态");
        }
    }

    // 克隆状态以便在闭包中使用
    let state_clone = state.edit_project_window_open.clone();

    println!("open_edit_project_window: 创建新的编辑项目窗口...");
    // 创建新的编辑项目窗口，并传递应用ID作为URL参数
    let url = format!("edit-project.html?id={}", app_id);
    let edit_project_window =
        tauri::WebviewWindowBuilder::new(&app, "edit_project", tauri::WebviewUrl::App(url.into()))
            .title("编辑项目")
            .inner_size(600.0, 500.0)
            .min_inner_size(500.0, 400.0)
            .center()
            .resizable(true)
            .decorations(true)
            .always_on_top(false)
            .build()
            .map_err(|e| format!("创建编辑项目窗口失败: {}", e))?;

    println!("open_edit_project_window: 编辑项目窗口创建成功，设置事件监听器");
    // 监听窗口关闭事件
    edit_project_window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            println!("open_edit_project_window: 编辑项目窗口关闭事件触发");
            // 窗口关闭时更新状态
            if let Ok(mut edit_project_open) = state_clone.lock() {
                *edit_project_open = false;
            }
        }
    });

    // 更新状态
    if let Ok(mut edit_project_open) = state.edit_project_window_open.lock() {
        *edit_project_open = true;
    }

    Ok("编辑项目窗口已创建".to_string())
}

/// 检查编辑项目窗口是否打开
#[tauri::command]
pub fn is_edit_project_window_open(state: State<'_, AppState>) -> bool {
    match state.edit_project_window_open.lock() {
        Ok(edit_project_open) => *edit_project_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

/// 关闭编辑项目窗口
#[tauri::command]
pub async fn close_edit_project_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    if let Some(edit_project_window) = app.get_webview_window("edit_project") {
        edit_project_window
            .close()
            .map_err(|e| format!("关闭编辑项目窗口失败: {}", e))?;

        // 更新状态
        if let Ok(mut edit_project_open) = state.edit_project_window_open.lock() {
            *edit_project_open = false;
        }

        Ok("编辑项目窗口已关闭".to_string())
    } else {
        Err("编辑项目窗口不存在".to_string())
    }
}

/// 获取主窗口大小
#[tauri::command]
pub async fn get_main_window_size(app: tauri::AppHandle) -> Result<(u32, u32), String> {
    println!("get_main_window_size: 尝试获取主窗口大小");

    // 获取主窗口
    match app.get_webview_window("main") {
        Some(window) => {
            println!("get_main_window_size: 成功获取主窗口");

            // 获取窗口大小
            match window.inner_size() {
                Ok(size) => {
                    println!("get_main_window_size: 成功获取窗口大小: {:?}", size);

                    // 获取缩放因子
                    let scale_factor = window.scale_factor().unwrap_or(1.0);
                    println!("get_main_window_size: 缩放因子: {}", scale_factor);

                    // 转换为逻辑像素
                    let logical_size = size.to_logical::<u32>(scale_factor);
                    println!("get_main_window_size: 逻辑大小: {:?}", logical_size);

                    Ok((logical_size.width, logical_size.height))
                }

                Err(e) => {
                    let error_msg = format!("获取窗口大小失败: {}", e);
                    eprintln!("get_main_window_size: {}", error_msg);
                    Err(error_msg)
                }
            }
        }
        None => {
            let error_msg = "主窗口不存在".to_string();
            eprintln!("get_main_window_size: {}", error_msg);

            // 列出所有可用窗口以便调试
            let windows = app.webview_windows();
            eprintln!("get_main_window_size: 当前应用中的窗口:");
            for (label, window) in &windows {
                eprintln!("  - 标签: {}, 窗口: {:?}", label, window);
            }

            Err(error_msg)
        }
    }
}