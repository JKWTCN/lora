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
    pub selected_category: Option<String>, // 记住当前选中的分组
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

    // 提取文件图标
    let icon_base64 = extract_file_icon(&file_path);

    Ok(serde_json::json!({
        "name": name,
        "path": file_path,
        "extension": extension,
        "size": size,
        "is_executable": is_executable,
        "is_shortcut": is_shortcut,
        "target_path": target_path,
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

// 加载应用数据
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
#[tauri::command]
fn get_app_icon(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // 使用系统关联的图标
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

        let output = Command::new("powershell")
            .args(["-ExecutionPolicy", "Bypass", "-Command", &script])
            .output()
            .map_err(|e| format!("PowerShell执行失败: {}", e))?;

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !result.is_empty() && result.starts_with("data:image/png;base64,") {
            Ok(result)
        } else {
            Err("无法提取图标".to_string())
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
                "-Command",
                &format!(
                    "Start-Process '{}' -Verb RunAs",
                    app_path.replace("'", "''")
                ),
            ])
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

// 在文件资源管理器中显示文件
#[tauri::command]
fn show_in_explorer(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let result = Command::new("explorer")
            .args(["/select,", &file_path])
            .spawn();

        match result {
            Ok(_) => Ok("已在资源管理器中显示文件".to_string()),
            Err(e) => Err(format!("在资源管理器中显示文件失败: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let result = Command::new("open").args(["-R", &file_path]).spawn();

        match result {
            Ok(_) => Ok("已在Finder中显示文件".to_string()),
            Err(e) => Err(format!("在Finder中显示文件失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 在Linux上，尝试使用不同的文件管理器
        let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm"];
        let parent_dir = Path::new(&file_path).parent().ok_or("无法获取父目录")?;

        for manager in &file_managers {
            if Command::new("which").arg(manager).output().is_ok() {
                let result = Command::new(manager).arg(parent_dir).spawn();

                if result.is_ok() {
                    return Ok("已在文件管理器中显示目录".to_string());
                }
            }
        }

        Err("未找到可用的文件管理器".to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("当前平台不支持在文件管理器中显示文件".to_string())
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
        let result = Command::new("explorer").arg(dir_path).spawn();

        match result {
            Ok(_) => Ok("已打开文件所在目录".to_string()),
            Err(e) => Err(format!("打开目录失败: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let result = Command::new("open").arg(dir_path).spawn();

        match result {
            Ok(_) => Ok("已打开文件所在目录".to_string()),
            Err(e) => Err(format!("打开目录失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm"];

        for manager in &file_managers {
            if Command::new("which").arg(manager).output().is_ok() {
                let result = Command::new(manager).arg(dir_path).spawn();

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
            save_selected_category,
            get_app_icon,
            run_as_admin,
            show_in_explorer,
            open_file_location,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
