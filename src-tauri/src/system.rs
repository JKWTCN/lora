use std::path::Path;
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
pub fn open_url(url: String, _launch_args: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // use std::os::windows::process::CommandExt;

        // if let Some(args_str) = launch_args {
        //     if !args_str.trim().is_empty() {
        //         use std::os::windows::process::CommandExt;

        //         let split_args: Vec<String> = args_str.split_whitespace().map(|s| s.to_string()).collect();
        //         let mut cmd_args = vec!["/C".to_string(), "start".to_string(), "".to_string(), url];
        //         cmd_args.extend(split_args);
        //         let result = Command::new("cmd").args(&cmd_args).creation_flags(0x08000000).spawn();
        //         return result.map(|_| "网址打开成功".to_string()).map_err(|e| format!("打开网址失败: {}", e));
        //     }
        // }
        // let result = Command::new("cmd").creation_flags(0x08000000).args(["/C", "start", "", &url]).spawn();
        Command::new("explorer").arg(&url).spawn().map_err(|e| format!("打开网址失败: {}", e))?;
        return Ok("网址打开成功".to_string());
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
pub fn open_folder(folder_path: String, _launch_args: Option<String>) -> Result<String, String> {
    let path = Path::new(&folder_path);
    if !path.exists() {
        return Err("文件夹不存在".to_string());
    }
    if !path.is_dir() {
        return Err("路径不是文件夹".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // use std::os::windows::process::CommandExt;

        // let mut cmd = Command::new("explorer");
        // cmd.creation_flags(0x08000000);
        // cmd.arg(&folder_path);
        // if let Some(args_str) = launch_args {
        //     if !args_str.trim().is_empty() {
        //         let split_args: Vec<String> =
        //             args_str.split_whitespace().map(|s| s.to_string()).collect();
        //         cmd.args(&split_args);
        //     }
        // }
        // let result = cmd.spawn();
        // return result
        //     .map(|_| "文件夹打开成功".to_string())
        //     .map_err(|e| format!("打开文件夹失败: {}", e));
        Command::new("explorer").arg(&folder_path).spawn().map_err(|e| format!("打开文件夹失败: {}", e))?;
        return Ok("文件夹打开成功".to_string());
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
        use std::os::windows::process::CommandExt;

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

#[tauri::command]
pub fn open_folder_dialog(title: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

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
