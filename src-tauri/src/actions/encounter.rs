use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::combatten::{Combatten, self};

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Encounter {
    id: i32,
    name: String,
    campaign_id: i32,
     combattens: Vec<Combatten>
}

pub fn add_encounter(db: &Connection, name: &str, campaign_id: i32) -> Result<(), rusqlite::Error> {
    let mut statement =
        db.prepare("INSERT INTO encounters (name, campaign_id) VALUES (@name, @campaignId)")?;
    statement.execute(named_params! { "@name": name, "@campaignId": campaign_id })?;

    Ok(())
}

pub fn get_all_encounters(
    db: &Connection,
    campaign_id: i32,
) -> Result<Vec<Encounter>, rusqlite::Error> {
    let mut statement = db
        .prepare("SELECT id, name, campaign_id FROM encounters WHERE campaign_id = @campaign_id")?;
    let encounters_iter = statement
        .query_map(named_params! {"@campaign_id": campaign_id}, |row| {
            Ok(Encounter {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
                combattens: Vec::new()
            })
        })
        .unwrap();
    let encounters = encounters_iter.collect::<Result<Vec<_>, _>>().unwrap();
    Ok(encounters)
}

pub fn get_all_encounters_by_campaign(
    db: &Connection,
    campaign_id: i32,
) -> Result<Vec<Encounter>, rusqlite::Error> {
    let mut statement = db
        .prepare("SELECT id, name, campaign_id FROM encounters WHERE campaign_id = @campaign_id")?;
    let encounters_iter = statement
        .query_map(named_params! { "@campaign_id": campaign_id }, |row| {
            Ok(Encounter {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
                combattens: Vec::new()
            })
        })
        .unwrap();
    let encounters = encounters_iter.collect::<Result<Vec<_>, _>>().unwrap();
    Ok(encounters)
}

pub fn view_encounter(db: &Connection, id: i32) -> Result<Encounter, rusqlite::Error> {
    let mut statement =
        db.prepare("SELECT id, name, campaign_id FROM encounters WHERE id = @id")?;
    let mut encounter = statement
        .query_row(named_params! { "@id": id }, |row| {
            Ok(Encounter {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
                combattens: Vec::new()
            })
        })
        .unwrap();
    let mut combStatement = 
        db.prepare("SELECT c.id, c.name, c.campaign_id FROM combatten_encounter ce LEFT JOIN combatten c ON (c.id = ce.combatten_id) WHERE ce.encounter_id = @id")?;
    let combattens_iter = combStatement
        .query_map(named_params! { "@id": id }, |row| {
            Ok(Combatten {
                id: row.get(0)?,
                name: row.get(1)?,
                campaign_id: row.get(2)?,
            })
        })
        .unwrap();
    let combattens = combattens_iter.collect::<Result<Vec<_>, _>>().unwrap();
    encounter.combattens = combattens;
    Ok(encounter)
}

pub fn remove_encounter(id: i32, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM encounters WHERE id = @id")?;
    statement.execute(named_params! { "@id": id })?;

    Ok(())
}
