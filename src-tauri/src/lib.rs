use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

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
}

// 分类数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub is_default: bool,
}

// 应用数据存储结构
#[derive(Serialize, Deserialize, Debug)]
pub struct AppStorage {
    pub apps: Vec<AppData>,
    pub categories: Vec<CategoryData>,
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

    // 获取文件名（不包含扩展名）
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未知应用")
        .to_string();

    // 获取文件扩展名
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 不再限制文件类型，所有文件都可以添加
    let is_executable = true; // 允许所有文件类型

    // 获取文件大小
    let metadata = fs::metadata(&path).map_err(|e| format!("获取文件信息失败: {}", e))?;
    let size = metadata.len();

    let mut target_path = None;
    let is_shortcut = extension == "lnk" || extension == "url" || extension == "desktop";

    // 如果是快捷方式，尝试解析目标路径
    if is_shortcut {
        target_path = resolve_shortcut_target(&file_path);
    }

    Ok(serde_json::json!({
        "name": name,
        "path": file_path,
        "extension": extension,
        "size": size,
        "is_executable": is_executable,
        "is_shortcut": is_shortcut,
        "target_path": target_path
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
    // 简单的.lnk文件解析（这是一个简化版本）
    // 在实际项目中，您可能需要使用Windows API或第三方库
    // 这里我们返回快捷方式本身的路径，实际应用中可以增强
    let path = Path::new(shortcut_path);
    if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
        // 这里可以添加更复杂的快捷方式解析逻辑
        // 暂时返回原路径
        Some(shortcut_path.to_string())
    } else {
        Some(shortcut_path.to_string())
    }
}

#[tauri::command]
fn launch_app(app_path: String) -> Result<String, String> {
    let path = Path::new(&app_path);

    if !path.exists() {
        return Err("应用文件不存在".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let result = Command::new("cmd")
            .args(["/C", "start", "", &app_path])
            .spawn();

        match result {
            Ok(_) => Ok("应用启动成功".to_string()),
            Err(e) => Err(format!("启动应用失败: {}", e)),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let result = Command::new(&app_path).spawn();

        match result {
            Ok(_) => Ok("应用启动成功".to_string()),
            Err(e) => Err(format!("启动应用失败: {}", e)),
        }
    }
}

// 获取应用数据目录
fn get_app_data_dir() -> Result<std::path::PathBuf, String> {
    // 使用临时路径作为数据目录
    let mut data_dir = std::env::temp_dir();
    data_dir.push("lora_launcher");

    // 确保目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    Ok(data_dir)
}

// 保存应用数据
#[tauri::command]
fn save_app_data(apps: Vec<AppData>, categories: Vec<CategoryData>) -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("apps.json");

    let storage = AppStorage { apps, categories };
    let json_data =
        serde_json::to_string_pretty(&storage).map_err(|e| format!("序列化数据失败: {}", e))?;

    fs::write(&file_path, json_data).map_err(|e| format!("保存文件失败: {}", e))?;

    Ok("数据保存成功".to_string())
}

// 加载应用数据
#[tauri::command]
fn load_app_data() -> Result<AppStorage, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join("apps.json");

    if !file_path.exists() {
        // 如果文件不存在，返回默认数据
        return Ok(AppStorage {
            apps: vec![],
            categories: vec![
                CategoryData {
                    id: "all".to_string(),
                    name: "全部应用".to_string(),
                    icon: "icon-apps".to_string(),
                    is_default: true,
                },
                CategoryData {
                    id: "utilities".to_string(),
                    name: "实用工具".to_string(),
                    icon: "icon-settings".to_string(),
                    is_default: false,
                },
            ],
        });
    }

    let json_data = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;

    let storage: AppStorage =
        serde_json::from_str(&json_data).map_err(|e| format!("解析数据失败: {}", e))?;

    Ok(storage)
}

// 删除应用
#[tauri::command]
fn delete_app(app_id: i64) -> Result<String, String> {
    let mut storage = load_app_data()?;
    storage.apps.retain(|app| app.id != app_id);
    save_app_data(storage.apps, storage.categories)?;
    Ok("应用删除成功".to_string())
}

// 更新应用分类
#[tauri::command]
fn update_app_category(app_id: i64, new_category: String) -> Result<String, String> {
    let mut storage = load_app_data()?;

    if let Some(app) = storage.apps.iter_mut().find(|app| app.id == app_id) {
        app.category = new_category;
        save_app_data(storage.apps, storage.categories)?;
        Ok("应用分类更新成功".to_string())
    } else {
        Err("应用不存在".to_string())
    }
}
#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_app_data,
            my_custom_command,
            get_file_info,
            launch_app,
            save_app_data,
            delete_app,
            update_app_category,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
