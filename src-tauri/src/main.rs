// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::encode;
// use global_hotkey::{
//     hotkey::{Code, HotKey, Modifiers},
//     GlobalHotKeyEvent, GlobalHotKeyManager,
// };
use log::{error, info};
use tauri::{
    api::{file, path},
    App, GlobalShortcutManager, Manager,
};

use std::time::Instant;
use xcap::Monitor;
extern crate base64;
extern crate image;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

#[tauri::command]
fn start_capture() -> Vec<String> {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    let mut res: Vec<String> = vec![];

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();
    
    }

    println!("运行耗时: {:?}", start.elapsed());
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
