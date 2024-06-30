// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod helper;
use helper::*;
fn main() {
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