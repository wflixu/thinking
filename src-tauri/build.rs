use std::env;

fn main() {
    // Set environment variables here
    env::set_var("TAURI_SIGNING_PRIVATE_KEY", "/Users/lixu/.tauri/thinking.key");
    env::set_var("TAURI_SIGNING_PRIVATE_KEY_PASSWORD", "");
    
    tauri_build::build()
}
