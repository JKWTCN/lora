use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// 版本信息结构
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionInfo {
    pub version: String,
    pub update_date: String,
    pub last_sync: String,
}

// 应用数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppData {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub icon: String,
    pub path: String,
    pub target_path: Option<String>, // 用于快捷方式的实际目标路径
    pub is_shortcut: bool,
    pub launch_args: Option<String>, // 启动参数
    pub target_type: Option<String>, // 目标类型: file, folder, url
}

// 分类数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub is_default: bool,
}

// 应用设置结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub prevent_auto_hide: bool,
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
    // 外观设置
    pub theme: Option<String>,
    pub icon_size: Option<u32>,
    pub sidebar_width: Option<u32>,
    pub enable_animations: Option<bool>,
    pub animation_speed: Option<String>,
    // 启动设置
    pub start_with_system: Option<bool>,
    pub start_minimized: Option<bool>,
    pub auto_hide_after_launch: Option<bool>,
    // 快捷键设置
    pub toggle_hotkey: Option<String>,
    pub global_hotkey: Option<bool>,
    // 搜索设置
    pub fuzzy_search: Option<bool>,
    pub search_in_path: Option<bool>,
    pub max_search_results: Option<u32>,
    // 数据管理
    pub auto_backup: Option<bool>,
    pub backup_interval: Option<String>,
    // 界面状态记录
    pub active_tab: Option<String>,
    pub last_selected_category: Option<String>,
    pub window_position_x: Option<i32>,
    pub window_position_y: Option<i32>,
    pub last_search_query: Option<String>,
    pub grid_view_enabled: Option<bool>,
    pub sort_order: Option<String>, // "name", "date_added", "date_modified", "frequency"
    pub show_hidden_files: Option<bool>,
}

// 应用数据存储结构
#[derive(Serialize, Deserialize, Debug)]
pub struct AppStorage {
    pub apps: Vec<AppData>,
    pub categories: Vec<CategoryData>,
    pub selected_category: Option<String>, // 记住当前选中的分组
}

// 应用状态结构
#[derive(Debug)]
pub struct AppState {
    pub settings_window_open: Arc<Mutex<bool>>,
    pub new_project_window_open: Arc<Mutex<bool>>,
    pub edit_project_window_open: Arc<Mutex<bool>>,
    #[allow(dead_code)]
    pub backup_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}
