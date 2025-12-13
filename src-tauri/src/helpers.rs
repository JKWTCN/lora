use std::fs;
use std::path::Path;
use std::process::Command;
use url::Url;
use base64::Engine as _Base64Engine;

// 解析快捷方式目标路径
pub fn resolve_shortcut_target(shortcut_path: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use lnk::ShellLink;

        let path = Path::new(shortcut_path);

        // 检查文件扩展名
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            match extension.to_lowercase().as_str() {
                "lnk" => {
                    // 解析.lnk文件
                    match ShellLink::open(shortcut_path) {
                        Ok(link) => {
                            if let Some(link_info) = link.link_info() {
                                if let Some(target) = link_info.local_base_path() {
                                    return Some(target.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            println!("解析.lnk文件失败: {:?}", e);
                        }
                    }
                }
                "url" => {
                    if let Ok(content) = fs::read_to_string(shortcut_path) {
                        for line in content.lines() {
                            if line.starts_with("URL=") {
                                let url = line.trim_start_matches("URL=");
                                return Some(url.to_string());
                            }
                        }
                    }
                }
                _ => {
                    return Some(shortcut_path.to_string());
                }
            }
        }

        Some(shortcut_path.to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

// 基于扩展名的简单图标标识符
pub fn extract_file_icon(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 如果路径看起来像 URL，尝试抓取 favicon
    if file_path.starts_with("http://") || file_path.starts_with("https://") {
        if let Ok(data_uri) = fetch_favicon(file_path) {
            return Some(data_uri);
        }
    }

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
        "url" => {
            // 有些快捷方式是 .url 文件，handle elsewhere; fallthrough to attempt fetch
            if let Ok(data_uri) = fetch_favicon(file_path) {
                return Some(data_uri);
            }
            Some("web".to_string())
        }
        "js" | "ts" => Some("javascript".to_string()),
        "py" => Some("python".to_string()),
        "java" => Some("java".to_string()),
        "cpp" | "c" | "h" => Some("code".to_string()),
        "json" => Some("json".to_string()),
        "xml" => Some("xml".to_string()),
        "css" => Some("css".to_string()),
        _ => None,
    }
}

// 同步函数：尝试从给定 URL 抓取 favicon 并返回 data URI
pub fn fetch_favicon(page_url: &str) -> Result<String, String> {
    // 尝试解析 URL
    let parsed = Url::parse(page_url).map_err(|e| format!("无效 URL: {}", e))?;

    // 首先尝试访问页面 HTML，以查找 <link rel="icon" ...>
    let client = reqwest::blocking::Client::builder()
        .user_agent("lora-favicon-fetcher/1.0")
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let resp = client
        .get(page_url)
        .send()
        .map_err(|e| format!("请求页面失败: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("页面返回错误状态: {}", resp.status()));
    }

    let body = resp.text().map_err(|e| format!("读取页面内容失败: {}", e))?;

    // 查找可能的 favicon 链接（rel=icon, shortcut icon）
    let mut favicon_url: Option<String> = None;
    for line in body.lines() {
        let lower = line.to_lowercase();
        if lower.contains("rel=\"icon\"") || lower.contains("rel=\'icon\'") || lower.contains("rel=\"shortcut icon\"") {
            // 简单解析 href
            if let Some(href_pos) = lower.find("href=") {
                let rest = &line[href_pos + 5..];
                // 提取引号内的值
                if let Some(start_quote) = rest.find('"') {
                    if let Some(end_quote) = rest[start_quote + 1..].find('"') {
                        let href = &rest[start_quote + 1..start_quote + 1 + end_quote];
                        favicon_url = Some(href.to_string());
                        break;
                    }
                } else if let Some(start_quote) = rest.find('\'') {
                    if let Some(end_quote) = rest[start_quote + 1..].find('\'') {
                        let href = &rest[start_quote + 1..start_quote + 1 + end_quote];
                        favicon_url = Some(href.to_string());
                        break;
                    }
                }
            }
        }
    }

    // 如果未在 HTML 中找到 favicon 链接，尝试 /favicon.ico
    let favicon_candidates = if let Some(fav) = favicon_url {
        vec![fav]
    } else {
        vec!["/favicon.ico".to_string()]
    };

    for candidate in favicon_candidates {
        // 解析相对 URL
        let fav_url: Url = if let Ok(u) = Url::parse(&candidate) {
            u
        } else if let Ok(u) = parsed.join(&candidate) {
            u
        } else {
            continue;
        };

        match client.get(fav_url.as_str()).send() {
            Ok(mut r) => {
                if r.status().is_success() {
                    let mut buf: Vec<u8> = Vec::new();
                    if r.copy_to(&mut buf).is_ok() {
                        let encoded = base64::engine::general_purpose::STANDARD.encode(&buf);
                        return Ok(format!("data:image/x-icon;base64,{}", encoded));
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Err("未能获取 favicon".to_string())
}

// extract_icon_from_exe retained for potential use by get_app_icon
pub fn extract_icon_from_exe(exe_path: &str) -> Result<String, String> {
    use base64::engine::general_purpose;
    use base64::Engine;
    use image::ImageEncoder;

    let temp_dir = format!(
        "temp_icons_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    if !Path::new(&temp_dir).exists() {
        fs::create_dir(&temp_dir).map_err(|e| format!("创建临时目录失败: {}", e))?;
    }

    let output = Command::new("7z")
        .args(&["e", exe_path, &format!("-o{}", temp_dir)])
        .output()
        .map_err(|e| format!("执行 7z 失败: {}", e))?;

    if !output.status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err("7z 提取失败".to_string());
    }

    let mut ico_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "ico" {
                    if let Ok(metadata) = fs::metadata(&path) {
                        ico_files.push((path, metadata.len()));
                    }
                }
            }
        }
    }

    let ico_path = ico_files
        .into_iter()
        .max_by_key(|&(_, size)| size)
        .map(|(path, _)| path);

    let result = if let Some(ico_path) = ico_path {
        match image::open(&ico_path) {
            Ok(img) => {
                let img_buffer = img.to_rgba8();
                let mut png_bytes: Vec<u8> = Vec::new();
                let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
                if encoder
                    .write_image(
                        img_buffer.as_raw(),
                        img_buffer.width(),
                        img_buffer.height(),
                        image::ColorType::Rgba8,
                    )
                    .is_ok()
                {
                    let base64_encoded = general_purpose::STANDARD.encode(&png_bytes);
                    Ok(format!("data:image/png;base64,{}", base64_encoded))
                } else {
                    Err("转换 ico 为 PNG 失败".to_string())
                }
            }
            Err(e) => Err(format!("加载 ico 文件失败: {}", e)),
        }
    } else {
        Err("未找到 ico 文件".to_string())
    };

    let _ = fs::remove_dir_all(&temp_dir);

    result
}
