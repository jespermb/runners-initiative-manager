use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::fs;
use tauri::{AppHandle, Manager};

pub type DbPool = Pool<SqliteConnectionManager>;

const CURRENT_DB_VERSION: u32 = 2;

/// Initializes the database connection pool, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<DbPool, Box<dyn std::error::Error>> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("initiative.sqlite");

    // Create connection manager
    let manager = SqliteConnectionManager::file(sqlite_path);
    
    // Create pool with default configuration
    let pool = Pool::new(manager)?;
    
    // Get a connection to initialize the database
    let mut conn = pool.get()?;
    
    let mut user_pragma = conn.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut conn, existing_user_version)?;
    
    // Configure pool connections to use WAL journal mode
    let conn = pool.get()?;
    conn.pragma_update(None, "journal_mode", "WAL")?;

    Ok(pool)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
  if existing_version < 1 {
    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

    tx.execute_batch(
      "
      CREATE TABLE campaigns (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        version INTEGER NOT NULL
      );
      CREATE TABLE encounters (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        campaign_id INTEGER NOT NULL
      );
      CREATE TABLE combattens (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        physical INTEGER NOT NULL,
        stun INTEGER NOT NULL,
        campaign_id INTEGER NOT NULL
      );
      "
    )?;

    tx.commit()?;
  }

  if existing_version < 2 {
    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

    tx.execute_batch(
      "
      CREATE TABLE encounter_combattens (
        id INTEGER PRIMARY KEY,
        encounter_id INTEGER NOT NULL,
        combatten_id INTEGER NOT NULL,
        damange INTEGER
        initiative INTEGER
      );
      "
    )?;

    tx.commit()?;
  }

  Ok(())
}
