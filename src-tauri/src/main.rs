// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn minimize(window: tauri::Window) {
    window.minimize().expect("Failed to minimize window");
}

#[tauri::command]
fn close(window: tauri::Window) {
    window.close().expect("Failed to close window");
}

// use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, minimize, close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
