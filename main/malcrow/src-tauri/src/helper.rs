use std::env;
use std::fs;
use std::path::Path;
use serde_json::{json, Value};
use std::fs::File;
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

pub fn get_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.json");
    if !config_path.exists() {
        let mut file = File::create(config_path)?;
        file.write_all(b"{}")?;
    }
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents
}
pub fn save_config(data:&str) {
    let config_path = Path::new("config.json");
    let mut file = File::create(config_path).unwrap();
    file.write_all(data).unwrap();
}