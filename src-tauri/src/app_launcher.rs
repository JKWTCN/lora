//! 应用启动器模块
//!
//! 此模块包含与应用启动相关的功能，包括：
//! - 获取文件信息
//! - 启动应用程序
//! - 自动隐藏窗口后启动应用
//! - 提取应用图标

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tauri::{AppHandle, Manager};

// 导入项目内部模块
use crate::data::load_app_settings;
use crate::helpers::{extract_file_icon, extract_icon_from_exe, resolve_shortcut_target};
use crate::models::AppSettings;

/// 获取文件信息
///
/// 此函数用于获取指定文件的详细信息，包括文件名、扩展名、大小等。
/// 对于快捷方式文件，会尝试解析其目标路径并获取目标文件的信息。
#[tauri::command]
pub fn get_file_info(file_path: String) -> Result<serde_json::Value, String> {
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
    // 如果是快捷方式，应返回快捷方式文件本身的名字，而不是它指向的目标名字
    let name = if is_shortcut {
        path.file_stem().and_then(|s| s.to_str()).unwrap_or("未知应用").to_string()
    } else {
        actual_path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("未知应用")
            .to_string()
    };

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

/// 启动应用程序
///
/// 此函数用于启动指定的应用程序，支持传递启动参数。
/// 对于快捷方式文件，会先解析其目标路径，然后启动目标应用程序。
#[tauri::command]
pub fn launch_app(app_path: String, launch_args: Option<String>) -> Result<String, String> {
    let path = Path::new(&app_path);

    if !path.exists() {
        return Err("应用文件不存在".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // 如果 launch_args 不存在或者为 ""
        if launch_args.is_none() || launch_args.as_ref().unwrap().trim().is_empty() {
            Command::new("explorer").arg(app_path).spawn().map_err(|e| format!("启动应用失败: {}", e))?;
            return Ok("应用启动成功".to_string());
        }
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
                return crate::system::open_url(url, launch_args);
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

/// 启动应用并检查是否需要自动隐藏窗口
///
/// 此函数先启动应用程序，然后根据设置决定是否自动隐藏主窗口。
/// 如果启用了自动隐藏功能，会在应用启动后延迟一段时间再隐藏窗口。
#[tauri::command]
pub async fn launch_app_with_auto_hide(
    app: AppHandle,
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
            let _: Result<(), _> = window.hide();
        }
    }

    Ok(launch_result)
}

/// 获取应用图标
///
/// 此函数用于从可执行文件中提取图标，并将其转换为 base64 编码的 PNG 图像。
/// 主要用于 Windows 平台，使用 Windows API 提取图标。
#[tauri::command]
pub fn get_app_icon(file_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // 如果传入的是 URL（网站），尝试使用 helpers::fetch_favicon
        if file_path.starts_with("http://") || file_path.starts_with("https://") {
            if let Ok(fav) = crate::helpers::fetch_favicon(&file_path) {
                return Ok(fav);
            }
        }
        // 优先尝试解压 exe 方法
        println!("优先尝试解压 exe 文件");
        match extract_icon_from_exe(&file_path) {
            Ok(icon_data) => Ok(icon_data),
            Err(_) => {
                // 如果解压失败，尝试使用 Windows API 方法
                println!("解压失败，尝试使用 Windows API 方法");
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
                    // 最后回退到 PowerShell 方法
                    println!("Windows API 方法也失败，尝试使用 PowerShell 方法");
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

/// 获取默认设置
///
/// 此函数返回应用程序的默认设置，用于在无法加载设置时提供后备值。
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
