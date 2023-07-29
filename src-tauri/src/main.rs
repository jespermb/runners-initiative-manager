// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod actions;

use actions::campaign::Campaign;
use actions::combatten::Combatten;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

use actions::campaign;
use actions::combatten;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Combatten
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
fn get_combatten(app_handle: AppHandle, id: i32) -> Combatten {
    let combatten = app_handle.db(|db|  combatten::view_combatten(db, id)).unwrap();
    combatten
}
#[tauri::command]
fn remove_combatten(app_handle: AppHandle, id: i32) -> String {
    app_handle.db(|db|  combatten::remove_combatten(id, db)).unwrap();

    format!("{} removed", id)
}

// Campaign
#[tauri::command]
fn add_campaign(app_handle: AppHandle, name: &str, version: i32) -> String {
    app_handle.db(|db|  campaign::add_campaign(db, name, version)).unwrap();

    format!("{} added", name)
}
#[tauri::command]
fn get_all_campaigns(app_handle: AppHandle) -> Vec<campaign::Campaign> {
    let items = app_handle.db(|db|  campaign::get_all_campaigns(db)).unwrap();
    items
}
#[tauri::command]
fn get_campaign(app_handle: AppHandle, id: i32) -> Campaign {
    let campaign = app_handle.db(|db|  campaign::view_campaign(db, id)).unwrap();
    campaign
}
#[tauri::command]
fn remove_campaign(app_handle: AppHandle, id: i32) -> () {
    let result = app_handle.db(|db|  campaign::remove_campaign(db, id)).unwrap();
    result
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![
            add_campaign,
            get_all_campaigns,
            get_campaign,
            remove_campaign,
            add_combatten, 
            get_all_combattens,
            get_combatten,
            remove_combatten
        ])
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
