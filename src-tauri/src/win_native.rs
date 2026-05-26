#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use std::ptr;

#[cfg(target_os = "windows")]
fn to_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

#[cfg(target_os = "windows")]
fn from_wide_buffer(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

#[cfg(target_os = "windows")]
pub fn shell_execute(
    file: &str,
    params: Option<&str>,
    directory: Option<&str>,
    verb: Option<&str>,
) -> Result<(), String> {
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::SW_SHOWNORMAL;

    let file_w = to_wide(file);
    let params_w = params.map(to_wide);
    let directory_w = directory.map(to_wide);
    let verb_w = to_wide(verb.unwrap_or("open"));

    let result = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            verb_w.as_ptr(),
            file_w.as_ptr(),
            params_w.as_ref().map_or(ptr::null(), |v| v.as_ptr()),
            directory_w.as_ref().map_or(ptr::null(), |v| v.as_ptr()),
            SW_SHOWNORMAL,
        )
    } as isize;

    if result > 32 {
        Ok(())
    } else {
        Err(format!("ShellExecuteW 失败，错误码: {}", result))
    }
}

#[cfg(target_os = "windows")]
pub fn is_auto_start_enabled(app_name: &str) -> Result<bool, String> {
    use winapi::shared::minwindef::HKEY;
    use winapi::shared::winerror::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
    use winapi::um::winnt::KEY_READ;
    use winapi::um::winreg::{RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER};

    let sub_key = to_wide("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    let value_name = to_wide(app_name);
    let mut key: HKEY = ptr::null_mut();
    let open_result =
        unsafe { RegOpenKeyExW(HKEY_CURRENT_USER, sub_key.as_ptr(), 0, KEY_READ, &mut key) };

    if open_result != ERROR_SUCCESS as i32 {
        return Ok(false);
    }

    let query_result = unsafe {
        RegQueryValueExW(
            key,
            value_name.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };
    unsafe {
        RegCloseKey(key);
    }

    if query_result == ERROR_SUCCESS as i32 {
        Ok(true)
    } else if query_result == ERROR_FILE_NOT_FOUND as i32 {
        Ok(false)
    } else {
        Err(format!("查询开机自启动状态失败，错误码: {}", query_result))
    }
}

#[cfg(target_os = "windows")]
pub fn set_auto_start(app_name: &str, exe_path: &str, enable: bool) -> Result<(), String> {
    use winapi::shared::minwindef::{DWORD, HKEY};
    use winapi::shared::winerror::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
    use winapi::um::winnt::{KEY_WRITE, REG_SZ};
    use winapi::um::winreg::{
        RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegSetValueExW, HKEY_CURRENT_USER,
    };

    let sub_key = to_wide("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    let value_name = to_wide(app_name);
    let mut key: HKEY = ptr::null_mut();
    let create_result = unsafe {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            sub_key.as_ptr(),
            0,
            ptr::null_mut(),
            0,
            KEY_WRITE,
            ptr::null_mut(),
            &mut key,
            ptr::null_mut(),
        )
    };

    if create_result != ERROR_SUCCESS as i32 {
        return Err(format!(
            "打开开机自启动注册表项失败，错误码: {}",
            create_result
        ));
    }

    let result = if enable {
        let value = to_wide(&format!("\"{}\"", exe_path));
        unsafe {
            RegSetValueExW(
                key,
                value_name.as_ptr(),
                0,
                REG_SZ,
                value.as_ptr() as *const u8,
                (value.len() * std::mem::size_of::<u16>()) as DWORD,
            )
        }
    } else {
        unsafe { RegDeleteValueW(key, value_name.as_ptr()) }
    };

    unsafe {
        RegCloseKey(key);
    }

    if result == ERROR_SUCCESS as i32 || (!enable && result == ERROR_FILE_NOT_FOUND as i32) {
        Ok(())
    } else {
        Err(format!("更新开机自启动失败，错误码: {}", result))
    }
}

#[cfg(target_os = "windows")]
fn build_filter(filters: &[(String, Vec<String>)]) -> Vec<u16> {
    let mut filter = Vec::new();
    for (name, extensions) in filters {
        filter.extend(OsStr::new(name).encode_wide());
        filter.push(0);

        let patterns = extensions
            .iter()
            .map(|ext| {
                if ext == "*" || ext == "*.*" {
                    "*.*".to_string()
                } else {
                    format!("*.{}", ext.trim_start_matches("*.").trim_start_matches('.'))
                }
            })
            .collect::<Vec<_>>()
            .join(";");
        filter.extend(OsStr::new(&patterns).encode_wide());
        filter.push(0);
    }
    filter.push(0);
    filter
}

#[cfg(target_os = "windows")]
pub fn open_file_dialog(title: &str, filters: &[(String, Vec<String>)]) -> Result<String, String> {
    use winapi::um::commdlg::{
        GetOpenFileNameW, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST, OPENFILENAMEW,
    };

    let title_w = to_wide(title);
    let filter_w = build_filter(filters);
    let mut file_buffer = vec![0u16; 32768];
    let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
    ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.lpstrFile = file_buffer.as_mut_ptr();
    ofn.nMaxFile = file_buffer.len() as u32;
    ofn.lpstrFilter = filter_w.as_ptr();
    ofn.lpstrTitle = title_w.as_ptr();
    ofn.Flags = OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST;

    let result = unsafe { GetOpenFileNameW(&mut ofn) };
    if result != 0 {
        Ok(from_wide_buffer(&file_buffer))
    } else {
        Err("用户取消了选择".to_string())
    }
}

#[cfg(target_os = "windows")]
pub fn save_file_dialog(
    title: &str,
    filters: &[(String, Vec<String>)],
    default_ext: &str,
    file_name: &str,
) -> Result<String, String> {
    use winapi::um::commdlg::{
        GetSaveFileNameW, OFN_EXPLORER, OFN_OVERWRITEPROMPT, OFN_PATHMUSTEXIST, OPENFILENAMEW,
    };

    let title_w = to_wide(title);
    let filter_w = build_filter(filters);
    let default_ext_w = to_wide(default_ext);
    let mut file_buffer = vec![0u16; 32768];
    let file_name_w = to_wide(file_name);
    for (index, value) in file_name_w.iter().enumerate().take(file_buffer.len()) {
        file_buffer[index] = *value;
    }

    let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
    ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.lpstrFile = file_buffer.as_mut_ptr();
    ofn.nMaxFile = file_buffer.len() as u32;
    ofn.lpstrFilter = filter_w.as_ptr();
    ofn.lpstrDefExt = default_ext_w.as_ptr();
    ofn.lpstrTitle = title_w.as_ptr();
    ofn.Flags = OFN_EXPLORER | OFN_OVERWRITEPROMPT | OFN_PATHMUSTEXIST;

    let result = unsafe { GetSaveFileNameW(&mut ofn) };
    if result != 0 {
        Ok(from_wide_buffer(&file_buffer))
    } else {
        Err("用户取消了选择".to_string())
    }
}

#[cfg(target_os = "windows")]
pub fn open_folder_dialog(title: &str) -> Result<String, String> {
    use windows_sys::Win32::System::Com::CoTaskMemFree;
    use windows_sys::Win32::UI::Shell::{
        SHBrowseForFolderW, SHGetPathFromIDListW, BIF_NEWDIALOGSTYLE, BIF_RETURNONLYFSDIRS,
        BROWSEINFOW,
    };

    let title_w = to_wide(title);
    let mut display_name = vec![0u16; 260];
    let mut browse_info: BROWSEINFOW = unsafe { std::mem::zeroed() };
    browse_info.pszDisplayName = display_name.as_mut_ptr();
    browse_info.lpszTitle = title_w.as_ptr();
    browse_info.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;

    let pidl = unsafe { SHBrowseForFolderW(&mut browse_info) };
    if pidl.is_null() {
        return Err("用户取消了选择".to_string());
    }

    let mut path_buffer = vec![0u16; 260];
    let success = unsafe { SHGetPathFromIDListW(pidl, path_buffer.as_mut_ptr()) };
    unsafe {
        CoTaskMemFree(pidl as *mut _);
    }

    if success != 0 {
        Ok(from_wide_buffer(&path_buffer))
    } else {
        Err("无法获取文件夹路径".to_string())
    }
}
