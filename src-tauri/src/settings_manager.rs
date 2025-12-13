//! 设置管理模块
//! 
//! 此模块包含所有与应用设置相关的功能，包括：
//! - 外观设置（主题、图标大小、侧栏宽度等）
//! - 启动设置（开机自启动、启动最小化等）
//! - 快捷键设置（全局快捷键、切换快捷键等）
//! - 搜索设置（模糊搜索、路径搜索等）
//! - 数据管理设置（自动备份、备份间隔等）

use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use crate::data::{load_app_settings, save_app_settings};
use crate::models::AppSettings;
use crate::system_integration::set_auto_start_windows;

/// 更新阻止自动隐藏设置
#[tauri::command]
pub fn update_prevent_auto_hide(prevent_auto_hide: bool) -> Result<String, String> {
    let mut settings = match load_app_settings() {
        Ok(settings) => settings,
        Err(_) => get_default_settings(),
    };

    settings.prevent_auto_hide = prevent_auto_hide;
    save_app_settings(settings)?;
    Ok("阻止自动隐藏设置已更新".to_string())
}

/// 获取默认设置
pub fn get_default_settings() -> AppSettings {
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

/// 重置设置到默认值
#[tauri::command]
pub fn reset_settings_to_default() -> Result<String, String> {
    let default_settings = get_default_settings();
    save_app_settings(default_settings)?;
    Ok("设置已重置为默认值".to_string())
}

/// 更新主题设置
#[tauri::command]
pub fn update_theme(theme: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.theme = Some(theme);
    save_app_settings(settings)?;
    Ok("主题设置已更新".to_string())
}

/// 更新图标大小设置
#[tauri::command]
pub fn update_icon_size(icon_size: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.icon_size = Some(icon_size);
    save_app_settings(settings)?;
    Ok("图标大小设置已更新".to_string())
}

/// 更新侧栏宽度设置
#[tauri::command]
pub fn update_sidebar_width(sidebar_width: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.sidebar_width = Some(sidebar_width);
    save_app_settings(settings)?;
    Ok("侧栏宽度设置已更新".to_string())
}

/// 更新动画设置
#[tauri::command]
pub fn update_animations(enable_animations: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.enable_animations = Some(enable_animations);
    save_app_settings(settings)?;
    Ok("动画设置已更新".to_string())
}

/// 更新动画速度设置
#[tauri::command]
pub fn update_animation_speed(animation_speed: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.animation_speed = Some(animation_speed);
    save_app_settings(settings)?;
    Ok("动画速度设置已更新".to_string())
}

/// 更新开机自启动设置
#[tauri::command]
pub fn update_start_with_system(start_with_system: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.start_with_system = Some(start_with_system);
    save_app_settings(settings)?;

    // 在 Windows 上实际设置开机自启动
    #[cfg(target_os = "windows")]
    {
        if start_with_system {
            set_auto_start_windows(true)?;
        } else {
            set_auto_start_windows(false)?;
        }
    }

    Ok("开机自启动设置已更新".to_string())
}

/// 更新启动最小化设置
#[tauri::command]
pub fn update_start_minimized(start_minimized: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.start_minimized = Some(start_minimized);
    save_app_settings(settings)?;
    Ok("启动最小化设置已更新".to_string())
}

/// 更新运行应用后自动隐藏设置
#[tauri::command]
pub fn update_auto_hide_after_launch(auto_hide_after_launch: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.auto_hide_after_launch = Some(auto_hide_after_launch);
    save_app_settings(settings)?;
    Ok("运行应用后自动隐藏设置已更新".to_string())
}

/// 更新快捷键设置
#[tauri::command]
pub async fn update_toggle_hotkey(
    app: tauri::AppHandle,
    toggle_hotkey: String,
) -> Result<String, String> {
    let mut settings = load_app_settings()?;

    // 先取消注册旧的快捷键
    if let Some(old_hotkey) = &settings.toggle_hotkey {
        if !old_hotkey.is_empty() {
            if let Ok(shortcut) = old_hotkey.parse::<Shortcut>() {
                let _ = app.global_shortcut().unregister(shortcut);
            }
        }
    }

    settings.toggle_hotkey = Some(toggle_hotkey.clone());
    save_app_settings(settings.clone())?;

    // 注册新的快捷键
    if settings.global_hotkey.unwrap_or(true) && !toggle_hotkey.is_empty() {
        if let Ok(shortcut) = toggle_hotkey.parse::<Shortcut>() {
            if let Err(e) = app.global_shortcut().register(shortcut) {
                return Err(format!("注册全局快捷键失败: {}", e));
            }
        } else {
            return Err("快捷键格式无效".to_string());
        }
    }

    Ok("快捷键设置已更新".to_string())
}

/// 更新全局快捷键设置
#[tauri::command]
pub async fn update_global_hotkey(
    app: tauri::AppHandle,
    global_hotkey: bool,
) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.global_hotkey = Some(global_hotkey);
    save_app_settings(settings.clone())?;

    if let Some(hotkey) = &settings.toggle_hotkey {
        if !hotkey.is_empty() {
            if global_hotkey {
                // 启用全局快捷键
                if let Ok(shortcut) = hotkey.parse::<Shortcut>() {
                    if let Err(e) = app.global_shortcut().register(shortcut) {
                        return Err(format!("注册全局快捷键失败: {}", e));
                    }
                } else {
                    return Err("快捷键格式无效".to_string());
                }
            } else {
                // 禁用全局快捷键
                if let Ok(shortcut) = hotkey.parse::<Shortcut>() {
                    let _ = app.global_shortcut().unregister(shortcut);
                }
            }
        }
    }

    Ok("全局快捷键设置已更新".to_string())
}

/// 更新模糊搜索设置
#[tauri::command]
pub fn update_fuzzy_search(fuzzy_search: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.fuzzy_search = Some(fuzzy_search);
    save_app_settings(settings)?;
    Ok("模糊搜索设置已更新".to_string())
}

/// 更新路径搜索设置
#[tauri::command]
pub fn update_search_in_path(search_in_path: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.search_in_path = Some(search_in_path);
    save_app_settings(settings)?;
    Ok("路径搜索设置已更新".to_string())
}

/// 更新最大搜索结果设置
#[tauri::command]
pub fn update_max_search_results(max_search_results: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.max_search_results = Some(max_search_results);
    save_app_settings(settings)?;
    Ok("最大搜索结果设置已更新".to_string())
}

/// 更新自动备份设置
#[tauri::command]
pub fn update_auto_backup(auto_backup: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.auto_backup = Some(auto_backup);
    save_app_settings(settings)?;
    Ok("自动备份设置已更新".to_string())
}

/// 更新备份间隔设置
#[tauri::command]
pub fn update_backup_interval(backup_interval: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.backup_interval = Some(backup_interval);
    save_app_settings(settings)?;
    Ok("备份间隔设置已更新".to_string())
}