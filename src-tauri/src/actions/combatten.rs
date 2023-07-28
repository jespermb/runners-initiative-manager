use rusqlite::{Connection, named_params};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Combatten {
    id: i32,
    name: String,
}

pub fn add_combatten(name: &str, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("INSERT INTO combattens (name) VALUES (@name)")?;
  statement.execute(named_params! { "@name": name })?;

  Ok(())
}

pub fn get_all_combattens(db: &Connection) -> Result<Vec<Combatten>, rusqlite::Error> {
  let mut statement = db.prepare("SELECT * FROM combattens")?;
  let combattens_iter = statement.query_map([], |row| {
    Ok(Combatten {
      id: row.get(0)?,
      name: row.get(1)?,
    })
  }).unwrap();
  let combattens = combattens_iter.collect::<Result<Vec<_>, _>>().unwrap();
  Ok(combattens)
}

pub fn remove_combatten(id: u32, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("DELETE FROM combattens WHERE id = @id")?;
  statement.execute(named_params! { "@id": id })?;

  Ok(())
}