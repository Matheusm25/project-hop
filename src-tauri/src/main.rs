// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_sources() -> Vec<String> {
    let paths: Vec<String> = fs::read_to_string("/home/matheus/github/project-hop/src-tauri/src/sources").unwrap().lines().map(|s| s.to_string()).collect();
    paths
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_sources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
