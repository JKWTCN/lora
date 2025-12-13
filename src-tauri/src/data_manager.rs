//! 数据管理模块
//! 
//! 此模块负责处理应用数据的导入、导出和重置功能。
//! 提供了用户界面友好的数据备份和恢复操作。

use std::os::windows::process::CommandExt;
use std::process::Command;
use crate::data;
use crate::system::open_file_dialog;

/// 导出数据到用户选择的文件
/// 
/// 此函数会打开一个文件保存对话框，让用户选择导出位置，
/// 然后将应用数据和设置导出为 JSON 格式。
#[tauri::command]
pub fn export_data() -> Result<String, String> {
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

        data::export_app_data_to_file(file_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 其他平台使用默认路径
        let data_dir = data::get_app_data_dir()?;
        let file_path = data_dir.join(format!(
            "lora_backup_{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        data::export_app_data_to_file(file_path.to_string_lossy().to_string())
    }
}

/// 从用户选择的文件导入数据
/// 
/// 此函数会打开一个文件选择对话框，让用户选择要导入的数据文件，
/// 然后将文件中的数据和设置导入到应用中。
#[tauri::command]
pub fn import_data() -> Result<String, String> {
    // 打开文件选择对话框
    #[cfg(target_os = "windows")]
    {
        // 使用已经实现的open_file_dialog函数
        let filters = vec![
            ("JSON文件".to_string(), vec!["json".to_string()]),
            ("所有文件".to_string(), vec!["*".to_string()]),
        ];

        let file_path = open_file_dialog("选择要导入的数据文件".to_string(), filters)?;
        data::import_app_data_from_file(file_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前平台不支持文件导入对话框".to_string())
    }
}

/// 重置所有数据
/// 
/// 此函数会清除所有应用数据和设置，恢复到初始状态。
/// 这是一个不可逆操作，请谨慎使用。
#[tauri::command]
pub fn reset_data() -> Result<String, String> {
    data::clear_all_data()
}