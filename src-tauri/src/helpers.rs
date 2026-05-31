use base64::Engine as _Base64Engine;
use std::fs;
use std::path::Path;
use std::time::Duration;
use url::Url;

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

    let client = reqwest::blocking::Client::builder()
        .user_agent("lora-favicon-fetcher/1.0")
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let mut favicon_candidates = Vec::new();
    if let Ok(resp) = client.get(page_url).send() {
        if resp.status().is_success() {
            if let Ok(body) = resp.text() {
                favicon_candidates.extend(extract_favicon_candidates(&body));
            }
        }
    }

    favicon_candidates.extend([
        "/favicon.ico".to_string(),
        "/favicon.png".to_string(),
        "/apple-touch-icon.png".to_string(),
    ]);

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
                        let mime_type = favicon_mime_type(fav_url.path());
                        return Ok(format!("data:{};base64,{}", mime_type, encoded));
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Err("未能获取 favicon".to_string())
}

fn extract_favicon_candidates(html: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    let mut rest = html;

    while let Some(tag_start) = rest.find("<link") {
        rest = &rest[tag_start + 5..];
        let Some(tag_end) = rest.find('>') else {
            break;
        };

        let tag = &rest[..tag_end];
        let rel = extract_html_attr(tag, "rel").unwrap_or_default().to_lowercase();
        if rel.split_whitespace().any(|value| {
            matches!(
                value,
                "icon" | "shortcut" | "apple-touch-icon" | "mask-icon" | "fluid-icon"
            )
        }) {
            if let Some(href) = extract_html_attr(tag, "href") {
                if !href.trim().is_empty() {
                    candidates.push(href);
                }
            }
        }

        rest = &rest[tag_end + 1..];
    }

    candidates
}

fn extract_html_attr(tag: &str, attr_name: &str) -> Option<String> {
    let mut rest = tag.trim();

    while !rest.is_empty() {
        rest = rest.trim_start();
        let name_end = rest
            .find(|ch: char| ch.is_whitespace() || ch == '=')
            .unwrap_or(rest.len());
        if name_end == 0 {
            break;
        }

        let name = &rest[..name_end];
        rest = rest[name_end..].trim_start();
        if !rest.starts_with('=') {
            continue;
        }

        rest = rest[1..].trim_start();
        let (value, remaining) = if let Some(quote) = rest.chars().next().filter(|ch| {
            *ch == '"' || *ch == '\''
        }) {
            let value_start = quote.len_utf8();
            if let Some(value_end) = rest[value_start..].find(quote) {
                (
                    rest[value_start..value_start + value_end].to_string(),
                    &rest[value_start + value_end + quote.len_utf8()..],
                )
            } else {
                (rest[value_start..].to_string(), "")
            }
        } else {
            let value_end = rest
                .find(|ch: char| ch.is_whitespace())
                .unwrap_or(rest.len());
            (
                rest[..value_end].trim_end_matches('/').to_string(),
                &rest[value_end..],
            )
        };

        if name.eq_ignore_ascii_case(attr_name) {
            return Some(value);
        }

        rest = remaining;
    }

    None
}

fn favicon_mime_type(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "image/x-icon",
    }
}
