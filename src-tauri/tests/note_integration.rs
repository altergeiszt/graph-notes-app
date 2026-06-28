use graph_notes_app_lib::db;
use graph_notes_app_lib::db::DbHandle;
use graph_notes_app_lib::engine::parser::parse_note;
use graph_notes_app_lib::engine::indexer::note_id_from_path;
use surrealdb::engine::local::Mem;
use std::path::Path;
use std::sync::Arc;

async fn setup_db() -> Arc<DbHandle> {
    let db: DbHandle = surrealdb::Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("graphnotes").use_db("test").await.unwrap();
    db.query(include_str!("../src/db/schema.surql")).await.unwrap();
    Arc::new(db)
}

#[tokio::test]
async fn test_note_upsert_and_retrieve() {
    let db = setup_db().await;
    let vault_root = Path::new("/vault");
    let note_path = Path::new("/vault/test-note.md");
    let content = "---\ntitle: Test Note\n---\nHello world.";
    let parsed = parse_note(note_path, content);
    let id = note_id_from_path(vault_root, note_path);

    db::notes::upsert(&db, &id, note_path, &parsed, content).await.unwrap();

    let summary = db::notes::get_summary(&db, &id).await.unwrap();
    assert_eq!(summary.title, "Test Note");
    assert_eq!(summary.path, "/vault/test-note.md");
}

#[tokio::test]
async fn test_upsert_is_idempotent() {
    // Calling upsert twice for the same note must not duplicate records
    let db = setup_db().await;
    let id = "note:abc123";
    let path = Path::new("/vault/idempotent.md");
    let parsed = parse_note(path, "# Idempotent");

    db::notes::upsert(&db, id, path, &parsed, "# Idempotent").await.unwrap();
    db::notes::upsert(&db, id, path, &parsed, "# Idempotent Updated").await.unwrap();

    let count: Vec<serde_json::Value> = db
        .query("SELECT count() FROM note GROUP ALL")
        .await.unwrap().take(0).unwrap();
    assert_eq!(count[0]["count"], serde_json::json!(1));
}

#[tokio::test]
async fn test_note_delete_cleans_up_edges() {
    // Edge case from §3.5.6: delete removes linked edges
    let _db = setup_db().await;
    // ... set up a note with a tag edge, then delete, verify edge is gone
}
