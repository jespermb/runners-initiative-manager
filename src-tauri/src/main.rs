// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn add_combatten(app_handle: AppHandle, name: &str) -> String {
    app_handle.db(|db|  database::add_combatten(name, db)).unwrap();

    format!("{} added", name)
}

#[tauri::command]
fn get_all_items(app_handle: AppHandle) -> Vec<String> {
    let items = app_handle.db(|db|  database::get_all_combattens(db)).unwrap();
    items
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![add_combatten, get_all_items])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
