use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/types/")]
pub struct Campaign {
    id: i32,
    name: String,
    version: i32,
}

pub fn add_campaign(db: &Connection, name: &str, version: i32) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO campaigns (name, version) VALUES (@name, @version)")?;
    statement.execute(named_params! { "@name": name, "@version": version })?;

    Ok(())
}

pub fn get_all_campaigns(db: &Connection) -> Result<Vec<Campaign>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM campaigns")?;
    let campaign_iter = statement.query_map([], |row| {
      Ok(Campaign {
        id: row.get(0)?,
        name: row.get(1)?,
        version: row.get(2)?,
      })
    }).unwrap();
    let campaigns = campaign_iter.collect::<Result<Vec<_>, _>>().unwrap();
    Ok(campaigns)
}

pub fn view_campaign(db: &Connection, id: i32) -> Result<Campaign, rusqlite::Error> {
  let mut statement = db.prepare("SELECT id, name, version FROM campaigns WHERE id = @id")?;
  let campaign = statement.query_row(named_params! { "@id": id }, |row| {
    Ok(Campaign {
      id: row.get(0)?,
      name: row.get(1)?,
      version: row.get(2)?,
    })
  }).unwrap();
  Ok(campaign)
}

pub fn remove_campaign(db: &Connection, id: i32) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("DELETE FROM campaigns WHERE id = @id")?;
  statement.execute(named_params! { "@id": id })?;

  Ok(print!("Campaign {} removed.", id))
}

