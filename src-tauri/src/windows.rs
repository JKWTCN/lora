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
    let follow_shortcut = crate::data::load_app_settings()?
        .open_shortcut_target_location
        .unwrap_or(false);
    let target_path = if follow_shortcut
        && Path::new(&file_path)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("lnk"))
    {
        crate::helpers::resolve_shortcut_target(&file_path)
            .filter(|target| Path::new(target).exists())
    } else {
        None
    };
    let path = Path::new(target_path.as_deref().unwrap_or(&file_path));
    if !path.exists() {
        return Err("文件或文件夹不存在".to_string());
    }
    let selected_path = path.to_str().ok_or("路径包含无效字符")?;
    let params = format!("/select,\"{}\"", selected_path);
    crate::win_native::shell_execute("explorer.exe", Some(&params), None, Some("open"))
        .map(|_| "已打开文件所在目录并选中文件".to_string())
        .map_err(|e| format!("打开文件位置失败: {}", e))
}
