// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use reader::{Config, Reader, Type};
use serde_json::{Map, Value};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open, get_config, get_columns])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open(
    config_path: String,
    file_path: String,
    _type: Type,
    from: Option<usize>,
    len: Option<usize>,
) -> Result<Vec<Map<String, Value>>, String> {
    let reader = Reader::new(config_path, file_path, _type).map_err(|e| e.to_string())?;

    match reader.read(from, len) {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_config(config_path: String) -> Result<Value, String> {
    let config = fs::read_to_string(config_path).map_err(|e| e.to_string())?;

    let data = serde_json::from_str(&config).map_err(|e| e.to_string())?;

    Ok(data)
}

#[tauri::command]
async fn get_columns(config_path: String, _type: Type) -> Result<Map<String, Value>, String> {
    let config = fs::read_to_string(config_path).map_err(|e| e.to_string()).unwrap();
    let config: Config = serde_json::from_str(&config).map_err(|e| e.to_string()).unwrap();

    Ok(Reader::get_columns(config, _type))
}