use rusqlite::{Connection, named_params};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Combatten {
    id: i32,
    name: String,
    campaign_id: i32,
}

pub fn add_combatten(name: &str, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("INSERT INTO combattens (name) VALUES (@name)")?;
  statement.execute(named_params! { "@name": name })?;

  Ok(())
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

pub fn remove_combatten(id: i32, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("DELETE FROM combattens WHERE id = @id")?;
  statement.execute(named_params! { "@id": id })?;

  Ok(())
}