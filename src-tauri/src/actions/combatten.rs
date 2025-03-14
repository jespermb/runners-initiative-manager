use rusqlite::{Connection, named_params};
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Combatten {
    pub id: i32,
    pub name: String,
    pub campaign_id: i32,
}

pub fn add_combatten(db: &Connection, name: &str, physical: i32, stun: i32, campaign_id: i32) -> Result<i32, rusqlite::Error> {
  let mut statement = db.prepare("INSERT INTO combattens (name, physical, stun, campaign_id) VALUES (@name, @physical, @stun, @campaign_id)")?;
  statement.execute(named_params! { "@name": name, "@physical": physical, "@stun": stun, "@campaign_id": campaign_id })?;
  let id: i64 = db.last_insert_rowid();
  Ok(id.try_into().unwrap())
}

pub fn get_all_combattens(db: &Connection) -> Result<Vec<Combatten>, rusqlite::Error> {
  let mut statement = db.prepare("SELECT id, name, campaign_id FROM combattens")?;
  let combattens_iter = statement.query_map([], |row| {
    Ok(Combatten {
      id: row.get(0)?,
      name: row.get(1)?,
      campaign_id: row.get(2)?,
    })
  }).unwrap();
  let combattens = combattens_iter.collect::<Result<Vec<_>, _>>().unwrap();
  Ok(combattens)
}

pub fn get_all_campaign_combattens(db: &Connection, campaign_id: i32) -> Result<Vec<Combatten>, rusqlite::Error> {
  let mut statement = db.prepare("SELECT id, name, campaign_id FROM combattens WHERE campaign_id = @campaign_id")?;
  let combattens_iter = statement.query_map(named_params! { "@campaign_id": campaign_id }, |row| {
    Ok(Combatten {
      id: row.get(0)?,
      name: row.get(1)?,
      campaign_id: row.get(2)?,
    })
  }).unwrap();
  let combattens = combattens_iter.collect::<Result<Vec<_>, _>>().unwrap();
  Ok(combattens)
}

pub fn view_combatten(db: &Connection, id: i32) -> Result<Combatten, rusqlite::Error> {
  let mut statement = db.prepare("SELECT id, name, campaign_id FROM combattens WHERE id = @id")?;
  let campaign = statement.query_row(named_params! { "@id": id }, |row| {
    Ok(Combatten {
      id: row.get(0)?,
      name: row.get(1)?,
      campaign_id: row.get(2)?,
    })
  }).unwrap();
  Ok(campaign)
}

pub fn edit_combatten(id: i32, name: &str, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("UPDATE combattens SET name = @name WHERE id = @id")?;
  statement.execute(named_params! { "@id": id, "@name": name })?;

  Ok(())
}

pub fn remove_combatten(id: i32, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("DELETE FROM combattens WHERE id = @id")?;
  statement.execute(named_params! { "@id": id })?;

  Ok(())
}