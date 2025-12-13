use crate::models::*;
use serde_json::Value;
use std::fs;

// 获取应用数据目录
pub fn get_app_data_dir() -> Result<std::path::PathBuf, String> {
    let mut data_dir = dirs::data_local_dir().ok_or("无法获取 AppData\\Local 目录")?;
    data_dir.push("lora_launcher");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    Ok(data_dir)
}

#[tauri::command]
pub fn save_app_data(
    apps: Vec<AppData>,
    categories: Vec<CategoryData>,
    selected_category: Option<String>,
) -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("apps.json");

    let storage = AppStorage {
        apps,
        categories,
        selected_category,
    };
    let json_data = serde_json::to_string_pretty(&storage).map_err(|e| format!("序列化数据失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("保存文件失败: {}", e))?;

    Ok("数据保存成功".to_string())
}

#[tauri::command]
pub fn load_app_data() -> Result<AppStorage, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("apps.json");

    if !file_path.exists() {
        return Ok(AppStorage {
            apps: vec![],
            categories: vec![],
            selected_category: Some("all".to_string()),
        });
    }

    let json_data = match std::fs::read(&file_path) {
        Ok(data) => data,
        Err(e) => return Err(format!("读取文件失败: {}", e)),
    };

    let storage: AppStorage = match serde_json::from_slice(&json_data) {
        Ok(storage) => storage,
        Err(e) => return Err(format!("解析数据失败: {}", e)),
    };

    Ok(storage)
}

#[tauri::command]
pub fn save_app_settings(settings: AppSettings) -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("settings.json");

    let json_data = serde_json::to_string_pretty(&settings).map_err(|e| format!("序列化设置失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("保存设置文件失败: {}", e))?;

    Ok("设置保存成功".to_string())
}

#[tauri::command]
pub fn load_app_settings() -> Result<AppSettings, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("settings.json");

    if !file_path.exists() {
        return Ok(get_default_settings());
    }

    let json_data = match std::fs::read(&file_path) {
        Ok(data) => data,
        Err(e) => return Err(format!("读取设置文件失败: {}", e)),
    };

    let settings: AppSettings = match serde_json::from_slice(&json_data) {
        Ok(settings) => settings,
        Err(e) => return Err(format!("解析设置失败: {}", e)),
    };

    Ok(settings)
}

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

#[tauri::command]
pub fn save_window_size(width: u32, height: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.window_width = Some(width);
    settings.window_height = Some(height);
    save_app_settings(settings)?;
    Ok("窗口大小保存成功".to_string())
}

#[tauri::command]
pub fn save_ui_state(
    active_tab: Option<String>,
    last_selected_category: Option<String>,
    window_position_x: Option<i32>,
    window_position_y: Option<i32>,
    last_search_query: Option<String>,
    grid_view_enabled: Option<bool>,
    sort_order: Option<String>,
    show_hidden_files: Option<bool>,
) -> Result<String, String> {
    let mut settings = load_app_settings()?;

    if let Some(tab) = active_tab {
        settings.active_tab = Some(tab);
    }
    if let Some(category) = last_selected_category {
        settings.last_selected_category = Some(category);
    }
    if let Some(x) = window_position_x {
        settings.window_position_x = Some(x);
    }
    if let Some(y) = window_position_y {
        settings.window_position_y = Some(y);
    }
    if let Some(query) = last_search_query {
        settings.last_search_query = Some(query);
    }
    if let Some(grid_view) = grid_view_enabled {
        settings.grid_view_enabled = Some(grid_view);
    }
    if let Some(sort) = sort_order {
        settings.sort_order = Some(sort);
    }
    if let Some(hidden) = show_hidden_files {
        settings.show_hidden_files = Some(hidden);
    }

    save_app_settings(settings)?;
    Ok("界面状态已保存".to_string())
}

#[tauri::command]
pub fn update_settings_batch(settings_update: Value) -> Result<String, String> {
    let mut settings = load_app_settings()?;

    if let Some(prevent_auto_hide) = settings_update.get("preventAutoHide").and_then(|v| v.as_bool()) {
        settings.prevent_auto_hide = prevent_auto_hide;
    }
    if let Some(window_width) = settings_update.get("windowWidth").and_then(|v| v.as_u64()) {
        settings.window_width = Some(window_width as u32);
    }
    if let Some(window_height) = settings_update.get("windowHeight").and_then(|v| v.as_u64()) {
        settings.window_height = Some(window_height as u32);
    }
    if let Some(theme) = settings_update.get("theme").and_then(|v| v.as_str()) {
        settings.theme = Some(theme.to_string());
    }
    if let Some(icon_size) = settings_update.get("iconSize").and_then(|v| v.as_u64()) {
        settings.icon_size = Some(icon_size as u32);
    }
    if let Some(sidebar_width) = settings_update.get("sidebarWidth").and_then(|v| v.as_u64()) {
        settings.sidebar_width = Some(sidebar_width as u32);
    }
    if let Some(enable_animations) = settings_update.get("enableAnimations").and_then(|v| v.as_bool()) {
        settings.enable_animations = Some(enable_animations);
    }
    if let Some(animation_speed) = settings_update.get("animationSpeed").and_then(|v| v.as_str()) {
        settings.animation_speed = Some(animation_speed.to_string());
    }
    if let Some(start_with_system) = settings_update.get("startWithSystem").and_then(|v| v.as_bool()) {
        settings.start_with_system = Some(start_with_system);
    }
    if let Some(start_minimized) = settings_update.get("startMinimized").and_then(|v| v.as_bool()) {
        settings.start_minimized = Some(start_minimized);
    }
    if let Some(auto_hide_after_launch) = settings_update.get("autoHideAfterLaunch").and_then(|v| v.as_bool()) {
        settings.auto_hide_after_launch = Some(auto_hide_after_launch);
    }
    if let Some(toggle_hotkey) = settings_update.get("toggleHotkey").and_then(|v| v.as_str()) {
        settings.toggle_hotkey = Some(toggle_hotkey.to_string());
    }
    if let Some(global_hotkey) = settings_update.get("globalHotkey").and_then(|v| v.as_bool()) {
        settings.global_hotkey = Some(global_hotkey);
    }
    if let Some(fuzzy_search) = settings_update.get("fuzzySearch").and_then(|v| v.as_bool()) {
        settings.fuzzy_search = Some(fuzzy_search);
    }
    if let Some(search_in_path) = settings_update.get("searchInPath").and_then(|v| v.as_bool()) {
        settings.search_in_path = Some(search_in_path);
    }
    if let Some(max_search_results) = settings_update.get("maxSearchResults").and_then(|v| v.as_u64()) {
        settings.max_search_results = Some(max_search_results as u32);
    }
    if let Some(auto_backup) = settings_update.get("autoBackup").and_then(|v| v.as_bool()) {
        settings.auto_backup = Some(auto_backup);
    }
    if let Some(backup_interval) = settings_update.get("backupInterval").and_then(|v| v.as_str()) {
        settings.backup_interval = Some(backup_interval.to_string());
    }

    save_app_settings(settings)?;
    Ok("设置已批量更新".to_string())
}

#[tauri::command]
pub fn delete_app(app_id: i64) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.apps.retain(|app| app.id != app_id);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("应用删除成功".to_string())
}

#[tauri::command]
pub fn update_app_category(app_id: i64, new_category: String) -> Result<String, String> {
    let mut storage = load_app_data()?;

    if let Some(app) = storage.apps.iter_mut().find(|app| app.id == app_id) {
        app.category = new_category;
        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
        Ok("应用分类更新成功".to_string())
    } else {
        Err("应用不存在".to_string())
    }
}

#[tauri::command]
pub fn save_selected_category(category_id: String) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.selected_category = Some(category_id);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("选中分组保存成功".to_string())
}

#[tauri::command]
pub async fn add_new_app(app: AppData) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.apps.push(app);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("应用添加成功".to_string())
}

#[tauri::command]
pub async fn update_app(app: AppData) -> Result<String, String> {
    let mut storage = load_app_data()?;

    if let Some(existing_app) = storage.apps.iter_mut().find(|a| a.id == app.id) {
        *existing_app = app;
        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
        Ok("应用更新成功".to_string())
    } else {
        Err("应用不存在".to_string())
    }
}

#[tauri::command]
pub async fn get_app_by_id(app_id: i64) -> Result<AppData, String> {
    let storage = load_app_data()?;

    if let Some(app) = storage.apps.iter().find(|a| a.id == app_id) {
        Ok(app.clone())
    } else {
        Err("应用不存在".to_string())
    }
}

// export/import/clear functions proxied to be used by UI
#[tauri::command]
pub fn export_app_data_to_file(file_path: String) -> Result<String, String> {
    let storage = load_app_data()?;
    let settings = load_app_settings()?;

    let export_data = serde_json::json!({
        "storage": storage,
        "settings": settings,
        "export_time": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "version": env!("CARGO_PKG_VERSION")
    });

    let json_data = serde_json::to_string_pretty(&export_data).map_err(|e| format!("序列化导出数据失败: {}", e))?;
    fs::write(&file_path, json_data).map_err(|e| format!("写入导出文件失败: {}", e))?;

    Ok("数据导出成功".to_string())
}

#[tauri::command]
pub fn import_app_data_from_file(file_path: String) -> Result<String, String> {
    let json_data = fs::read_to_string(&file_path).map_err(|e| format!("读取导入文件失败: {}", e))?;

    let import_data: Value = serde_json::from_str(&json_data).map_err(|e| format!("解析导入数据失败: {}", e))?;

    if let Some(storage_data) = import_data.get("storage") {
        let storage: AppStorage = serde_json::from_value(storage_data.clone()).map_err(|e| format!("解析存储数据失败: {}", e))?;
        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    }

    if let Some(settings_data) = import_data.get("settings") {
        let settings: AppSettings = serde_json::from_value(settings_data.clone()).map_err(|e| format!("解析设置数据失败: {}", e))?;
        save_app_settings(settings)?;
    }

    Ok("数据导入成功".to_string())
}

#[tauri::command]
pub fn clear_all_data() -> Result<String, String> {
    let empty_storage = AppStorage { apps: vec![], categories: vec![], selected_category: Some("all".to_string()) };
    save_app_data(empty_storage.apps, empty_storage.categories, empty_storage.selected_category)?;
    let default_settings = get_default_settings();
    save_app_settings(default_settings)?;
    Ok("所有数据已清空".to_string())
}
