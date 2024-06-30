// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod helper;
use std::path::Path;

use helper::*;
use msgbox::IconType;
use serde_json::Value;
fn main() {
  if !Path::new("./dummy.exe").exists() {
    let _ = msgbox::create("Fatal Error", "dummy.exe is missing, and Malcrow cannot be used without it, dummy.exe should be in the current working directory. Exiting!", IconType::Error);
    std::process::exit(1);
}
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![_get_config,_save_config])
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
  for i in 0..config["proc"].as_array().unwrap().len() {
    if Path::new(&format!("dummy/{}",config["proc"][i])).exists() {
      
    }
  }
}