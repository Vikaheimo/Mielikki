// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use mielikki::{CurrentDir, CurrentDirError, FolderData};

pub struct OuterCurrentDir(pub Mutex<CurrentDir>);

#[tauri::command]
fn get_current_folder(state: tauri::State<OuterCurrentDir>) -> Result<FolderData, CurrentDirError> {
    let state_guard = state.0.lock().unwrap();

    Ok(state_guard.get_folder_data()?)
}

fn main() {
    tauri::Builder::default()
        .manage(OuterCurrentDir(Mutex::new(CurrentDir::new(Path::new(".")))))
        .invoke_handler(tauri::generate_handler![get_current_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
