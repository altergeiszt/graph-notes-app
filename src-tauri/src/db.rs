use std::path::PathBuf;
use surrealdb::engine::local::SurrealKv;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::{AppHandle, Manager};

/// Convenient alias for the local SurrealDB handle used by the app.
pub type DbHandle = Surreal<Db>;

/// Resolves the folder under app data where the database files live.
pub fn resolve_db_dir(app: &AppHandle) -> Result<PathBuf, String> {
	let app_data_dir = app
		.path()
		.app_data_dir()
		.map_err(|err| err.to_string())?;

	Ok(app_data_dir.join("graphnotes.db"))
}

/// Opens the SurrealKV database, selects the namespace/db, and applies the schema.
pub async fn init_db(app: &AppHandle) -> Result<DbHandle, String> {
	let db_dir = resolve_db_dir(app)?;

	std::fs::create_dir_all(&db_dir).map_err(|err| err.to_string())?;

	let db = Surreal::new::<SurrealKv>(db_dir)
		.await
		.map_err(|err| err.to_string())?;

	db.use_ns("graphnotes").use_db("vault").await.map_err(|err| err.to_string())?;
	apply_schema(&db).await?;

	Ok(db)
}

/// Applies the Phase 1.1 SurrealQL schema.
///
/// All statements use IF NOT EXISTS so this is safe to call on every startup.
/// dangling_node is defined before links_to because links_to references it in its OUT type.
pub async fn apply_schema(db: &DbHandle) -> Result<(), String> {
	// Dangling link placeholders — defined first because links_to references this table.
	db.query(
		"DEFINE TABLE IF NOT EXISTS dangling_node SCHEMAFULL;
		DEFINE FIELD IF NOT EXISTS name ON dangling_node TYPE string;
		DEFINE INDEX IF NOT EXISTS dangling_name ON dangling_node COLUMNS name UNIQUE;"
	).await.map_err(|err| err.to_string())?;

	// Notes as documents and graph nodes.
	db.query(
		"DEFINE TABLE IF NOT EXISTS note SCHEMAFULL;
		DEFINE FIELD IF NOT EXISTS path        ON note TYPE string;
		DEFINE FIELD IF NOT EXISTS title       ON note TYPE string;
		DEFINE FIELD IF NOT EXISTS content     ON note TYPE string;
		DEFINE FIELD IF NOT EXISTS frontmatter ON note TYPE object;
		DEFINE FIELD IF NOT EXISTS created_at  ON note TYPE datetime;
		DEFINE FIELD IF NOT EXISTS updated_at  ON note TYPE datetime;
		DEFINE INDEX IF NOT EXISTS note_path   ON note COLUMNS path UNIQUE;"
	).await.map_err(|err| err.to_string())?;

	// Wikilink edges between notes (or to dangling placeholders).
	db.query(
		"DEFINE TABLE IF NOT EXISTS links_to SCHEMAFULL TYPE RELATION IN note OUT note | dangling_node;
		DEFINE FIELD IF NOT EXISTS alias          ON links_to TYPE option<string>;
		DEFINE FIELD IF NOT EXISTS section_anchor ON links_to TYPE option<string>;
		DEFINE FIELD IF NOT EXISTS block_id       ON links_to TYPE option<string>;
		DEFINE FIELD IF NOT EXISTS line_number    ON links_to TYPE int;"
	).await.map_err(|err| err.to_string())?;

	// Tags as taxonomy nodes.
	db.query(
		"DEFINE TABLE IF NOT EXISTS tag SCHEMAFULL;
		DEFINE FIELD IF NOT EXISTS name ON tag TYPE string;
		DEFINE FIELD IF NOT EXISTS slug ON tag TYPE string;
		DEFINE INDEX IF NOT EXISTS tag_slug ON tag COLUMNS slug UNIQUE;"
	).await.map_err(|err| err.to_string())?;

	// Note-to-tag relation.
	db.query(
		"DEFINE TABLE IF NOT EXISTS tagged_with SCHEMAFULL TYPE RELATION IN note OUT tag;"
	).await.map_err(|err| err.to_string())?;

	Ok(())
}
