use crate::database::DbPool;
use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::State;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Campaign {
    pub id: i32,
    pub name: String,
    pub version: i32,
}

#[tauri::command]
pub async fn add_campaign(state: State<'_, DbPool>, name: &str, version: i32) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("INSERT INTO campaigns (name, version) VALUES (@name, @version)")
        .map_err(|e| e.to_string())?;
    statement
        .execute(named_params! { "@name": name, "@version": version })
        .map_err(|e| e.to_string())?;

    Ok(format!("{} added", name))
}

#[tauri::command]
pub async fn get_all_campaigns(state: State<'_, DbPool>) -> Result<Vec<Campaign>, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn.prepare("SELECT * FROM campaigns").map_err(|e| e.to_string())?;
    let campaign_iter = statement
        .query_map([], |row| {
            Ok(Campaign {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    let campaigns = campaign_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(campaigns)
}

#[tauri::command]
pub async fn get_campaign(state: State<'_, DbPool>, id: i32) -> Result<Campaign, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("SELECT id, name, version FROM campaigns WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    let campaign = statement
        .query_row(named_params! { "@id": id }, |row| {
            Ok(Campaign {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    Ok(campaign)
}

#[tauri::command]
pub async fn remove_campaign(state: State<'_, DbPool>, id: i32) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("DELETE FROM campaigns WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! { "@id": id })
        .map_err(|e| e.to_string())?;

    Ok(format!("Campaign {} removed", id))
}
