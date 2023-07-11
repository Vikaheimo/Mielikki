// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mielikki::FileData;
use mielikki::{CurrentDir, CurrentDirError, FolderData};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct OuterCurrentDir(pub Arc<Mutex<CurrentDir>>);

#[tauri::command]
async fn get_current_folder(
    state: tauri::State<'_, OuterCurrentDir>,
) -> Result<FolderData, CurrentDirError> {
    let state_guard = state.0.lock().await;

    state_guard.get_folder_data()
}

#[tauri::command]
async fn move_to_folder(
    state: tauri::State<'_, OuterCurrentDir>,
    folder_path: String,
    to_parent: bool,
) -> Result<(), CurrentDirError> {
    let mut state_guard = state.0.lock().await;

    state_guard.move_to_dir(&PathBuf::from(folder_path), to_parent)
}

#[tauri::command]
async fn move_to_parent_folder(
    state: tauri::State<'_, OuterCurrentDir>,
) -> Result<String, CurrentDirError> {
    let mut state_guard = state.0.lock().await;

    state_guard.move_to_parent_dir()
}

#[tauri::command]
async fn current_dir_is_root(state: tauri::State<'_, OuterCurrentDir>) -> Result<bool, ()> {
    let state_guard = state.0.lock().await;
    Ok(state_guard.current_dir_is_root())
}

#[tauri::command]
async fn find_file(
    state: tauri::State<'_, OuterCurrentDir>,
    name: String,
    files: bool,
    folders: bool,
    links: bool,
    exact: bool,
) -> Result<Vec<FileData>, CurrentDirError> {
    let state_guard = state.0.lock().await;
    state_guard
        .search_files(name, files, folders, links, exact)
        .await
}

#[tauri::command]
async fn create_file(
    state: tauri::State<'_, OuterCurrentDir>,
    filename: String,
    filetype: String,
) -> Result<(), CurrentDirError> {
    let state_guard = state.0.lock().await;
    state_guard.create_file(filename, filetype).await
}

#[tauri::command]
async fn delete_file(
    state: tauri::State<'_, OuterCurrentDir>,
    filename: String,
    filetype: String,
) -> Result<(), CurrentDirError> {
    let state_guard = state.0.lock().await;
    state_guard.delete_file(filename, filetype).await
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(OuterCurrentDir(Arc::new(Mutex::new(
            CurrentDir::new(Path::new(".")).await,
        ))))
        .invoke_handler(tauri::generate_handler![
            get_current_folder,
            move_to_folder,
            move_to_parent_folder,
            current_dir_is_root,
            find_file,
            create_file,
            delete_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
