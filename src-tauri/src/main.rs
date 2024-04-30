// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
async fn enlarge_window(window: tauri::Window) {
    let size = window.inner_size().expect("Failed to get window size");
    let start_width = size.width;
    let start_height = size.height;
    let target_width = 1689;
    let target_height = size.height;
    let duration = Duration::from_millis(100);
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
async fn shrink_window(window: tauri::Window) {
    let size = window.inner_size().expect("Failed to get window size");
    let start_width = size.width;
    let start_height = size.height;
    let target_width = 563;
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

#[tauri::command]
fn openurl(url: &str) {
    webbrowser::open(url).expect("Failed to open URL");
}

use std::fs;
use serde_json::Value;

#[tauri::command]
fn read_config_file(filename: &str) -> Result<String, String> {
    match fs::read_to_string(filename) {
        Ok(content) => Ok(content),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn update_config_file(filename: &str, key: &str, value: Value) -> Result<(), String> {
    let config = match read_config_file(filename) {
        Ok(content) => content,
        Err(err) => return Err(format!("Failed to read config file: {}", err)),
    };
    let mut config_map: serde_json::Map<String, Value> =
        serde_json::from_str(&config).map_err(|err| format!("Failed to parse config file: {}", err))?;
    config_map.insert(key.to_string(), value);
    let new_config = serde_json::Value::Object(config_map);
    let new_config_string = serde_json::to_string_pretty(&new_config)
        .map_err(|err| format!("Failed to serialize config file: {}", err))?;
    fs::write(filename, new_config_string)
        .map_err(|err| format!("Failed to write config file: {}", err))?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            minimize, 
            close, 
            enlarge_window, 
            shrink_window, 
            openurl, 
            read_config_file, 
            update_config_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
