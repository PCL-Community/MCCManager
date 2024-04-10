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

use std::time::Duration;
use tokio::time::sleep;

#[tauri::command]
async fn enlargewindow(window: tauri::Window) {
    let size = window.inner_size().expect("Failed to get window size");
    let start_width = size.width;
    let start_height = size.height;
    let target_width = size.width * 3;
    let target_height = size.height;
    let duration = Duration::from_millis(50);
    let steps = 50;
    for i in 1..=steps {
        let current_width = start_width + ((target_width - start_width) * i / steps);
        let current_height = start_height + ((target_height - start_height) * i / steps);
        window
            .set_size(
                tauri::Size::Physical(tauri::PhysicalSize {
                    width: current_width,
                    height: current_height,
                })
            )
            .expect("Failed to set window size");
        sleep(duration / steps).await;
    }
}

#[tauri::command]
async fn shrinkwindow(window: tauri::Window) {
    let size = window.inner_size().expect("Failed to get window size");
    let start_width = size.width;
    let start_height = size.height;
    let target_width = size.width / 3;
    let target_height = size.height;
    let duration = Duration::from_millis(100);
    let steps = 50;
    for i in 1..=steps {
        let current_width = start_width - ((start_width - target_width) * i / steps);
        let current_height = start_height - ((start_height - target_height) * i / steps);
        window
            .set_size(
                tauri::Size::Physical(tauri::PhysicalSize {
                    width: current_width,
                    height: current_height,
                })
            )
            .expect("Failed to set window size");
        sleep(duration / steps).await;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, minimize, close, enlargewindow, shrinkwindow])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
