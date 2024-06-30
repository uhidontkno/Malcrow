use std::env;
use std::fs;
use std::path::Path;
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