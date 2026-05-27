use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::process::Command;

#[tauri::command]
pub fn detect_target_type(target_path: String) -> Result<String, String> {
    if target_path.starts_with("http://")
        || target_path.starts_with("https://")
        || target_path.starts_with("ftp://")
        || target_path.starts_with("file://")
    {
        return Ok("url".to_string());
    }

    let path = Path::new(&target_path);
    if path.exists() {
        if path.is_dir() {
            Ok("folder".to_string())
        } else {
            Ok("file".to_string())
        }
    } else {
        if path.extension().is_some() {
            Ok("file".to_string())
        } else {
            Ok("folder".to_string())
        }
    }
}

#[tauri::command]
pub fn open_url(url: String, launch_args: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let params = launch_args
            .as_deref()
            .filter(|args| !args.trim().is_empty());
        return crate::win_native::shell_execute(&url, params, None, Some("open"))
            .map(|_| "网址打开成功".to_string())
            .map_err(|e| format!("打开网址失败: {}", e));
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
        return result
            .map(|_| "网址打开成功".to_string())
            .map_err(|e| format!("打开网址失败: {}", e));
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
        return result
            .map(|_| "网址打开成功".to_string())
            .map_err(|e| format!("打开网址失败: {}", e));
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("当前平台不支持打开网址".to_string())
    }
}

#[tauri::command]
pub fn open_folder(folder_path: String, launch_args: Option<String>) -> Result<String, String> {
    let path = Path::new(&folder_path);
    if !path.exists() {
        return Err("文件夹不存在".to_string());
    }
    if !path.is_dir() {
        return Err("路径不是文件夹".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let params = launch_args
            .as_deref()
            .filter(|args| !args.trim().is_empty());
        return crate::win_native::shell_execute(&folder_path, params, None, Some("open"))
            .map(|_| "文件夹打开成功".to_string())
            .map_err(|e| format!("打开文件夹失败: {}", e));
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
        return result
            .map(|_| "文件夹打开成功".to_string())
            .map_err(|e| format!("打开文件夹失败: {}", e));
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

#[tauri::command]
pub fn open_file_dialog(
    title: String,
    filters: Vec<(String, Vec<String>)>,
) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        crate::win_native::open_file_dialog(&title, &filters)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件选择对话框".to_string())
    }
}

#[tauri::command]
pub fn open_folder_dialog(title: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        crate::win_native::open_folder_dialog(&title)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件夹选择对话框".to_string())
    }
}
