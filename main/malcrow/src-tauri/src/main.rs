// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod helper;
use std::path::Path;
use std::fs;
use std::process::Command;
use helper::*;
use msgbox::IconType;
use serde_json::Value;
fn main() {
  if !Path::new("./dummy.exe").exists() {
    let _ = msgbox::create("Fatal Error", "dummy.exe is missing, and Malcrow cannot be used without it, dummy.exe should be in the current working directory. Exiting!", IconType::Error);
    std::process::exit(1);
}
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![_get_config,_save_config,update_procs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn _get_config() -> String {
  get_config()
}

#[tauri::command]
fn _save_config(data:&str) -> () {
  save_config(data)
}
#[tauri::command]
fn update_procs() -> () {
    let conf = get_config();
    let config: Value = serde_json::from_str(&conf).unwrap();

    // Ensure the dummy directory exists
    fs::create_dir_all("dummy").unwrap();

    for i in 0..config["proc"].as_array().unwrap().len() {
        let process_name = config["proc"][i].as_str().unwrap();
        let process_path = format!("dummy/{}", process_name);

        if Path::new(&process_path).exists() {
            println!("Process {} exists, attempting to kill it.", process_name);
            taskkill(process_name);
        } else {
            println!("Process {} does not exist, creating it.", process_name);
            match fs::copy("dummy.exe", &process_path) {
                Ok(_) => println!("Copied dummy.exe to {}", process_path),
                Err(e) => eprintln!("Failed to copy dummy.exe to {}: {}", process_path, e),
            }
        }
    }
    run_procs(config);
}

fn run_procs(config: Value) {
    let directory = "dummy/";
    fs::create_dir_all(directory).unwrap();

    let entries = fs::read_dir(directory).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if !config["proc"].as_array().unwrap().iter().any(|p| p.as_str().unwrap() == file_name) {
            println!("Process {} is not in the config, attempting to kill and remove it.", file_name);
            taskkill(file_name);
            fs::remove_file(&path).unwrap();
        }

        if path.is_file() && path.extension().map_or(false, |ext| ext == "exe") {
            println!("Starting process: {}", path.display());
            let _ = Command::new(&path).spawn();
        }
    }
}