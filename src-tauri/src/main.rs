// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod database;

use actions::campaign;
use actions::combatten;
use actions::encounter;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            
            // Initialize database pool
            let db_pool = database::initialize_database(&handle)
                .expect("Database initialization should succeed");
            
            // Manage the database pool
            app.manage(db_pool);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Campaign commands
            campaign::add_campaign,
            campaign::get_all_campaigns,
            campaign::get_campaign,
            campaign::remove_campaign,
            
            // Combatten commands
            combatten::add_combatten,
            combatten::get_all_combattens,
            combatten::get_all_campaign_combattens,
            combatten::get_combatten,
            combatten::edit_combatten,
            combatten::remove_combatten,
            
            // Encounter commands
            encounter::add_encounter,
            encounter::get_all_encounters,
            encounter::get_encounter,
            encounter::remove_encounter,
            encounter::add_combatten_to_encounter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
