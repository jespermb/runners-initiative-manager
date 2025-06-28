use crate::database::DbPool;
use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::State;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Combatten {
    pub id: i32,
    pub name: String,
    pub combatten_type: String,
    pub campaign_id: i32,
    pub physical: i32,
    pub stun: i32,
}

#[tauri::command]
pub async fn add_combatten(
    state: State<'_, DbPool>,
    name: &str,
    combatten_type: &str,
    physical: i32,
    stun: i32,
    campaign_id: i32,
) -> Result<Combatten, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("INSERT INTO combattens (name, type, physical, stun, campaign_id) VALUES (@name, @type, @physical, @stun, @campaign_id)")
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! {
            "@name": name,
            "@type": combatten_type,
            "@physical": physical,
            "@stun": stun,
            "@campaign_id": campaign_id
        })
        .map_err(|e| e.to_string())?;
    
    let id: i64 = conn.last_insert_rowid();
    let id_i32: i32 = id.try_into().map_err(|_| "Failed to convert rowid to i32".to_string())?;
    
    // Fetch the newly created combatten
    let combatten = view_combatten_internal(&conn, id_i32)
        .map_err(|e| e.to_string())?;
    
    Ok(combatten)
}

#[tauri::command]
pub async fn get_all_combattens(state: State<'_, DbPool>) -> Result<Vec<Combatten>, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("SELECT id, name, type, campaign_id, physical, stun FROM combattens")
        .map_err(|e| e.to_string())?;
    
    let combattens_iter = statement
        .query_map([], |row| {
            Ok(Combatten {
                id: row.get(0)?,
                name: row.get(1)?,
                combatten_type: row.get(2)?,
                campaign_id: row.get(3)?,
                physical: row.get(4)?,
                stun: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    let combattens = combattens_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(combattens)
}

#[tauri::command]
pub async fn get_all_campaign_combattens(
    state: State<'_, DbPool>,
    campaign_id: i32,
) -> Result<Vec<Combatten>, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("SELECT id, name, type, campaign_id, physical, stun FROM combattens WHERE campaign_id = @campaign_id")
        .map_err(|e| e.to_string())?;
    
    let combattens_iter = statement
        .query_map(named_params! { "@campaign_id": campaign_id }, |row| {
            Ok(Combatten {
                id: row.get(0)?,
                name: row.get(1)?,
                combatten_type: row.get(2)?,
                campaign_id: row.get(3)?,
                physical: row.get(4)?,
                stun: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    let combattens = combattens_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(combattens)
}

// Internal function to reuse in add_combatten
fn view_combatten_internal(conn: &rusqlite::Connection, id: i32) -> Result<Combatten, rusqlite::Error> {
    let mut statement = conn.prepare("SELECT id, name, type, campaign_id, physical, stun FROM combattens WHERE id = @id")?;
    let combatten = statement.query_row(named_params! { "@id": id }, |row| {
        Ok(Combatten {
            id: row.get(0)?,
            name: row.get(1)?,
            combatten_type: row.get(2)?,
            campaign_id: row.get(3)?,
            physical: row.get(4)?,
            stun: row.get(5)?,
        })
    })?;
    Ok(combatten)
}

#[tauri::command]
pub async fn get_combatten(state: State<'_, DbPool>, id: i32) -> Result<Combatten, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    view_combatten_internal(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn edit_combatten(
    state: State<'_, DbPool>,
    id: i32,
    name: &str,
    physical: i32,
    stun: i32,
) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("UPDATE combattens SET name = @name, physical = @physical, stun = @stun WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! { 
            "@id": id, 
            "@name": name,
            "@physical": physical,
            "@stun": stun
        })
        .map_err(|e| e.to_string())?;

    Ok(format!("Combatten {} updated", id))
}

#[tauri::command]
pub async fn remove_combatten(state: State<'_, DbPool>, id: i32) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    
    // First, remove the combatten from all encounters
    let mut encounter_statement = conn
        .prepare("DELETE FROM encounter_combattens WHERE combatten_id = @combatten_id")
        .map_err(|e| e.to_string())?;
    
    encounter_statement
        .execute(named_params! { "@combatten_id": id })
        .map_err(|e| e.to_string())?;
    
    // Then, remove the combatten itself
    let mut combatten_statement = conn
        .prepare("DELETE FROM combattens WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    combatten_statement
        .execute(named_params! { "@id": id })
        .map_err(|e| e.to_string())?;

    Ok(format!("Combatten {} removed from all encounters and deleted", id))
}
