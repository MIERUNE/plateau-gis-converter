// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::map::Keys;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, mierune])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn mierune() -> String {
    format!("MIERUNE!")
}
