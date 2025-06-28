use crate::database::DbPool;
use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::State;
use ts_rs::TS;

use super::combatten::Combatten;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct EncounterCombatten {
    pub id: i32,
    pub name: String,
    pub combatten_type: String,
    pub campaign_id: i32,
    pub initiative: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Encounter {
    pub id: i32,
    pub name: String,
    pub campaign_id: i32,
    pub combattens: Vec<EncounterCombatten>,
}

#[tauri::command]
pub async fn add_encounter(
    state: State<'_, DbPool>,
    name: &str,
    campaign_id: i32,
) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("INSERT INTO encounters (name, campaign_id) VALUES (@name, @campaignId)")
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! { "@name": name, "@campaignId": campaign_id })
        .map_err(|e| e.to_string())?;

    Ok(format!("Encounter {} added", name))
}

#[tauri::command]
pub async fn get_all_encounters(
    state: State<'_, DbPool>,
    campaign_id: i32,
) -> Result<Vec<Encounter>, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("SELECT id, name, campaign_id FROM encounters WHERE campaign_id = @campaign_id")
        .map_err(|e| e.to_string())?;
    
    let encounters_iter = statement
        .query_map(named_params! {"@campaign_id": campaign_id}, |row| {
            Ok(Encounter {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
                combattens: Vec::new(),
            })
        })
        .map_err(|e| e.to_string())?;
    
    let encounters = encounters_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(encounters)
}

#[tauri::command]
pub async fn get_encounter(state: State<'_, DbPool>, id: i32) -> Result<Encounter, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    
    // Get encounter details
    let mut statement = conn
        .prepare("SELECT id, name, campaign_id FROM encounters WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    let mut encounter = statement
        .query_row(named_params! { "@id": id }, |row| {
            Ok(Encounter {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
                combattens: Vec::new(),
            })
        })
        .map_err(|e| e.to_string())?;
    
    // Get associated combattens ordered by initiative (desc)
    let mut comb_statement = conn
        .prepare(
            "SELECT c.id, c.name, c.type, c.campaign_id, ce.initiative 
            FROM encounter_combattens ce 
            LEFT JOIN combattens c ON (c.id = ce.combatten_id) 
            WHERE ce.encounter_id = @id
            ORDER BY ce.initiative DESC"
        )
        .map_err(|e| e.to_string())?;
    
    let combattens_iter = comb_statement
        .query_map(named_params! { "@id": id }, |row| {
            Ok(EncounterCombatten {
                id: row.get(0)?,
                name: row.get(1)?,
                combatten_type: row.get(2)?,
                campaign_id: row.get(3)?,
                initiative: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    let combattens = combattens_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    encounter.combattens = combattens;
    Ok(encounter)
}

#[tauri::command]
pub async fn remove_encounter(state: State<'_, DbPool>, id: i32) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("DELETE FROM encounters WHERE id = @id")
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! { "@id": id })
        .map_err(|e| e.to_string())?;

    Ok(format!("Encounter {} removed", id))
}

#[tauri::command]
pub async fn add_combatten_to_encounter(
    state: State<'_, DbPool>,
    encounter_id: i32,
    combatten_id: i32,
    initiative: i32,
) -> Result<String, String> {
    let conn = state.get().map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare(
            "INSERT INTO encounter_combattens (encounter_id, combatten_id, initiative) 
             VALUES (@encounter_id, @combatten_id, @initiative)"
        )
        .map_err(|e| e.to_string())?;
    
    statement
        .execute(named_params! {
            "@encounter_id": encounter_id,
            "@combatten_id": combatten_id,
            "@initiative": initiative
        })
        .map_err(|e| e.to_string())?;

    Ok(format!("Combatten {} added to encounter {}", combatten_id, encounter_id))
}
