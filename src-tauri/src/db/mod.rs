pub mod notes;

use std::path::PathBuf;
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use tauri::{AppHandle, Manager};

pub type DbHandle = Surreal<Db>;

pub fn resolve_db_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|d| d.join("graphnotes.db"))
        .map_err(|e| e.to_string())
}

pub async fn init_db(app: &AppHandle) -> Result<DbHandle, String> {
    let db_dir = resolve_db_dir(app)?;
    std::fs::create_dir_all(&db_dir).map_err(|e| e.to_string())?;

    let db = Surreal::new::<SurrealKv>(db_dir)
        .await
        .map_err(|e| e.to_string())?;

    db.use_ns("graphnotes")
        .use_db("vault")
        .await
        .map_err(|e| e.to_string())?;

    run_migrations(&db).await?;
    Ok(db)
}

pub async fn run_migrations(db: &DbHandle) -> Result<(), String> {
    db.query(include_str!("schema.surql"))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
