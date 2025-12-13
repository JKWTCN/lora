use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

mod app_launcher;
mod backup;
mod data;
mod data_manager;
mod helpers;
mod models;
mod settings_manager;
mod system;
mod system_integration;
mod window_manager;
mod windows;
use crate::backup::BackupManager;
use crate::models::{AppState, VersionInfo};
use crate::system_integration::{create_global_shortcut_handler, initialize_global_shortcuts, initialize_tray};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// 获取应用版本号
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// 获取应用更新日期
#[tauri::command]
fn get_app_update_date() -> String {
    // 尝试从 version-info.json 文件读取更新日期
    let version_info_path = Path::new("version-info.json");

    if let Ok(json_data) = fs::read_to_string(version_info_path) {
        if let Ok(version_info) = serde_json::from_str::<VersionInfo>(&json_data) {
            return version_info.update_date;
        }
    }

    // 如果读取失败，返回当前UTC日期作为后备
    use chrono::Utc;
    let now = Utc::now();
    now.format("%Y-%m-%d").to_string()
}

// 退出应用
#[tauri::command]
async fn quit_app(app: tauri::AppHandle) -> Result<String, String> {
    app.exit(0);
    Ok("应用已退出".to_string())
}

// 通知主窗口刷新数据
#[tauri::command]
async fn notify_main_window_refresh(app: tauri::AppHandle) -> Result<String, String> {
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.emit("data-updated", {});
    }
    Ok("通知已发送".to_string())
}

/// 主库文件
///
/// 此文件是 Tauri 应用程序的主入口点，负责：
/// - 初始化应用程序状态
/// - 配置插件和全局快捷键
/// - 设置系统托盘和窗口事件处理
/// - 注册所有 Tauri 命令处理器
///
/// 大部分功能已拆分到专门的模块中：
/// - window_manager: 窗口管理功能
/// - settings_manager: 应用设置管理
/// - app_launcher: 应用启动功能
/// - system_integration: 系统集成功能
/// - data_manager: 数据导入导出功能

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化应用状态
    let app_state = AppState {
        settings_window_open: Arc::new(Mutex::new(false)),
        new_project_window_open: Arc::new(Mutex::new(false)),
        edit_project_window_open: Arc::new(Mutex::new(false)),
        backup_handle: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(create_global_shortcut_handler())
                .build(),
        )
        .setup(|app| {
            // 初始化系统托盘
            initialize_tray(&app.handle())?;
            
            // 初始化全局快捷键
            initialize_global_shortcuts(&app.handle())?;
            
            // 设置主窗口隐藏任务栏图标
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.set_skip_taskbar(true);

                // 尝试读取保存的窗口大小并应用（修复重启后大小不恢复的问题）
                if let Ok(settings) = crate::data::load_app_settings() {
                    if let (Some(w), Some(h)) = (settings.window_width, settings.window_height) {
                        // 使用逻辑像素设置窗口大小
                        use tauri::Size;
                        use tauri::LogicalSize;
                        let _ = main_window.set_size(Size::Logical(LogicalSize::new(w as f64, h as f64)));
                    }
                    // 如果保存了窗口位置，也尝试恢复
                    if let (Some(x), Some(y)) = (settings.window_position_x, settings.window_position_y) {
                        use tauri::Position;
                        use tauri::LogicalPosition;
                        let _ = main_window.set_position(Position::Logical(LogicalPosition::new(x as f64, y as f64)));
                    }
                }
            }

            // 启动定时备份任务
            let app_handle_for_backup = app.handle().clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Err(e) = BackupManager::start_backup_task(app_handle_for_backup).await {
                        eprintln!("启动备份任务失败: {}", e);
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            data::load_app_data,
            data::save_app_settings,
            data::load_app_settings,
            data::save_window_size,
            window_manager::get_main_window_size,
            settings_manager::update_prevent_auto_hide,
            system_integration::update_tray_menu,
            window_manager::toggle_window_visibility,
            window_manager::open_settings_window,
            window_manager::close_settings_window,
            window_manager::is_settings_window_open,
            window_manager::open_new_project_window,
            window_manager::close_new_project_window,
            window_manager::is_new_project_window_open,
            window_manager::open_edit_project_window,
            window_manager::close_edit_project_window,
            window_manager::is_edit_project_window_open,
            data::get_app_by_id,
            data::update_app,
            data::add_new_app,
            quit_app,
            app_launcher::get_file_info,
            app_launcher::launch_app,
            app_launcher::launch_app_with_auto_hide,
            system::open_url,
            system::open_folder,
            system::open_file_dialog,
            system::open_folder_dialog,
            system::detect_target_type,
            data::save_app_data,
            data::delete_app,
            data::update_app_category,
            data::save_selected_category,
            app_launcher::get_app_icon,
            windows::run_as_admin,
            windows::open_file_location,
            get_app_version,
            get_app_update_date,
            settings_manager::reset_settings_to_default,
            data::export_app_data_to_file,
            data::import_app_data_from_file,
            data::clear_all_data,
            settings_manager::update_theme,
            settings_manager::update_icon_size,
            settings_manager::update_sidebar_width,
            settings_manager::update_animations,
            settings_manager::update_animation_speed,
            settings_manager::update_start_with_system,
            settings_manager::update_start_minimized,
            settings_manager::update_auto_hide_after_launch,
            settings_manager::update_toggle_hotkey,
            settings_manager::update_global_hotkey,
            settings_manager::update_fuzzy_search,
            settings_manager::update_search_in_path,
            settings_manager::update_max_search_results,
            settings_manager::update_auto_backup,
            settings_manager::update_backup_interval,
            backup::manual_backup,
            backup::get_backup_status,
            data::save_ui_state,
            data::update_settings_batch,
            windows::check_auto_start_status,
            data_manager::export_data,
            data_manager::import_data,
            data_manager::reset_data,
            greet,
            notify_main_window_refresh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

