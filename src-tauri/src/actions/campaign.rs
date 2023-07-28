use rusqlite::{Connection, named_params};

pub fn add_campaign(name: &str, db: &Connection) -> Result<(), rusqlite::Error> {
  let mut statement = db.prepare("INSERT INTO campaigns (name) VALUES (@name)")?;
  statement.execute(named_params! { "@name": name })?;

  Ok(())
}

pub fn get_all_campaigns(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
  let mut statement = db.prepare("SELECT * FROM campaigns")?;
  let mut rows = statement.query([])?;
  let mut items = Vec::new();
  while let Some(row) = rows.next()? {
    let name: String = row.get("name")?;

    items.push(name);
  }

  Ok(items)
}