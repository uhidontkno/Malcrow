// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod helper;
use std::path::Path;
use std::fs;
use std::process::Command;
use helper::*;
use msgbox::IconType;
use serde_json::Value;
use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};



fn main() {
  if !Path::new("./dummy.exe").exists() {
    let _ = msgbox::create("Fatal Error", "dummy.exe is missing, and Malcrow cannot be used without it, dummy.exe should be in the current working directory. Exiting!", IconType::Error);
    std::process::exit(1);
}
let quit = CustomMenuItem::new("quit".to_string(), "Quit");
let mut hide = CustomMenuItem::new("info".to_string(), "Malcrow");
hide = hide.disabled();

let tray_menu = SystemTrayMenu::new()
  .add_item(quit)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(hide);
let tray = SystemTray::new().with_menu(tray_menu);
tauri::Builder::default()
.system_tray(tray)
.invoke_handler(tauri::generate_handler![_get_config,_save_config,update_procs,kill_procs])
.on_system_tray_event(|app, event| match event {
  SystemTrayEvent::LeftClick {
    position: _,
    size: _,
    ..
  } => {
    println!("system tray received a left click");
    let window = app.get_window("main").unwrap();
    window.show().unwrap();
  }
  SystemTrayEvent::MenuItemClick { id, .. } => {
    match id.as_str() {
      "quit" => {
        std::process::exit(0);
      }
      "hide" => {
      }
      _ => {}
    }
  }
  _ => {}
})
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
    if conf == "{}".to_string() {
      return
    }
    // Ensure the dummy directory exists
    fs::create_dir_all("dummy").unwrap();

    for i in 0..config["proc"].as_array().unwrap().len() {
        let process_name = config["proc"][i].as_str().unwrap();
        let process_path = format!("dummy/{}", process_name);

        if Path::new(&process_path).exists() {
            println!("Killing {}...", process_name);
            taskkill(process_name);
        } else {
            println!("Process {} does not exist, creating it.", process_name);
            match fs::copy("dummy.exe", &process_path) {
                Ok(_) => {},
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
            println!("{} not in conf, removing.", file_name);
            taskkill(file_name);
            fs::remove_file(&path).unwrap();
        }

        if path.is_file() && path.extension().map_or(false, |ext| ext == "exe") {
            println!("Starting {}...", path.display());
            let _ = Command::new(&path).spawn();
        }
    }
}
#[tauri::command]
fn kill_procs() {
    let conf = get_config();
    let config: Value = serde_json::from_str(&conf).unwrap();
    if conf == "{}".to_string() {
      return
    }
    for i in 0..config["proc"].as_array().unwrap().len() {
        let process_name = config["proc"][i].as_str().unwrap();
        println!("Killing {}...", process_name);
        taskkill(process_name);  
    }
}

