#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn check_auto_start_status() -> Result<bool, String> {
    crate::win_native::is_auto_start_enabled("Lora")
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn run_as_admin(app_path: String, launch_args: Option<String>) -> Result<String, String> {
    let work_dir = Path::new(&app_path).parent().and_then(|p| p.to_str());
    let params = launch_args
        .as_deref()
        .filter(|args| !args.trim().is_empty());
    crate::win_native::shell_execute(&app_path, params, work_dir, Some("runas"))
        .map(|_| "应用以管理员权限启动成功".to_string())
        .map_err(|e| format!("以管理员权限启动应用失败: {}", e))
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn open_file_location(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    let dir_path = path
        .parent()
        .ok_or("无法获取文件目录")?
        .to_str()
        .ok_or("路径包含无效字符")?;
    crate::win_native::shell_execute(dir_path, None, None, Some("open"))
        .map(|_| "已打开文件所在目录".to_string())
        .map_err(|e| format!("打开目录失败: {}", e))
}
