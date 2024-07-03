use std::env;
use std::fs;
use std::path::Path;

use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::HKEY;
use winapi::um::winnt::REG_OPTION_NON_VOLATILE;
use winapi::um::winreg::RegCreateKeyExW;
use winapi::um::winnt::KEY_SET_VALUE;
use winapi::um::winreg::RegSetValueExW;

pub fn create_registry_key(
    root_key: HKEY, 
    root_key_path: &str, 
    subkey_name: &str, 
    value_name: &str, 
    value_type: u32, 
    value_data: &[u8],
) -> Result<(), String> {
    let full_path = format!("{}\\{}", root_key_path, subkey_name);

    unsafe {
        let mut hkey: HKEY = ptr::null_mut();
        let result = RegCreateKeyExW(
            root_key, 
            OsStr::new(&full_path).encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
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
