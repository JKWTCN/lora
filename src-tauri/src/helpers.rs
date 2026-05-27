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

// extract_icon_from_exe retained for potential use by get_app_icon
pub fn extract_icon_from_exe(exe_path: &str) -> Result<String, String> {
    use base64::engine::general_purpose;
    use base64::Engine;
    use image::ImageEncoder;

    let pe_data = fs::read(exe_path).map_err(|e| format!("读取 exe 文件失败: {}", e))?;
    let ico_data = extract_ico_from_pe_resources(&pe_data)?;
    let img = image::load_from_memory_with_format(&ico_data, image::ImageFormat::Ico)
        .map_err(|e| format!("加载 ico 资源失败: {}", e))?;

    let img_buffer = img.to_rgba8();
    let mut png_bytes: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    encoder
        .write_image(
            img_buffer.as_raw(),
            img_buffer.width(),
            img_buffer.height(),
            image::ColorType::Rgba8,
        )
        .map_err(|e| format!("转换 ico 为 PNG 失败: {}", e))?;

    let base64_encoded = general_purpose::STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_encoded))
}

#[derive(Clone, Copy)]
struct PeSection {
    virtual_address: u32,
    virtual_size: u32,
    raw_offset: u32,
    raw_size: u32,
}

#[derive(Clone, Copy)]
struct ResourceData {
    id: u16,
    offset: usize,
    size: usize,
}

fn extract_ico_from_pe_resources(data: &[u8]) -> Result<Vec<u8>, String> {
    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;

    let pe = parse_pe(data)?;
    let icon_resources = pe.resource_entries(RT_ICON)?;
    let group_resources = pe.resource_entries(RT_GROUP_ICON)?;

    let group = group_resources
        .into_iter()
        .max_by_key(|resource| resource.size)
        .ok_or_else(|| "未找到 GROUP_ICON 资源".to_string())?;

    let group_data = data
        .get(group.offset..group.offset + group.size)
        .ok_or_else(|| "GROUP_ICON 资源范围无效".to_string())?;

    build_ico_from_group_icon(group_data, &icon_resources, data)
}

fn build_ico_from_group_icon(
    group_data: &[u8],
    icon_resources: &[ResourceData],
    pe_data: &[u8],
) -> Result<Vec<u8>, String> {
    if group_data.len() < 6 {
        return Err("GROUP_ICON 资源太短".to_string());
    }

    let icon_type = read_u16(group_data, 2)?;
    let count = read_u16(group_data, 4)? as usize;
    if icon_type != 1 || count == 0 {
        return Err("GROUP_ICON 格式无效".to_string());
    }

    let entries_size = count
        .checked_mul(14)
        .ok_or_else(|| "GROUP_ICON 条目数量无效".to_string())?;
    if group_data.len() < 6 + entries_size {
        return Err("GROUP_ICON 条目不完整".to_string());
    }

    let mut icon_images: Vec<Vec<u8>> = Vec::with_capacity(count);
    let mut ico = Vec::with_capacity(6 + count * 16);
    ico.extend_from_slice(&group_data[0..6]);

    let mut image_offset = 6 + count * 16;
    for index in 0..count {
        let entry_offset = 6 + index * 14;
        let image_id = read_u16(group_data, entry_offset + 12)?;
        let resource = icon_resources
            .iter()
            .find(|resource| resource.id == image_id)
            .ok_or_else(|| format!("未找到 ICON 资源: {}", image_id))?;

        let image = pe_data
            .get(resource.offset..resource.offset + resource.size)
            .ok_or_else(|| format!("ICON 资源范围无效: {}", image_id))?;

        ico.extend_from_slice(&group_data[entry_offset..entry_offset + 8]);
        ico.extend_from_slice(&(image.len() as u32).to_le_bytes());
        ico.extend_from_slice(&(image_offset as u32).to_le_bytes());
        icon_images.push(image.to_vec());
        image_offset = image_offset
            .checked_add(image.len())
            .ok_or_else(|| "ICO 文件偏移溢出".to_string())?;
    }

    for image in icon_images {
        ico.extend_from_slice(&image);
    }

    Ok(ico)
}

struct ParsedPe<'a> {
    data: &'a [u8],
    resource_root_offset: usize,
    sections: Vec<PeSection>,
}

fn parse_pe(data: &[u8]) -> Result<ParsedPe<'_>, String> {
    if data.get(0..2) != Some(b"MZ") {
        return Err("不是有效的 Windows PE 文件".to_string());
    }

    let pe_offset = read_u32(data, 0x3c)? as usize;
    if data.get(pe_offset..pe_offset + 4) != Some(b"PE\0\0") {
        return Err("PE 文件头无效".to_string());
    }

    let number_of_sections = read_u16(data, pe_offset + 6)? as usize;
    let optional_header_size = read_u16(data, pe_offset + 20)? as usize;
    let optional_header_offset = pe_offset + 24;
    let magic = read_u16(data, optional_header_offset)?;
    let data_directory_offset = match magic {
        0x10b => optional_header_offset + 96,
        0x20b => optional_header_offset + 112,
        _ => return Err("不支持的 PE optional header 格式".to_string()),
    };

    let resource_rva = read_u32(data, data_directory_offset + 16)?;
    if resource_rva == 0 {
        return Err("PE 文件不包含资源表".to_string());
    }

    let section_table_offset = optional_header_offset + optional_header_size;
    let mut sections = Vec::with_capacity(number_of_sections);
    for index in 0..number_of_sections {
        let section_offset = section_table_offset + index * 40;
        sections.push(PeSection {
            virtual_size: read_u32(data, section_offset + 8)?,
            virtual_address: read_u32(data, section_offset + 12)?,
            raw_size: read_u32(data, section_offset + 16)?,
            raw_offset: read_u32(data, section_offset + 20)?,
        });
    }

    let resource_root_offset =
        rva_to_offset(resource_rva, &sections).ok_or_else(|| "无法定位 PE 资源表".to_string())?;

    Ok(ParsedPe {
        data,
        resource_root_offset,
        sections,
    })
}

impl ParsedPe<'_> {
    fn resource_entries(&self, resource_type: u16) -> Result<Vec<ResourceData>, String> {
        let Some(type_directory) = self.find_resource_directory(0, resource_type)? else {
            return Ok(Vec::new());
        };

        let mut resources = Vec::new();
        for name_entry in self.resource_directory_entries(type_directory)? {
            if !name_entry.is_directory {
                continue;
            }

            for language_entry in self.resource_directory_entries(name_entry.target_offset)? {
                if language_entry.is_directory {
                    continue;
                }

                let data_entry_offset = self.resource_root_offset + language_entry.target_offset;
                let data_rva = read_u32(self.data, data_entry_offset)?;
                let data_size = read_u32(self.data, data_entry_offset + 4)? as usize;
                if let Some(data_offset) = rva_to_offset(data_rva, &self.sections) {
                    resources.push(ResourceData {
                        id: name_entry.id,
                        offset: data_offset,
                        size: data_size,
                    });
                }
            }
        }

        Ok(resources)
    }

    fn find_resource_directory(
        &self,
        directory_offset: usize,
        id: u16,
    ) -> Result<Option<usize>, String> {
        for entry in self.resource_directory_entries(directory_offset)? {
            if entry.id == id && entry.is_directory {
                return Ok(Some(entry.target_offset));
            }
        }

        Ok(None)
    }

    fn resource_directory_entries(
        &self,
        directory_offset: usize,
    ) -> Result<Vec<ResourceDirectoryEntry>, String> {
        let absolute_directory_offset = self.resource_root_offset + directory_offset;
        let named_count = read_u16(self.data, absolute_directory_offset + 12)? as usize;
        let id_count = read_u16(self.data, absolute_directory_offset + 14)? as usize;
        let total_count = named_count
            .checked_add(id_count)
            .ok_or_else(|| "资源目录条目数量无效".to_string())?;

        let mut entries = Vec::with_capacity(total_count);
        for index in 0..total_count {
            let entry_offset = absolute_directory_offset + 16 + index * 8;
            let name = read_u32(self.data, entry_offset)?;
            let offset_to_data = read_u32(self.data, entry_offset + 4)?;
            let is_directory = (offset_to_data & 0x8000_0000) != 0;
            entries.push(ResourceDirectoryEntry {
                id: (name & 0xffff) as u16,
                is_directory,
                target_offset: (offset_to_data & 0x7fff_ffff) as usize,
            });
        }

        Ok(entries)
    }
}

struct ResourceDirectoryEntry {
    id: u16,
    is_directory: bool,
    target_offset: usize,
}

fn rva_to_offset(rva: u32, sections: &[PeSection]) -> Option<usize> {
    sections.iter().find_map(|section| {
        let section_size = section.virtual_size.max(section.raw_size);
        let section_end = section.virtual_address.checked_add(section_size)?;
        if rva >= section.virtual_address && rva < section_end {
            let offset = section
                .raw_offset
                .checked_add(rva - section.virtual_address)?;
            Some(offset as usize)
        } else {
            None
        }
    })
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, String> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| "读取 u16 超出文件范围".to_string())?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, String> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| "读取 u32 超出文件范围".to_string())?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
