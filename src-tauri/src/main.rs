// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs};

use tauri::api::path::home_dir;

#[tauri::command]
fn get_sources() -> Vec<String> {
    let user_dir = home_dir().unwrap().into_os_string();
    
    match fs::read_to_string(format!("{}/.hop-sources", user_dir.to_str().unwrap())) {
        Ok(file) => file.lines().map(|s| s.to_string()).collect(),
        Err(_error) => [].to_vec()
    }
}

#[tauri::command]
fn update_source(source_name: String, source_path: String) {
    let user_dir = home_dir().unwrap().into_os_string();
    

    match fs::read_to_string(format!("{}/.hop-sources", user_dir.to_str().unwrap())) {
        Ok(file) => {
            fs::write(
                format!("{}/.hop-sources", user_dir.to_str().unwrap()),
                format!("{}\n{} {}", file, source_name, source_path)
            ).unwrap()

        },
        Err(_error) => {
            match fs::write(
                format!("{}/.hop-sources", user_dir.to_str().unwrap()),
                format!("{} {}", source_name, source_path)
            ) {

                Ok(_file) => print!("File was created"),
                Err(error) => panic!("Problem creating the file: {:?}", error),
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_source, get_sources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
