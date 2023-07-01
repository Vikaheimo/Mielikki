// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mielikki::{CurrentDir, CurrentDirError, FolderData};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct OuterCurrentDir(pub Mutex<CurrentDir>);

#[tauri::command]
fn get_current_folder(state: tauri::State<OuterCurrentDir>) -> Result<FolderData, CurrentDirError> {
    let state_guard = state.0.lock().unwrap();

    state_guard.get_folder_data()
}

#[tauri::command]
fn move_to_folder(
    state: tauri::State<OuterCurrentDir>,
    folder_path: String,
) -> Result<(), CurrentDirError> {
    let mut state_guard = state.0.lock().unwrap();

    state_guard.move_to_dir(&PathBuf::from(folder_path))
}

#[tauri::command]
fn move_to_parent_folder(state: tauri::State<OuterCurrentDir>) -> Result<(), CurrentDirError> {
    let mut state_guard = state.0.lock().unwrap();

    state_guard.move_to_parent_dir()
}

fn main() {
    tauri::Builder::default()
        .manage(OuterCurrentDir(Mutex::new(CurrentDir::new(Path::new(".")))))
        .invoke_handler(tauri::generate_handler![
            get_current_folder,
            move_to_folder,
            move_to_parent_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
