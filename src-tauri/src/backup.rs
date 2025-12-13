use crate::data::get_app_data_dir;
use crate::data::{load_app_data, load_app_settings};
use chrono::{Local, Datelike, Timelike, NaiveDate};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub struct BackupManager {
    #[allow(dead_code)]
    pub is_running: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl BackupManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            is_running: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }

    pub async fn perform_backup() -> Result<String, String> {
        println!("开始执行定时备份...");

        let data_dir = get_app_data_dir()?;
        let backup_dir = data_dir.join("backups");
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败: {}", e))?;
        }

        let now = Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let backup_file_path = backup_dir.join(format!("lora_auto_backup_{}.json", timestamp));

        let storage = load_app_data()?;
        let settings = load_app_settings()?;

        let backup_data = json!({"storage": storage, "settings": settings, "backup_time": now.timestamp(), "backup_type": "auto", "version": env!("CARGO_PKG_VERSION")});
        let json_data = serde_json::to_string_pretty(&backup_data).map_err(|e| format!("序列化备份数据失败: {}", e))?;
        fs::write(&backup_file_path, json_data).map_err(|e| format!("写入备份文件失败: {}", e))?;

        // cleanup: keep last 10
        let _ = Self::cleanup_old_backups(&backup_dir);

        Ok(format!("自动备份成功完成: {}", backup_file_path.display()))
    }

    fn cleanup_old_backups(backup_dir: &Path) -> Result<(), String> {
        let mut backup_files = Vec::new();

        if let Ok(entries) = fs::read_dir(backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("lora_auto_backup_") && file_name.ends_with(".json") {
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

        backup_files.sort_by(|a, b| b.1.cmp(&a.1));

        if backup_files.len() > 10 {
            for (path, _) in backup_files.iter().skip(10) {
                let _ = fs::remove_file(path);
            }
        }

        Ok(())
    }

    fn calculate_next_backup_time(interval: &str) -> Result<chrono::DateTime<Local>, String> {
        let now = Local::now();

        match interval {
            "daily" => {
                let next_day = now.date_naive().succ_opt().ok_or("计算明天日期失败")?;
                let next_time = next_day.and_hms_opt(now.hour(), now.minute(), now.second()).ok_or("构建明天时间失败")?;
                Ok(next_time.and_local_timezone(Local).single().ok_or("构建本地时间失败")?)
            }
            "weekly" => {
                let days_until_next_week = 7 - now.weekday().num_days_from_monday() % 7;
                let next_week_date = now.date_naive() + chrono::Duration::days(days_until_next_week as i64);
                let next_time = next_week_date.and_hms_opt(now.hour(), now.minute(), now.second()).ok_or("构建下周时间失败")?;
                Ok(next_time.and_local_timezone(Local).single().ok_or("构建本地时间失败")?)
            }
            "monthly" => {
                let next_month = if now.month() == 12 { now.year() + 1 } else { now.year() };
                let next_month_num = if now.month() == 12 { 1 } else { now.month() + 1 };
                let next_day = if let Some(date) = NaiveDate::from_ymd_opt(next_month, next_month_num, now.day()) { date } else { NaiveDate::from_ymd_opt(next_month, next_month_num + 1, 1).and_then(|d| d.pred_opt()).ok_or("计算下个月最后一天失败")? };
                let next_time = next_day.and_hms_opt(now.hour(), now.minute(), now.second()).ok_or("构建下个月时间失败")?;
                Ok(next_time.and_local_timezone(Local).single().ok_or("构建本地时间失败")?)
            }
            _ => Err("不支持的备份间隔".to_string()),
        }
    }

    pub async fn start_backup_task(_app_handle: tauri::AppHandle) -> Result<(), String> {
        loop {
            let settings = load_app_settings()?;
            if !settings.auto_backup.unwrap_or(false) {
                sleep(Duration::from_secs(3600)).await;
                continue;
            }

            let backup_interval = settings.backup_interval.unwrap_or("weekly".to_string());
            let next_backup_time = Self::calculate_next_backup_time(&backup_interval)?;
            let now = Local::now();

            if next_backup_time > now {
                let duration_until_next = next_backup_time.signed_duration_since(now);
                let seconds_until_next = duration_until_next.num_seconds().max(0) as u64;
                sleep(Duration::from_secs(seconds_until_next)).await;
            }

            let _ = Self::perform_backup().await;
        }
    }
}

#[tauri::command]
pub async fn manual_backup() -> Result<String, String> {
    BackupManager::perform_backup().await
}

#[tauri::command]
pub fn get_backup_status() -> Result<serde_json::Value, String> {
    let data_dir = get_app_data_dir()?;
    let backup_dir = data_dir.join("backups");

    let mut backup_files = Vec::new();

    if backup_dir.exists() {
        if let Ok(entries) = fs::read_dir(&backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("lora_auto_backup_") && file_name.ends_with(".json") {
                            if let Ok(metadata) = fs::metadata(&path) {
                                if let Ok(modified) = metadata.modified() {
                                    if let Ok(modified_datetime) = modified.duration_since(std::time::UNIX_EPOCH) {
                                        backup_files.push(serde_json::json!({"name": file_name, "path": path.to_string_lossy(), "size": metadata.len(), "modified": modified_datetime.as_secs()}));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    backup_files.sort_by(|a, b| {
        let a_time = a.get("modified").and_then(|v| v.as_u64()).unwrap_or(0);
        let b_time = b.get("modified").and_then(|v| v.as_u64()).unwrap_or(0);
        b_time.cmp(&a_time)
    });

    Ok(serde_json::json!({"backup_dir": backup_dir.to_string_lossy(), "backup_files": backup_files, "total_count": backup_files.len()}))
}
