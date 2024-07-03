use std::env;
use std::fs;
use std::path::Path;

use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::HKEY;
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::winnt::REG_OPTION_NON_VOLATILE;
use winapi::um::winreg::RegCreateKeyExW;
use winapi::um::winnt::KEY_SET_VALUE;
use winapi::um::winreg::RegOpenCurrentUser;
use winapi::um::winreg::RegOpenKeyExW;
use winapi::um::winreg::RegRenameKey;
use winapi::um::winreg::RegSetValueExW;
use winapi::shared::ntdef::LPWSTR;

fn _mk_regkey(
    root_key: HKEY, 
    key_path: &str, 
    value_name: &str, 
    value_type: u32, 
    value_data: &[u8],
) -> Result<(), String> {


    unsafe {
        let mut hkey: HKEY = ptr::null_mut();
        let result = RegCreateKeyExW(
            root_key, 
            OsStr::new(&key_path).encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            0, 
            ptr::null_mut(), 
            REG_OPTION_NON_VOLATILE, 
            KEY_SET_VALUE, 
            ptr::null_mut(), 
            &mut hkey, 
            ptr::null_mut()
        );

        match result {
            0 => {
                let success = RegSetValueExW(
                    hkey, 
                    OsStr::new(value_name).encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
                    0, 
                    value_type, 
                    value_data.as_ptr() as *const _, // Cast to a const byte pointer
                    value_data.len() as u32 // Convert the length of value_data to u32 for cbData
                );
                if success == 0 {
                    Ok(())
                } else {
                    Err(format!("Failed to set value"))
                }
            },
            err => Err(format!("Failed to open key: {}", err)),
        }
    }
}

fn _regkey_exists(root_key: HKEY, key_path: &str) -> bool {
    let key_name: LPWSTR = key_path.as_ptr() as LPWSTR;
    let mut handle: *mut _ = ptr::null_mut();
    let result = unsafe {
        RegOpenKeyExW(root_key, key_name, 0, 0, &mut handle)
    };

    match result {
        0 => true, // Success, key exists
        2 => false, // Error, key does not exist
        _ => false, // Other errors
    }
}

fn _mv_regkey(old_key_path: &str, new_key_path: &str) -> Result<(), u32> {
    let old_key_name: LPCWSTR = old_key_path.as_ptr() as LPCWSTR;
    let new_key_name: LPCWSTR = new_key_path.as_ptr() as LPCWSTR;
    let old_key_handle: _ = ptr::null_mut::<HKEY>();
    let result = unsafe {
        RegOpenCurrentUser(KEY_SET_VALUE, old_key_handle)
    };
    if result != 0 {
        return Err(result.try_into().unwrap());
    }
    let rename_result = unsafe {
        RegRenameKey(old_key_handle.as_ref().unwrap().clone(), old_key_name, new_key_name)
    };
    if rename_result == 0 {
        Ok(())
    } else {
        Err(rename_result.try_into().unwrap())
    }
}
pub fn mk_regkey(root_key: HKEY,key_path: &str,value_name: &str,value_type: u32,value_data: &[u8]) -> Result<(), String> {
    if _regkey_exists(root_key, key_path) {
        let _ = _mv_regkey(key_path, &format!("{}_bak",key_path));
    }
    _mk_regkey(root_key, key_path, value_name, value_type, value_data)
}

pub fn appdata() -> Option<String> {
    if let Some(appdata_dir) = env::var_os("APPDATA") {
        if let Some(appdata_dir) = appdata_dir.to_str() {
            return Some(appdata_dir.to_string());
        }
    }
    None
}

pub fn make_dummyexe(procname: &str) -> &str {
    if fs::metadata("dummy.exe").is_err() {
        return "not ok";
    }

    fs::copy("dummy.exe", format!("dummies/{}.exe", procname)).expect("Failed to copy file");
    "ok"
}

pub fn get_config() -> String {
    if !Path::new("config.json").exists() {
        save_config("{}")
    }
    fs::read_to_string("config.json").expect("Failed to read file")
}

pub fn save_config(data: &str) {
    fs::write("config.json", data).expect("Unable to write file");
}
pub fn taskkill(proc: &str) {
    #[cfg(windows)]
    {
    std::process::Command::new("taskkill").args(&["/IM", proc, "/F"]).output().ok();
    }
    #[cfg(unix)]
    {
        std::process::Command::new("pkill").arg(proc).output().ok();
    }
}
