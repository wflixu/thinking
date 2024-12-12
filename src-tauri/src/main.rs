// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// extern crate base64;
// extern crate image;

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// fn main() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_notification::init())
//         .plugin(tauri_plugin_os::init())
//         .plugin(tauri_plugin_clipboard_manager::init())
//         .plugin(tauri_plugin_http::init())
//         .plugin(tauri_plugin_shell::init())
//         .plugin(tauri_plugin_dialog::init())
//         .plugin(tauri_plugin_fs::init())
//         .plugin(tauri_plugin_updater::Builder::new().build())
//         .plugin(tauri_plugin_process::init())
//         .plugin(tauri_plugin_global_shortcut::Builder::new().build())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }




fn main() {
    thinking_lib::run()
}
