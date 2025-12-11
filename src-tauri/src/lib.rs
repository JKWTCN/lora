use chrono::{Datelike, Local, NaiveDate, Timelike};
#[cfg(target_os = "windows")]
use lnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Emitter, Manager, State,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tokio::time::sleep;

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
    pub backup_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

// 备份任务管理器
pub struct BackupManager {
    pub is_running: Arc<Mutex<bool>>,
}

impl BackupManager {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    // 执行备份
    pub async fn perform_backup() -> Result<String, String> {
        println!("开始执行定时备份...");

        // 获取应用数据目录
        let data_dir = get_app_data_dir()?;

        // 创建备份子目录
        let backup_dir = data_dir.join("backups");
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败: {}", e))?;
        }

        // 生成备份文件名（带时间戳）
        let now = Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let backup_file_path = backup_dir.join(format!("lora_auto_backup_{}.json", timestamp));

        // 加载应用数据和设置
        let storage = load_app_data()?;
        let settings = load_app_settings()?;

        // 创建备份数据
        let backup_data = serde_json::json!({
            "storage": storage,
            "settings": settings,
            "backup_time": now.timestamp(),
            "backup_type": "auto",
            "version": env!("CARGO_PKG_VERSION")
        });

        // 写入备份文件
        let json_data = serde_json::to_string_pretty(&backup_data)
            .map_err(|e| format!("序列化备份数据失败: {}", e))?;

        fs::write(&backup_file_path, json_data).map_err(|e| format!("写入备份文件失败: {}", e))?;

        // 清理旧备份（保留最近10个）
        Self::cleanup_old_backups(&backup_dir)?;

        let message = format!("自动备份成功完成: {}", backup_file_path.display());
        println!("{}", message);
        Ok(message)
    }

    // 清理旧备份文件，保留最近的10个
    fn cleanup_old_backups(backup_dir: &Path) -> Result<(), String> {
        let mut backup_files = Vec::new();

        if let Ok(entries) = fs::read_dir(backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("lora_auto_backup_")
                            && file_name.ends_with(".json")
                        {
                            if let Ok(metadata) = fs::metadata(&path) {
                                if let Ok(modified) = metadata.modified() {
                                    backup_files.push((path, modified));
                                }
                            }
                        }
                    }
                }
            }
        }

        // 按修改时间排序（最新的在前）
        backup_files.sort_by(|a, b| b.1.cmp(&a.1));

        // 删除超过10个的旧备份
        if backup_files.len() > 10 {
            for (path, _) in backup_files.iter().skip(10) {
                if let Err(e) = fs::remove_file(path) {
                    println!("删除旧备份文件失败: {}, 错误: {}", path.display(), e);
                } else {
                    println!("已删除旧备份文件: {}", path.display());
                }
            }
        }

        Ok(())
    }

    // 计算下次备份时间
    fn calculate_next_backup_time(interval: &str) -> Result<chrono::DateTime<Local>, String> {
        let now = Local::now();

        match interval {
            "daily" => {
                // 每天备份：明天同一时间
                let next_day = now.date_naive().succ_opt().ok_or("计算明天日期失败")?;
                let next_time = next_day
                    .and_hms_opt(now.hour(), now.minute(), now.second())
                    .ok_or("构建明天时间失败")?;
                Ok(next_time
                    .and_local_timezone(Local)
                    .single()
                    .ok_or("构建本地时间失败")?)
            }
            "weekly" => {
                // 每周备份：下周同一时间
                let days_until_next_week = 7 - now.weekday().num_days_from_monday() % 7;
                let next_week_date =
                    now.date_naive() + chrono::Duration::days(days_until_next_week as i64);
                let next_time = next_week_date
                    .and_hms_opt(now.hour(), now.minute(), now.second())
                    .ok_or("构建下周时间失败")?;
                Ok(next_time
                    .and_local_timezone(Local)
                    .single()
                    .ok_or("构建本地时间失败")?)
            }
            "monthly" => {
                // 每月备份：下个月同一时间
                let next_month = if now.month() == 12 {
                    now.year() + 1
                } else {
                    now.year()
                };
                let next_month_num = if now.month() == 12 {
                    1
                } else {
                    now.month() + 1
                };

                // 尝试下个月同一天，如果不存在则使用下个月最后一天
                let next_day = if let Some(date) =
                    NaiveDate::from_ymd_opt(next_month, next_month_num, now.day())
                {
                    date
                } else {
                    // 获取下个月最后一天
                    NaiveDate::from_ymd_opt(next_month, next_month_num + 1, 1)
                        .and_then(|d| d.pred_opt())
                        .ok_or("计算下个月最后一天失败")?
                };

                let next_time = next_day
                    .and_hms_opt(now.hour(), now.minute(), now.second())
                    .ok_or("构建下个月时间失败")?;
                Ok(next_time
                    .and_local_timezone(Local)
                    .single()
                    .ok_or("构建本地时间失败")?)
            }
            _ => Err("不支持的备份间隔".to_string()),
        }
    }

    // 启动定时备份任务
    pub async fn start_backup_task(app_handle: tauri::AppHandle) -> Result<(), String> {
        loop {
            // 检查是否已启用自动备份
            let settings = load_app_settings()?;
            if !settings.auto_backup.unwrap_or(false) {
                println!("自动备份未启用，等待1小时后重新检查");
                sleep(Duration::from_secs(3600)).await;
                continue;
            }

            let backup_interval = settings.backup_interval.unwrap_or("weekly".to_string());
            println!("启动定时备份任务，间隔: {}", backup_interval);

            // 计算下次备份时间
            let next_backup_time = Self::calculate_next_backup_time(&backup_interval)?;
            let now = Local::now();

            if next_backup_time > now {
                let duration_until_next = next_backup_time.signed_duration_since(now);
                let seconds_until_next = duration_until_next.num_seconds().max(0) as u64;

                println!(
                    "下次备份时间: {}, 等待 {} 秒",
                    next_backup_time, seconds_until_next
                );

                // 等待到下次备份时间
                sleep(Duration::from_secs(seconds_until_next)).await;
            }

            // 执行备份
            if let Err(e) = Self::perform_backup().await {
                println!("自动备份失败: {}", e);
            }
        }
    }
}

// 手动执行备份
#[tauri::command]
async fn manual_backup() -> Result<String, String> {
    match BackupManager::perform_backup().await {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("手动备份失败: {}", e)),
    }
}

// 获取备份状态信息
#[tauri::command]
fn get_backup_status() -> Result<serde_json::Value, String> {
    let data_dir = get_app_data_dir()?;
    let backup_dir = data_dir.join("backups");

    let mut backup_files = Vec::new();

    if backup_dir.exists() {
        if let Ok(entries) = fs::read_dir(&backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("lora_auto_backup_")
                            && file_name.ends_with(".json")
                        {
                            if let Ok(metadata) = fs::metadata(&path) {
                                if let Ok(modified) = metadata.modified() {
                                    if let Ok(modified_datetime) =
                                        modified.duration_since(std::time::UNIX_EPOCH)
                                    {
                                        backup_files.push(serde_json::json!({
                                            "name": file_name,
                                            "path": path.to_string_lossy(),
                                            "size": metadata.len(),
                                            "modified": modified_datetime.as_secs()
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 按修改时间排序（最新的在前）
    backup_files.sort_by(|a, b| {
        let a_time = a.get("modified").and_then(|v| v.as_u64()).unwrap_or(0);
        let b_time = b.get("modified").and_then(|v| v.as_u64()).unwrap_or(0);
        b_time.cmp(&a_time)
    });

    Ok(serde_json::json!({
        "backup_dir": backup_dir.to_string_lossy(),
        "backup_files": backup_files,
        "total_count": backup_files.len()
    }))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_file_info(file_path: String) -> Result<serde_json::Value, String> {
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
    let name = actual_path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未知应用")
        .to_string();

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

    // 提取文件图标（对于快捷方式，尝试使用目标文件的图标）
    let icon_base64 = if is_shortcut && target_path.is_some() {
        extract_file_icon(&actual_path)
    } else {
        extract_file_icon(&file_path)
    };

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

// 解析快捷方式目标路径
fn resolve_shortcut_target(shortcut_path: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        // Windows快捷方式解析
        resolve_windows_shortcut(shortcut_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 其他系统的处理（如果需要）
        None
    }
}

#[cfg(target_os = "windows")]
fn resolve_windows_shortcut(shortcut_path: &str) -> Option<String> {
    let path = Path::new(shortcut_path);

    // 检查文件扩展名
    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
        match extension.to_lowercase().as_str() {
            "lnk" => {
                // 解析.lnk文件
                match ShellLink::open(shortcut_path) {
                    Ok(link) => {
                        // 使用正确的API获取目标路径
                        if let Some(link_info) = link.link_info() {
                            if let Some(target) = link_info.local_base_path() {
                                println!("快捷方式目标路径: {:?}", target);
                                return Some(target.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        println!("解析.lnk文件失败: {:?}", e);
                    }
                }
            }
            "url" => {
                // 解析.url文件（Internet快捷方式）
                if let Ok(content) = fs::read_to_string(shortcut_path) {
                    for line in content.lines() {
                        if line.starts_with("URL=") {
                            let url = line.trim_start_matches("URL=");
                            println!("URL快捷方式目标: {}", url);
                            return Some(url.to_string());
                        }
                    }
                }
            }
            _ => {
                // 其他文件类型直接返回原路径
                return Some(shortcut_path.to_string());
            }
        }
    }

    // 如果解析失败，返回原路径
    Some(shortcut_path.to_string())
}

// 提取文件图标并转换为 Base64 字符串或图标标识符
fn extract_file_icon(file_path: &str) -> Option<String> {
    // 简化实现：直接根据文件扩展名返回图标标识符
    // 未来可以扩展为提取真实图标
    let path = Path::new(file_path);
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 根据文件扩展名返回不同的图标标识符
    match extension.as_str() {
        "exe" => Some("exe".to_string()),
        "msi" => Some("installer".to_string()),
        "lnk" => Some("shortcut".to_string()),
        "txt" => Some("text".to_string()),
        "pdf" => Some("pdf".to_string()),
        "doc" | "docx" => Some("word".to_string()),
        "xls" | "xlsx" => Some("excel".to_string()),
        "ppt" | "pptx" => Some("powerpoint".to_string()),
        "zip" | "rar" | "7z" => Some("archive".to_string()),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" => Some("image".to_string()),
        "mp3" | "wav" | "flac" => Some("audio".to_string()),
        "mp4" | "avi" | "mkv" => Some("video".to_string()),
        "html" | "htm" => Some("web".to_string()),
        "js" | "ts" => Some("javascript".to_string()),
        "py" => Some("python".to_string()),
        "java" => Some("java".to_string()),
        "cpp" | "c" | "h" => Some("code".to_string()),
        "json" => Some("json".to_string()),
        "xml" => Some("xml".to_string()),
        "css" => Some("css".to_string()),
        _ => None, // 对于未知类型，返回 None，前端会显示默认图标
    }
}

#[tauri::command]
fn launch_app(app_path: String, launch_args: Option<String>) -> Result<String, String> {
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
                return open_url(url, launch_args);
            } else {
                return Err("无法解析URL快捷方式".to_string());
            }
        } else {
            app_path.clone()
        };

        // 使用解析后的路径启动应用
        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                use std::os::windows::process::CommandExt;

                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();

                // 使用更安全的进程创建方式，直接启动应用而不是通过cmd
                let result = Command::new(&actual_path)
                    .creation_flags(0x00000008 | 0x08000000) // DETACHED_PROCESS | CREATE_NO_WINDOW
                    .args(&split_args)
                    .current_dir(Path::new(&actual_path).parent().unwrap_or(Path::new(".")))
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();

                match result {
                    Ok(_) => Ok("应用启动成功".to_string()),
                    Err(e) => Err(format!("启动应用失败: {}", e)),
                }
            } else {
                // 使用更安全的进程创建方式
                let result = Command::new(&actual_path)
                    .creation_flags(0x00000008 | 0x08000000) // DETACHED_PROCESS | CREATE_NO_WINDOW
                    .current_dir(Path::new(&actual_path).parent().unwrap_or(Path::new(".")))
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();

                match result {
                    Ok(_) => Ok("应用启动成功".to_string()),
                    Err(e) => Err(format!("启动应用失败: {}", e)),
                }
            }
        } else {
            // 使用更安全的进程创建方式
            let result = Command::new(&actual_path)
                .creation_flags(0x00000008 | 0x08000000) // DETACHED_PROCESS | CREATE_NO_WINDOW
                .current_dir(Path::new(&actual_path).parent().unwrap_or(Path::new(".")))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();

            match result {
                Ok(_) => Ok("应用启动成功".to_string()),
                Err(e) => Err(format!("启动应用失败: {}", e)),
            }
        }
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

// 启动应用并检查是否需要自动隐藏窗口
#[tauri::command]
async fn launch_app_with_auto_hide(
    app: tauri::AppHandle,
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
            let _ = window.hide();
        }
    }

    Ok(launch_result)
}

// 打开网址
#[tauri::command]
fn open_url(url: String, launch_args: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                let mut cmd_args = vec!["/C".to_string(), "start".to_string(), "".to_string(), url];
                cmd_args.extend(split_args);

                let result = Command::new("cmd")
                    .args(&cmd_args)
                    .creation_flags(0x08000000)
                    .spawn();

                match result {
                    Ok(_) => Ok("网址打开成功".to_string()),
                    Err(e) => Err(format!("打开网址失败: {}", e)),
                }
            } else {
                let result = Command::new("cmd")
                    .creation_flags(0x08000000)
                    .args(["/C", "start", "", &url])
                    .spawn();

                match result {
                    Ok(_) => Ok("网址打开成功".to_string()),
                    Err(e) => Err(format!("打开网址失败: {}", e)),
                }
            }
        } else {
            let result = Command::new("cmd")
                .creation_flags(0x08000000)
                .args(["/C", "start", "", &url])
                .spawn();

            match result {
                Ok(_) => Ok("网址打开成功".to_string()),
                Err(e) => Err(format!("打开网址失败: {}", e)),
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut cmd = Command::new("open").creation_flags(0x08000000);
        cmd.arg(&url);

        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                cmd.args(&split_args);
            }
        }

        let result = cmd.spawn();

        match result {
            Ok(_) => Ok("网址打开成功".to_string()),
            Err(e) => Err(format!("打开网址失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("xdg-open").creation_flags(0x08000000);
        cmd.arg(&url);

        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                cmd.args(&split_args);
            }
        }

        let result = cmd.spawn();

        match result {
            Ok(_) => Ok("网址打开成功".to_string()),
            Err(e) => Err(format!("打开网址失败: {}", e)),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("当前平台不支持打开网址".to_string())
    }
}

// 打开文件夹
#[tauri::command]
fn open_folder(folder_path: String, launch_args: Option<String>) -> Result<String, String> {
    let path = Path::new(&folder_path);

    if !path.exists() {
        return Err("文件夹不存在".to_string());
    }

    if !path.is_dir() {
        return Err("路径不是文件夹".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("explorer");
        cmd.creation_flags(0x08000000);
        cmd.arg(&folder_path);
        cmd.creation_flags(0x08000000);
        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                cmd.args(&split_args);
            }
        }

        let result = cmd.spawn();

        match result {
            Ok(_) => Ok("文件夹打开成功".to_string()),
            Err(e) => Err(format!("打开文件夹失败: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut cmd = Command::new("open").creation_flags(0x08000000);
        cmd.arg(&folder_path);

        if let Some(args_str) = launch_args {
            if !args_str.trim().is_empty() {
                let split_args: Vec<String> =
                    args_str.split_whitespace().map(|s| s.to_string()).collect();
                cmd.args(&split_args);
            }
        }

        let result = cmd.spawn();

        match result {
            Ok(_) => Ok("文件夹打开成功".to_string()),
            Err(e) => Err(format!("打开文件夹失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm"];

        for manager in &file_managers {
            if Command::new("which")
                .creation_flags(0x08000000)
                .arg(manager)
                .output()
                .is_ok()
            {
                let mut cmd = Command::new(manager).creation_flags(0x08000000);
                cmd.arg(&folder_path);

                if let Some(args_str) = launch_args {
                    if !args_str.trim().is_empty() {
                        let split_args: Vec<String> =
                            args_str.split_whitespace().map(|s| s.to_string()).collect();
                        cmd.args(&split_args);
                    }
                }

                let result = cmd.spawn();

                if result.is_ok() {
                    return Ok("文件夹打开成功".to_string());
                }
            }
        }

        Err("未找到可用的文件管理器".to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("当前平台不支持打开文件夹".to_string())
    }
}

// 打开文件选择对话框
#[tauri::command]
fn open_file_dialog(title: String, filters: Vec<(String, Vec<String>)>) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // 构建PowerShell脚本来显示文件选择对话框
        let mut filter_string = String::new();
        for (name, extensions) in filters {
            if !filter_string.is_empty() {
                filter_string.push('|');
            }
            let ext_string = extensions.join(";*.");
            filter_string.push_str(&format!("{}|*.{}", name, ext_string));
        }

        let script = format!(
            r#"
            Add-Type -AssemblyName System.Windows.Forms
            $openFileDialog = New-Object System.Windows.Forms.OpenFileDialog
            $openFileDialog.Title = '{}'
            $openFileDialog.Filter = '{}'
            $openFileDialog.RestoreDirectory = $true
            $result = $openFileDialog.ShowDialog()
            if ($result -eq [System.Windows.Forms.DialogResult]::OK) {{
                Write-Output $openFileDialog.FileName
            }} else {{
                Write-Output ""
            }}
            "#,
            title.replace("'", "''"),
            filter_string.replace("'", "''")
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

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !result.is_empty() {
            Ok(result)
        } else {
            Err("用户取消了选择".to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件选择对话框".to_string())
    }
}

// 打开文件夹选择对话框
#[tauri::command]
fn open_folder_dialog(title: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let script = format!(
            r#"
            Add-Type -AssemblyName System.Windows.Forms
            $folderBrowser = New-Object System.Windows.Forms.FolderBrowserDialog
            $folderBrowser.Description = '{}'
            $folderBrowser.ShowNewFolderButton = $true
            $result = $folderBrowser.ShowDialog()
            if ($result -eq [System.Windows.Forms.DialogResult]::OK) {{
                Write-Output $folderBrowser.SelectedPath
            }} else {{
                Write-Output ""
            }}
            "#,
            title.replace("'", "''")
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

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !result.is_empty() {
            Ok(result)
        } else {
            Err("用户取消了选择".to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件夹选择对话框".to_string())
    }
}

// 自动判断目标类型
#[tauri::command]
fn detect_target_type(target_path: String) -> Result<String, String> {
    // 如果是URL
    if target_path.starts_with("http://")
        || target_path.starts_with("https://")
        || target_path.starts_with("ftp://")
        || target_path.starts_with("file://")
    {
        return Ok("url".to_string());
    }

    // 如果是文件系统路径
    let path = Path::new(&target_path);
    if path.exists() {
        if path.is_dir() {
            Ok("folder".to_string())
        } else {
            Ok("file".to_string())
        }
    } else {
        // 如果路径不存在，根据扩展名判断
        if path.extension().is_some() {
            Ok("file".to_string())
        } else {
            // 没有扩展名，假设是文件夹
            Ok("folder".to_string())
        }
    }
}

// 获取应用数据目录
fn get_app_data_dir() -> Result<std::path::PathBuf, String> {
    // 使用 AppData\Local 路径作为数据目录
    let mut data_dir = dirs::data_local_dir().ok_or("无法获取 AppData\\Local 目录")?;
    data_dir.push("lora_launcher");

    // 确保目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    Ok(data_dir)
}

// 保存应用数据
#[tauri::command]
fn save_app_data(
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
    let json_data =
        serde_json::to_string_pretty(&storage).map_err(|e| format!("序列化数据失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("保存文件失败: {}", e))?;

    Ok("数据保存成功".to_string())
}

// 加载应用数据 - 优化版本
#[tauri::command]
fn load_app_data() -> Result<AppStorage, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("apps.json");

    if !file_path.exists() {
        // 如果文件不存在，返回默认数据
        return Ok(AppStorage {
            apps: vec![],
            categories: vec![],
            selected_category: Some("all".to_string()), // 默认选中"全部应用"
        });
    }

    // 使用更高效的文件读取
    let json_data = match std::fs::read(&file_path) {
        Ok(data) => data,
        Err(e) => return Err(format!("读取文件失败: {}", e)),
    };

    // 直接从字节解析 JSON，避免字符串转换
    let storage: AppStorage = match serde_json::from_slice(&json_data) {
        Ok(storage) => storage,
        Err(e) => return Err(format!("解析数据失败: {}", e)),
    };

    Ok(storage)
}

// 保存应用设置
#[tauri::command]
fn save_app_settings(settings: AppSettings) -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("settings.json");

    let json_data =
        serde_json::to_string_pretty(&settings).map_err(|e| format!("序列化设置失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("保存设置文件失败: {}", e))?;

    Ok("设置保存成功".to_string())
}

// 更新阻止自动隐藏设置
#[tauri::command]
fn update_prevent_auto_hide(prevent_auto_hide: bool) -> Result<String, String> {
    let mut settings = match load_app_settings() {
        Ok(settings) => settings,
        Err(_) => get_default_settings(),
    };

    settings.prevent_auto_hide = prevent_auto_hide;
    save_app_settings(settings)?;
    Ok("阻止自动隐藏设置已更新".to_string())
}

// 更新托盘菜单项
#[tauri::command]
async fn update_tray_menu(
    app: tauri::AppHandle,
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

// 加载应用设置 - 优化版本
#[tauri::command]
fn load_app_settings() -> Result<AppSettings, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("settings.json");

    if !file_path.exists() {
        // 如果文件不存在，返回默认设置
        return Ok(get_default_settings());
    }

    // 使用更高效的文件读取
    let json_data = match std::fs::read(&file_path) {
        Ok(data) => data,
        Err(e) => return Err(format!("读取设置文件失败: {}", e)),
    };

    // 直接从字节解析 JSON
    let settings: AppSettings = match serde_json::from_slice(&json_data) {
        Ok(settings) => settings,
        Err(e) => return Err(format!("解析设置失败: {}", e)),
    };

    Ok(settings)
}

// 保存窗口大小
#[tauri::command]
fn save_window_size(width: u32, height: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.window_width = Some(width);
    settings.window_height = Some(height);
    save_app_settings(settings)?;
    Ok("窗口大小保存成功".to_string())
}

// 获取默认设置
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

// 重置设置到默认值
#[tauri::command]
fn reset_settings_to_default() -> Result<String, String> {
    let default_settings = get_default_settings();
    save_app_settings(default_settings)?;
    Ok("设置已重置为默认值".to_string())
}

// 更新主题设置
#[tauri::command]
fn update_theme(theme: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.theme = Some(theme);
    save_app_settings(settings)?;
    Ok("主题设置已更新".to_string())
}

// 更新图标大小设置
#[tauri::command]
fn update_icon_size(icon_size: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.icon_size = Some(icon_size);
    save_app_settings(settings)?;
    Ok("图标大小设置已更新".to_string())
}

// 更新侧栏宽度设置
#[tauri::command]
fn update_sidebar_width(sidebar_width: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.sidebar_width = Some(sidebar_width);
    save_app_settings(settings)?;
    Ok("侧栏宽度设置已更新".to_string())
}

// 更新动画设置
#[tauri::command]
fn update_animations(enable_animations: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.enable_animations = Some(enable_animations);
    save_app_settings(settings)?;
    Ok("动画设置已更新".to_string())
}

// 更新动画速度设置
#[tauri::command]
fn update_animation_speed(animation_speed: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.animation_speed = Some(animation_speed);
    save_app_settings(settings)?;
    Ok("动画速度设置已更新".to_string())
}

// 更新开机自启动设置
#[tauri::command]
fn update_start_with_system(start_with_system: bool) -> Result<String, String> {
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

// 更新启动最小化设置
#[tauri::command]
fn update_start_minimized(start_minimized: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.start_minimized = Some(start_minimized);
    save_app_settings(settings)?;
    Ok("启动最小化设置已更新".to_string())
}

// 更新运行应用后自动隐藏设置
#[tauri::command]
fn update_auto_hide_after_launch(auto_hide_after_launch: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.auto_hide_after_launch = Some(auto_hide_after_launch);
    save_app_settings(settings)?;
    Ok("运行应用后自动隐藏设置已更新".to_string())
}

// 更新快捷键设置
#[tauri::command]
async fn update_toggle_hotkey(
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

// 更新全局快捷键设置
#[tauri::command]
async fn update_global_hotkey(
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

// 更新模糊搜索设置
#[tauri::command]
fn update_fuzzy_search(fuzzy_search: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.fuzzy_search = Some(fuzzy_search);
    save_app_settings(settings)?;
    Ok("模糊搜索设置已更新".to_string())
}

// 更新路径搜索设置
#[tauri::command]
fn update_search_in_path(search_in_path: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.search_in_path = Some(search_in_path);
    save_app_settings(settings)?;
    Ok("路径搜索设置已更新".to_string())
}

// 更新最大搜索结果设置
#[tauri::command]
fn update_max_search_results(max_search_results: u32) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.max_search_results = Some(max_search_results);
    save_app_settings(settings)?;
    Ok("最大搜索结果设置已更新".to_string())
}

// 更新自动备份设置
#[tauri::command]
fn update_auto_backup(auto_backup: bool) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.auto_backup = Some(auto_backup);
    save_app_settings(settings)?;
    Ok("自动备份设置已更新".to_string())
}

// 更新备份间隔设置
#[tauri::command]
fn update_backup_interval(backup_interval: String) -> Result<String, String> {
    let mut settings = load_app_settings()?;
    settings.backup_interval = Some(backup_interval);
    save_app_settings(settings)?;
    Ok("备份间隔设置已更新".to_string())
}

// 保存界面状态
#[tauri::command]
fn save_ui_state(
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

// 批量更新设置
#[tauri::command]
fn update_settings_batch(settings_update: serde_json::Value) -> Result<String, String> {
    let mut settings = load_app_settings()?;

    // 从JSON中更新设置
    if let Some(prevent_auto_hide) = settings_update
        .get("preventAutoHide")
        .and_then(|v| v.as_bool())
    {
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
    if let Some(enable_animations) = settings_update
        .get("enableAnimations")
        .and_then(|v| v.as_bool())
    {
        settings.enable_animations = Some(enable_animations);
    }
    if let Some(animation_speed) = settings_update
        .get("animationSpeed")
        .and_then(|v| v.as_str())
    {
        settings.animation_speed = Some(animation_speed.to_string());
    }
    if let Some(start_with_system) = settings_update
        .get("startWithSystem")
        .and_then(|v| v.as_bool())
    {
        settings.start_with_system = Some(start_with_system);
    }
    if let Some(start_minimized) = settings_update
        .get("startMinimized")
        .and_then(|v| v.as_bool())
    {
        settings.start_minimized = Some(start_minimized);
    }
    if let Some(auto_hide_after_launch) = settings_update
        .get("autoHideAfterLaunch")
        .and_then(|v| v.as_bool())
    {
        settings.auto_hide_after_launch = Some(auto_hide_after_launch);
    }
    if let Some(toggle_hotkey) = settings_update.get("toggleHotkey").and_then(|v| v.as_str()) {
        settings.toggle_hotkey = Some(toggle_hotkey.to_string());
    }
    if let Some(global_hotkey) = settings_update
        .get("globalHotkey")
        .and_then(|v| v.as_bool())
    {
        settings.global_hotkey = Some(global_hotkey);
    }
    if let Some(fuzzy_search) = settings_update.get("fuzzySearch").and_then(|v| v.as_bool()) {
        settings.fuzzy_search = Some(fuzzy_search);
    }
    if let Some(search_in_path) = settings_update
        .get("searchInPath")
        .and_then(|v| v.as_bool())
    {
        settings.search_in_path = Some(search_in_path);
    }
    if let Some(max_search_results) = settings_update
        .get("maxSearchResults")
        .and_then(|v| v.as_u64())
    {
        settings.max_search_results = Some(max_search_results as u32);
    }
    if let Some(auto_backup) = settings_update.get("autoBackup").and_then(|v| v.as_bool()) {
        settings.auto_backup = Some(auto_backup);
    }
    if let Some(backup_interval) = settings_update
        .get("backupInterval")
        .and_then(|v| v.as_str())
    {
        settings.backup_interval = Some(backup_interval.to_string());
    }

    save_app_settings(settings)?;
    Ok("设置已批量更新".to_string())
}

// Windows 开机自启动实现
#[cfg(target_os = "windows")]
fn set_auto_start_windows(enable: bool) -> Result<(), String> {
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
// 检查是否已设置开机自启动
#[tauri::command]
fn check_auto_start_status() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
        let app_name = "Lora";

        let script = format!(
            r#"
            try {{
                $value = Get-ItemProperty -Path "HKCU:\{}" -Name "{}" -ErrorAction SilentlyContinue
                if ($value) {{
                    Write-Output "TRUE"
                }} else {{
                    Write-Output "FALSE"
                }}
            }} catch {{
                Write-Output "FALSE"
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

        let result_string = String::from_utf8_lossy(&output.stdout);
        let result = result_string.trim();
        Ok(result == "TRUE")
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(false) // 其他平台暂不支持
    }
}

// 导出数据到用户选择的文件
#[tauri::command]
fn export_data() -> Result<String, String> {
    // 打开文件保存对话框
    #[cfg(target_os = "windows")]
    {
        let script = r#"
            Add-Type -AssemblyName System.Windows.Forms
            $saveFileDialog = New-Object System.Windows.Forms.SaveFileDialog
            $saveFileDialog.Title = '导出数据'
            $saveFileDialog.Filter = 'JSON文件|*.json|所有文件|*.*'
            $saveFileDialog.DefaultExt = 'json'
            $saveFileDialog.FileName = "lora_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
            $result = $saveFileDialog.ShowDialog()
            if ($result -eq [System.Windows.Forms.DialogResult]::OK) {
                Write-Output $saveFileDialog.FileName
            } else {
                Write-Output ""
            }
        "#;

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

        let file_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if file_path.is_empty() {
            return Err("用户取消了导出操作".to_string());
        }

        export_app_data_to_file(file_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 其他平台使用默认路径
        let data_dir = get_app_data_dir()?;
        let file_path = data_dir.join(format!(
            "lora_backup_{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        export_app_data_to_file(file_path.to_string_lossy().to_string())
    }
}

// 从用户选择的文件导入数据
#[tauri::command]
fn import_data() -> Result<String, String> {
    // 打开文件选择对话框
    #[cfg(target_os = "windows")]
    {
        // 使用已经实现的open_file_dialog函数
        let filters = vec![
            ("JSON文件".to_string(), vec!["json".to_string()]),
            ("所有文件".to_string(), vec!["*".to_string()]),
        ];

        let file_path = open_file_dialog("选择要导入的数据文件".to_string(), filters)?;
        import_app_data_from_file(file_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件导入对话框".to_string())
    }
}

// 重置数据（包含确认）
#[tauri::command]
fn reset_data() -> Result<String, String> {
    clear_all_data()
}

// 导出应用数据
#[tauri::command]
fn export_app_data_to_file(file_path: String) -> Result<String, String> {
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

    let json_data = serde_json::to_string_pretty(&export_data)
        .map_err(|e| format!("序列化导出数据失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("写入导出文件失败: {}", e))?;

    Ok("数据导出成功".to_string())
}

// 从文件导入应用数据
#[tauri::command]
fn import_app_data_from_file(file_path: String) -> Result<String, String> {
    let json_data =
        fs::read_to_string(&file_path).map_err(|e| format!("读取导入文件失败: {}", e))?;

    let import_data: serde_json::Value =
        serde_json::from_str(&json_data).map_err(|e| format!("解析导入数据失败: {}", e))?;

    // 导入存储数据
    if let Some(storage_data) = import_data.get("storage") {
        let storage: AppStorage = serde_json::from_value(storage_data.clone())
            .map_err(|e| format!("解析存储数据失败: {}", e))?;

        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    }

    // 导入设置数据
    if let Some(settings_data) = import_data.get("settings") {
        let settings: AppSettings = serde_json::from_value(settings_data.clone())
            .map_err(|e| format!("解析设置数据失败: {}", e))?;

        save_app_settings(settings)?;
    }

    Ok("数据导入成功".to_string())
}

// 清空所有数据
#[tauri::command]
fn clear_all_data() -> Result<String, String> {
    // 清空应用数据
    let empty_storage = AppStorage {
        apps: vec![],
        categories: vec![],
        selected_category: Some("all".to_string()),
    };
    save_app_data(
        empty_storage.apps,
        empty_storage.categories,
        empty_storage.selected_category,
    )?;

    // 重置设置到默认值
    reset_settings_to_default()?;

    Ok("所有数据已清空".to_string())
}

// 删除应用
#[tauri::command]
fn delete_app(app_id: i64) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.apps.retain(|app| app.id != app_id);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("应用删除成功".to_string())
}

// 更新应用分类
#[tauri::command]
fn update_app_category(app_id: i64, new_category: String) -> Result<String, String> {
    let mut storage = load_app_data()?;

    if let Some(app) = storage.apps.iter_mut().find(|app| app.id == app_id) {
        app.category = new_category;
        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
        Ok("应用分类更新成功".to_string())
    } else {
        Err("应用不存在".to_string())
    }
}

// 保存当前选中的分组
#[tauri::command]
fn save_selected_category(category_id: String) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.selected_category = Some(category_id);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("选中分组保存成功".to_string())
}

// 获取应用图标的专用命令

fn extract_icon_from_exe(exe_path: &str) -> Result<String, String> {
    use base64::engine::general_purpose;
    use base64::Engine;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    // 创建临时目录，使用时间戳确保唯一性
    let temp_dir = format!(
        "temp_icons_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    // 创建临时目录
    if !Path::new(&temp_dir).exists() {
        fs::create_dir(&temp_dir).map_err(|e| format!("创建临时目录失败: {}", e))?;
    }

    // 使用 7z 提取 exe 文件
    let output = Command::new("7z")
        .args(&["e", exe_path, &format!("-o{}", temp_dir)])
        .output()
        .map_err(|e| format!("执行 7z 失败: {}", e))?;

    if !output.status.success() {
        // 清理临时目录
        let _ = fs::remove_dir_all(&temp_dir);
        return Err("7z 提取失败".to_string());
    }

    // 查找 ico 文件，选择大小最大的
    let mut ico_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&temp_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "ico" {
                        if let Ok(metadata) = fs::metadata(&path) {
                            ico_files.push((path, metadata.len()));
                        }
                    }
                }
            }
        }
    }

    // 选择大小最大的 ico 文件
    let ico_path = ico_files
        .into_iter()
        .max_by_key(|&(_, size)| size)
        .map(|(path, _)| path);

    let result = if let Some(ico_path) = ico_path {
        // 使用 image crate 加载 ico 文件并转换为 PNG
        match image::open(&ico_path) {
            Ok(img) => {
                let img_buffer = img.to_rgba8();
                let mut png_bytes: Vec<u8> = Vec::new();
                {
                    use image::ImageEncoder;
                    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
                    if encoder
                        .write_image(
                            img_buffer.as_raw(),
                            img_buffer.width(),
                            img_buffer.height(),
                            image::ColorType::Rgba8,
                        )
                        .is_ok()
                    {
                        let base64_encoded = general_purpose::STANDARD.encode(&png_bytes);
                        Ok(format!("data:image/png;base64,{}", base64_encoded))
                    } else {
                        Err("转换 ico 为 PNG 失败".to_string())
                    }
                }
            }
            Err(e) => Err(format!("加载 ico 文件失败: {}", e)),
        }
    } else {
        Err("未找到 ico 文件".to_string())
    };

    // 清理临时目录
    let _ = fs::remove_dir_all(&temp_dir);

    result
}

#[tauri::command]
fn get_app_icon(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use base64::engine::general_purpose;
        use base64::Engine;
        use image::{ImageBuffer, Rgba};
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr;
        use winapi::um::shellapi::ExtractIconExW;
        use winapi::um::wingdi::{GetObjectW, BITMAP};
        use winapi::um::winuser::ICONINFO;

        // 转换路径为宽字符
        let wide_path: Vec<u16> = OsStr::new(&file_path)
            .encode_wide()
            .chain(Some(0))
            .collect();
        let icon_count =
            unsafe { ExtractIconExW(wide_path.as_ptr(), -1, ptr::null_mut(), ptr::null_mut(), 0) };
        let mut success = false;
        let mut result_str = String::new();
        if icon_count > 0 {
            let mut large_icons: Vec<winapi::shared::windef::HICON> =
                vec![ptr::null_mut(); icon_count as usize];
            let mut small_icons: Vec<winapi::shared::windef::HICON> =
                vec![ptr::null_mut(); icon_count as usize];
            let extracted = unsafe {
                ExtractIconExW(
                    wide_path.as_ptr(),
                    0,
                    large_icons.as_mut_ptr(),
                    small_icons.as_mut_ptr(),
                    icon_count,
                )
            };
            if extracted == icon_count {
                let icon = large_icons.get(0).copied().unwrap_or(ptr::null_mut());
                if !icon.is_null() {
                    let mut icon_info: ICONINFO = unsafe { std::mem::zeroed() };
                    let result = unsafe {
                        use winapi::um::winuser::GetIconInfo;
                        GetIconInfo(icon, &mut icon_info)
                    };
                    if result != 0 {
                        let mut bitmap: BITMAP = unsafe { std::mem::zeroed() };
                        if unsafe {
                            GetObjectW(
                                icon_info.hbmColor as _,
                                std::mem::size_of::<BITMAP>() as i32,
                                &mut bitmap as *mut _ as _,
                            )
                        } != 0
                        {
                            let width = bitmap.bmWidth;
                            let height = bitmap.bmHeight;
                            let stride = bitmap.bmWidthBytes as usize;
                            let buffer_size = (stride * height as usize) as u32;
                            let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
                            let bytes_copied = unsafe {
                                use winapi::um::wingdi::GetBitmapBits;
                                GetBitmapBits(
                                    icon_info.hbmColor as _,
                                    buffer_size as i32,
                                    buffer.as_mut_ptr() as _,
                                )
                            };
                            if bytes_copied == buffer_size as i32 {
                                let mut rgba_data =
                                    Vec::with_capacity((width * height * 4) as usize);
                                for y in 0..height {
                                    for x in 0..width {
                                        let offset = (y * stride as i32 + x * 4) as usize;
                                        if offset + 3 < buffer.len() {
                                            rgba_data.push(buffer[offset + 2]); // R
                                            rgba_data.push(buffer[offset + 1]); // G
                                            rgba_data.push(buffer[offset + 0]); // B
                                            rgba_data.push(buffer[offset + 3]); // A
                                        }
                                    }
                                }
                                if let Some(img) = ImageBuffer::<Rgba<u8>, _>::from_raw(
                                    width as u32,
                                    height as u32,
                                    rgba_data,
                                ) {
                                    let mut png_bytes: Vec<u8> = Vec::new();
                                    {
                                        use image::ImageEncoder;

                                        let encoder =
                                            image::codecs::png::PngEncoder::new(&mut png_bytes);
                                        if encoder
                                            .write_image(
                                                &img,
                                                width as u32,
                                                height as u32,
                                                image::ColorType::Rgba8,
                                            )
                                            .is_ok()
                                        {
                                            let base64_str =
                                                general_purpose::STANDARD.encode(&png_bytes);
                                            result_str =
                                                format!("data:image/png;base64,{}", base64_str);
                                            success = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if success {
            Ok(result_str)
        } else {
            // 尝试解压 exe 方法
            println!("尝试解压 exe 文件");
            match extract_icon_from_exe(&file_path) {
                Ok(icon_data) => Ok(icon_data),
                Err(_) => {
                    // 回退到 PowerShell 方法
                    let script = format!(
                        r#"
                        try {{
                            Add-Type -AssemblyName System.Drawing
                            $icon = [System.Drawing.Icon]::ExtractAssociatedIcon('{}')
                            if ($icon) {{
                                $bitmap = $icon.ToBitmap()
                                $stream = New-Object System.IO.MemoryStream
                                $bitmap.Save($stream, [System.Drawing.Imaging.ImageFormat]::Png)
                                $bytes = $stream.ToArray()
                                $base64 = [System.Convert]::ToBase64String($bytes)
                                $stream.Dispose()
                                $bitmap.Dispose()
                                $icon.Dispose()
                                Write-Output "data:image/png;base64,$base64"
                            }} else {{
                                Write-Output ""
                            }}
                        }} catch {{
                            Write-Output ""
                        }}
                        "#,
                        file_path.replace("'", "''")
                    );

                    let output = std::process::Command::new("powershell")
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

                    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !result.is_empty() && result.starts_with("data:image/png;base64,") {
                        Ok(result)
                    } else {
                        Err("无法提取图标".to_string())
                    }
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持图标提取".to_string())
    }
}

// 以管理员权限运行应用
#[tauri::command]
fn run_as_admin(app_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let result = Command::new("powershell")
            .args([
                "-WindowStyle",
                "Hidden",
                "-Command",
                &format!(
                    "Start-Process '{}' -Verb RunAs",
                    app_path.replace("'", "''")
                ),
            ])
            .creation_flags(0x08000000)
            .spawn();

        match result {
            Ok(_) => Ok("应用以管理员权限启动成功".to_string()),
            Err(e) => Err(format!("以管理员权限启动应用失败: {}", e)),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持管理员权限运行".to_string())
    }
}

// 打开文件所在目录
#[tauri::command]
fn open_file_location(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    let dir_path = path
        .parent()
        .ok_or("无法获取文件目录")?
        .to_str()
        .ok_or("路径包含无效字符")?;

    #[cfg(target_os = "windows")]
    {
        let result = Command::new("explorer")
            .creation_flags(0x08000000)
            .arg(dir_path)
            .spawn();

        match result {
            Ok(_) => Ok("已打开文件所在目录".to_string()),
            Err(e) => Err(format!("打开目录失败: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let result = Command::new("open")
            .creation_flags(0x08000000)
            .arg(dir_path)
            .spawn();

        match result {
            Ok(_) => Ok("已打开文件所在目录".to_string()),
            Err(e) => Err(format!("打开目录失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm"];

        for manager in &file_managers {
            if Command::new("which")
                .creation_flags(0x08000000)
                .creation_flags(0x08000000)
                .arg(manager)
                .output()
                .is_ok()
            {
                let result = Command::new(manager)
                    .creation_flags(0x08000000)
                    .creation_flags(0x08000000)
                    .arg(dir_path)
                    .spawn();

                if result.is_ok() {
                    return Ok("已打开文件所在目录".to_string());
                }
            }
        }

        Err("未找到可用的文件管理器".to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("当前平台不支持打开目录".to_string())
    }
}
#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}

// 显示/隐藏窗口
#[tauri::command]
async fn toggle_window_visibility(app: tauri::AppHandle) -> Result<String, String> {
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

// 打开设置窗口
#[tauri::command]
async fn open_settings_window(
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

// 检查设置窗口是否打开
#[tauri::command]
fn is_settings_window_open(state: State<'_, AppState>) -> bool {
    match state.settings_window_open.lock() {
        Ok(settings_open) => *settings_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

// 关闭设置窗口
#[tauri::command]
async fn close_settings_window(
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

// 打开新建项目窗口
#[tauri::command]
async fn open_new_project_window(
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

// 检查新建项目窗口是否打开
#[tauri::command]
fn is_new_project_window_open(state: State<'_, AppState>) -> bool {
    match state.new_project_window_open.lock() {
        Ok(new_project_open) => *new_project_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

// 关闭新建项目窗口
#[tauri::command]
async fn close_new_project_window(
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

// 添加新应用
#[tauri::command]
async fn add_new_app(app: AppData) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.apps.push(app);
    save_app_data(storage.apps, storage.categories, storage.selected_category)?;
    Ok("应用添加成功".to_string())
}

// 更新应用数据
#[tauri::command]
async fn update_app(app: AppData) -> Result<String, String> {
    let mut storage = load_app_data()?;

    // 查找并更新应用
    if let Some(existing_app) = storage.apps.iter_mut().find(|a| a.id == app.id) {
        *existing_app = app;
        save_app_data(storage.apps, storage.categories, storage.selected_category)?;
        Ok("应用更新成功".to_string())
    } else {
        Err("应用不存在".to_string())
    }
}

// 根据ID获取应用数据
#[tauri::command]
async fn get_app_by_id(app_id: i64) -> Result<AppData, String> {
    let storage = load_app_data()?;

    if let Some(app) = storage.apps.iter().find(|a| a.id == app_id) {
        Ok(app.clone())
    } else {
        Err("应用不存在".to_string())
    }
}

// 打开编辑项目窗口
#[tauri::command]
async fn open_edit_project_window(
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

// 检查编辑项目窗口是否打开
#[tauri::command]
fn is_edit_project_window_open(state: State<'_, AppState>) -> bool {
    match state.edit_project_window_open.lock() {
        Ok(edit_project_open) => *edit_project_open,
        Err(_) => false, // 如果锁定失败，默认返回 false
    }
}

// 关闭编辑项目窗口
#[tauri::command]
async fn close_edit_project_window(
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
                .with_handler(|app, shortcut, event| {
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
                })
                .build(),
        )
        .setup(|app| {
            // 立即设置主窗口隐藏任务栏图标（首次启动时）
            // 添加日志记录以便调试
            println!("Setup: 尝试获取主窗口 'main'...");

            // 首先检查所有窗口，确保没有重复的主窗口
            let windows = app.webview_windows();
            println!("Setup: 当前应用中的窗口数量: {}", windows.len());
            let mut main_window_count = 0;

            for (label, window) in &windows {
                println!("Setup: 发现窗口 - 标签: {}, 窗口: {:?}", label, window);
                if label == "main" {
                    main_window_count += 1;
                }
            }

            if main_window_count > 1 {
                eprintln!(
                    "Setup: 警告 - 发现多个主窗口实例！数量: {}",
                    main_window_count
                );
            }

            match app.get_webview_window("main") {
                Some(main_window) => {
                    println!("Setup: 成功获取主窗口，设置隐藏任务栏图标");
                    if let Err(e) = main_window.set_skip_taskbar(true) {
                        eprintln!("Setup: 设置主窗口隐藏任务栏图标失败: {}", e);
                    } else {
                        println!("Setup: 成功设置主窗口隐藏任务栏图标");
                    }
                }
                None => {
                    eprintln!("Setup: 警告 - 无法获取主窗口 'main'，可能窗口尚未初始化");
                    // 列出所有可用窗口以便调试
                    println!("Setup: 当前可用窗口:");
                    for (label, _) in &windows {
                        println!("  - {}", label);
                    }
                }
            }

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

            // 注册全局快捷键和事件处理器
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

            // 创建托盘图标
            let _tray = TrayIconBuilder::with_id("main_tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Lora Launcher")
                .on_menu_event(move |app, event| {
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
                })
                .on_tray_icon_event(|tray, event| {
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
                })
                .build(app)?;

            // 为主窗口添加事件处理
            if let Some(main_window) = app.get_webview_window("main") {
                let window_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            // 阻止默认的关闭行为
                            api.prevent_close();

                            // 隐藏窗口到托盘
                            let _ = window_clone.hide();
                            // 窗口隐藏时，显示任务栏图标
                            let _ = window_clone.set_skip_taskbar(false);
                        }
                        _ => {}
                    }
                });
            }

            // 启动定时备份任务 - 延迟到应用完全启动后
            let app_handle_for_backup = app.handle().clone();
            std::thread::spawn(move || {
                // 创建一个新的 Tokio 运行时来运行备份任务
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
            load_app_data,
            save_app_settings,
            load_app_settings,
            save_window_size,
            get_main_window_size,
            update_prevent_auto_hide,
            update_tray_menu,
            toggle_window_visibility,
            open_settings_window,
            close_settings_window,
            is_settings_window_open,
            open_new_project_window,
            close_new_project_window,
            is_new_project_window_open,
            open_edit_project_window,
            close_edit_project_window,
            is_edit_project_window_open,
            get_app_by_id,
            update_app,
            add_new_app,
            quit_app,
            my_custom_command,
            get_file_info,
            launch_app,
            launch_app_with_auto_hide,
            open_url,
            open_folder,
            open_file_dialog,
            open_folder_dialog,
            detect_target_type,
            save_app_data,
            delete_app,
            update_app_category,
            save_selected_category,
            get_app_icon,
            run_as_admin,
            open_file_location,
            get_app_version,
            get_app_update_date,
            reset_settings_to_default,
            export_app_data_to_file,
            import_app_data_from_file,
            clear_all_data,
            update_theme,
            update_icon_size,
            update_sidebar_width,
            update_animations,
            update_animation_speed,
            update_start_with_system,
            update_start_minimized,
            update_auto_hide_after_launch,
            update_toggle_hotkey,
            update_global_hotkey,
            update_fuzzy_search,
            update_search_in_path,
            update_max_search_results,
            update_auto_backup,
            update_backup_interval,
            manual_backup,
            get_backup_status,
            save_ui_state,
            update_settings_batch,
            check_auto_start_status,
            export_data,
            import_data,
            reset_data,
            greet,
            notify_main_window_refresh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 获取主窗口大小
#[tauri::command]
async fn get_main_window_size(app: tauri::AppHandle) -> Result<(u32, u32), String> {
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
