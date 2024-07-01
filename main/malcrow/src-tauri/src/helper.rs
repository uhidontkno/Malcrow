use std::env;
use std::fs;
use std::path::Path;

use winreg::RegKey;
use winreg::HKEY;
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

pub fn make_regkey(root_key: HKEY, key_path: &str, value_name: &str){
    let key_parts: Vec<&str> = key_path.split('\\').collect();
    let root = RegKey::predef(root_key);
    let mut current_key = root;
    for part in &key_parts {
        let result = current_key.create_subkey(part);
        match result {
            Ok((subkey, _)) => {
                current_key = subkey;
            }
            Err(_) => {
                return;
            }
        }
    }
    if let Ok(existing_value) = current_key.get_value::<u32, _>(value_name) {
        let backup_value_name = format!("{}_bak", value_name);
        let _ = current_key.set_value(&backup_value_name, &existing_value);
    }
    let _ = current_key.set_value(value_name, &679004u32);
}
