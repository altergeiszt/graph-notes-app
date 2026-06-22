
use std::path::PathBuf;
use surrealdb::engine::local::SurrealKv;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::{AppHandle, Manager};

/// Convenient alias for the local SurrealDB handle used by the app.
pub type DbHandle = Surreal<Db>;

/// Resolves the folder under app data where the local database files live.
///
/// Fill in the path layout you want for your vault metadata, for example:
/// `app_data_dir()/graph-notes-app/db`.
pub fn resolve_db_dir(app: &AppHandle) -> Result<PathBuf, String> {
	let app_data_dir = app
		.path()
		.app_data_dir()
		.map_err(|err| err.to_string())?;

	Ok(app_data_dir.join("graph-notes-app"))
}

/// Opens the database and prepares it for the rest of the app.
///
/// This is the place to:
/// - create or reuse the local database directory,
/// - connect SurrealDB to the local engine,
/// - select namespace and database,
/// - apply the first schema.
pub async fn init_db(app: &AppHandle) -> Result<DbHandle, String> {
	let db_dir = resolve_db_dir(app)?;

	std::fs::create_dir_all(&db_dir).map_err(|err| err.to_string())?;
	// Fill in the local engine connection here.
	// Example shape:
	// let db = Surreal::new::<Db>(db_dir).await.map_err(|err| err.to_string())?;
	// Pass the PathBuf by value (not a reference) so it satisfies IntoEndpoint.
	let db = Surreal::new::<SurrealKv>(db_dir)
		.await
		.map_err(|err| err.to_string())?;
	// Fill in namespace/database selection and schema setup here.
	db.use_ns("graphnotes").use_db("vault").await.map_err(|err| err.to_string())?;
	apply_schema(&db).await?;

	
	Ok(db)
}

/// Applies the initial SurrealQL schema used by Phase 1.1.
///
/// Keep this small at first: note, tag, and relationship tables only.
#[allow(dead_code)]
pub async fn apply_schema(db: &DbHandle) -> Result<(), String> {
	db.query(
		"DEFINE TABLE IF NOT EXISTS note SCHEMALESS;
		DEFINE TABLE IF NOT EXISTS tag SCHEMALESS;
		DEFINE TABLE IF NOT EXISTS note_tag TYPE RELATION IN note OUT tag;"
    ).await.map_err(|err| err.to_string())?;
	// Fill in the schema statements here, usually with `query(...).await` calls.
	// Example shape:
	// db.query(include_str!("schema.sql")).await.map_err(|err| err.to_string())?;

	Ok(())
}
