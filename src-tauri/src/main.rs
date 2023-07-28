// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod actions;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

use actions::campaign;
use actions::combatten;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn add_combatten(app_handle: AppHandle, name: &str) -> String {
    app_handle.db(|db|  combatten::add_combatten(name, db)).unwrap();

    format!("{} added", name)
}

#[tauri::command]
fn get_all_combattens(app_handle: AppHandle) -> Vec<combatten::Combatten> {
    let items = app_handle.db(|db|  combatten::get_all_combattens(db)).unwrap();
    items
}

#[tauri::command]
fn remove_combatten(app_handle: AppHandle, id: u32) -> String {
    app_handle.db(|db|  combatten::remove_combatten(id, db)).unwrap();

    format!("{} removed", id)
}

#[tauri::command]
fn get_all_campaigns(app_handle: AppHandle) -> Vec<String> {
    let campaigns = app_handle.db(|db| campaign::get_all_campaigns(db)).unwrap();
    campaigns
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![add_combatten, get_all_combattens, remove_combatten])
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
