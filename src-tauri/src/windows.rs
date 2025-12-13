use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use std::process::Command;
#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn check_auto_start_status() -> Result<bool, String> {
    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    let app_name = "Lora";

    let script = format!(r#"
        try {{
            $value = Get-ItemProperty -Path "HKCU:\\{}" -Name "{}" -ErrorAction SilentlyContinue
            if ($value) {{ Write-Output "TRUE" }} else {{ Write-Output "FALSE" }}
        }} catch {{ Write-Output "FALSE" }}
        "#, reg_key, app_name);

    let output = Command::new("powershell").args(["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", &script]).creation_flags(0x08000000).output().map_err(|e| format!("PowerShell执行失败: {}", e))?;
    let result_string = String::from_utf8_lossy(&output.stdout);
    let result = result_string.trim();
    Ok(result == "TRUE")
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn run_as_admin(app_path: String) -> Result<String, String> {
    let result = Command::new("powershell").args(["-WindowStyle", "Hidden", "-Command", &format!("Start-Process '{}' -Verb RunAs", app_path.replace("'", "''"))]).creation_flags(0x08000000).spawn();
    match result {
        Ok(_) => Ok("应用以管理员权限启动成功".to_string()),
        Err(e) => Err(format!("以管理员权限启动应用失败: {}", e)),
    }
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn open_file_location(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    let dir_path = path.parent().ok_or("无法获取文件目录")?.to_str().ok_or("路径包含无效字符")?;
    let result = Command::new("explorer").creation_flags(0x08000000).arg(dir_path).spawn();
    match result { Ok(_) => Ok("已打开文件所在目录".to_string()), Err(e) => Err(format!("打开目录失败: {}", e)) }
}
