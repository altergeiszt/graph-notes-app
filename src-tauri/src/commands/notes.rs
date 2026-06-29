use std::path::PathBuf;
use serde_json::Value as JsonValue;
use surrealdb::types::RecordId;
use tauri::Emitter;

use crate::db;
use crate::engine::indexer::{note_id_from_path, IndexerService};
use crate::engine::parser;
use crate::engine::refactor::cascade_rename;
use crate::state::AppState;
use crate::types::{NoteRecord, NoteSummary, RefactorResult};

// ─── note_list ───────────────────────────────────────────────────────────────

/// Returns all notes sorted by updated_at descending.
#[tauri::command]
pub async fn note_list(state: tauri::State<'_, AppState>) -> Result<Vec<NoteSummary>, String> {
    let mut resp = state
        .db
        .query(
            "SELECT id, path, title, updated_at,
               array::flatten(->tagged_with->tag.slug) AS tags
             FROM note ORDER BY updated_at DESC",
        )
        .await
        .map_err(|e| e.to_string())?;

    let rows: Vec<JsonValue> = resp.take(0).map_err(|e| e.to_string())?;
    rows.into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
        .collect()
}

// ─── note_read ───────────────────────────────────────────────────────────────

/// Reads note content from disk (source of truth) and metadata from DB.
#[tauri::command]
pub async fn note_read(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<NoteRecord, String> {
    let p = PathBuf::from(&path);
    let content = tokio::fs::read_to_string(&p)
        .await
        .map_err(|e| format!("Failed to read {path}: {e}"))?;

    let mut resp = state
        .db
        .query(
            "SELECT id, path, title, frontmatter, created_at, updated_at
             FROM note WHERE path = $path LIMIT 1",
        )
        .bind(("path", path.clone()))
        .await
        .map_err(|e| e.to_string())?;

    let rows: Vec<JsonValue> = resp.take(0).map_err(|e| e.to_string())?;
    let mut record: NoteRecord = rows
        .into_iter()
        .next()
        .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
        .transpose()?
        .ok_or_else(|| format!("Note not found in index: {path}"))?;

    record.content = content; // disk is always the source of truth
    Ok(record)
}

// ─── note_save ───────────────────────────────────────────────────────────────

/// Writes content to disk, then re-parses and updates the SurrealDB record.
#[tauri::command]
pub async fn note_save(
    path: String,
    content: String,
    state: tauri::State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<NoteSummary, String> {
    tokio::fs::write(&path, &content)
        .await
        .map_err(|e| format!("Write failed: {e}"))?;

    let p = PathBuf::from(&path);
    let vault_root = state
        .vault_path
        .read()
        .await
        .clone()
        .ok_or("No vault open")?;

    let parsed = parser::parse_note(&p, &content);
    let id = note_id_from_path(&vault_root, &p);

    let summary = db::notes::upsert(&state.db, &id, &p, &parsed, &content).await?;
    db::notes::upsert_tags(&state.db, &id, &parsed).await?;

    Ok(summary)
}

// ─── note_create ─────────────────────────────────────────────────────────────

/// Creates a new note file with a stub frontmatter, indexes it, and returns its summary.
#[tauri::command]
pub async fn note_create(
    title: String,
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<NoteSummary, String> {
    let vault_root = state
        .vault_path
        .read()
        .await
        .clone()
        .ok_or("No vault open")?;

    let safe_name = sanitize_filename(&title);
    let safe_name = if safe_name.is_empty() {
        "Untitled".to_string()
    } else {
        safe_name
    };

    // Avoid filename collisions: append _2, _3, …
    let mut path = vault_root.join(format!("{safe_name}.md"));
    let mut counter = 2u32;
    while path.exists() {
        path = vault_root.join(format!("{safe_name}_{counter}.md"));
        counter += 1;
    }

    let frontmatter = format!(
        "---\ntitle: {title}\ndate: {}\n---\n",
        chrono::Local::now().format("%Y-%m-%d"),
    );
    tokio::fs::write(&path, &frontmatter)
        .await
        .map_err(|e| e.to_string())?;

    let indexer = IndexerService {
        db: state.db.clone(),
        vault_root: vault_root.clone(),
        app_handle: app,
    };

    // SurrealDB can emit "Transaction write conflict" under concurrent load; retry up to 5×.
    let mut attempts = 0u32;
    loop {
        match indexer.index_one(&path).await {
            Ok(()) => break,
            Err(e) if e.to_string().contains("Transaction write conflict") && attempts < 5 => {
                attempts += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(50 * u64::from(attempts))).await;
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    let id = note_id_from_path(&vault_root, &path);
    db::notes::get_summary(&state.db, &id).await
}

// ─── note_delete ─────────────────────────────────────────────────────────────

/// Deletes the note file from disk and removes its DB record and edges.
#[tauri::command]
pub async fn note_delete(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let vault_root = state
        .vault_path
        .read()
        .await
        .clone()
        .ok_or("No vault open")?;

    let p = PathBuf::from(&path);
    let id = note_id_from_path(&vault_root, &p);
    let (table, key) = id.split_once(':').unwrap();
    let rid = RecordId::new(table, key);

    // Delete file first; ignore ENOENT (file may have been deleted externally).
    match tokio::fs::remove_file(&p).await {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.to_string()),
    }

    // Remove edges then the record.
    state
        .db
        .query(
            "DELETE FROM links_to   WHERE in = $rid OR out = $rid;
             DELETE FROM tagged_with WHERE in = $rid;
             DELETE $rid;",
        )
        .bind(("rid", rid))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ─── note_rename ─────────────────────────────────────────────────────────────

/// Renames a note file on disk and updates its DB path and ID (Phase 3 fills in link cascade).
#[tauri::command]
pub async fn note_rename(
    old_path: String,
    new_title: String,
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<NoteSummary, String> {
    let vault_root = state
        .vault_path
        .read()
        .await
        .clone()
        .ok_or("No vault open")?;

    let old_p = PathBuf::from(&old_path);
    let safe_name = sanitize_filename(&new_title);
    let safe_name = if safe_name.is_empty() {
        "Untitled".to_string()
    } else {
        safe_name
    };

    let new_p = old_p
        .parent()
        .unwrap_or(&vault_root)
        .join(format!("{safe_name}.md"));

    // Derive old title from the old path stem before anything changes on disk.
    let old_title = old_p
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    tokio::fs::rename(&old_p, &new_p)
        .await
        .map_err(|e| e.to_string())?;

    // Spawn cascade link refactor BEFORE deleting the old DB record so the
    // links_to graph query can still resolve edges pointing to the old note.
    {
        let db_c = state.db.clone();
        let app_c = app.clone();
        let root_c = vault_root.clone();
        let old_t = old_title.clone();
        let new_t = new_title.clone();
        tokio::spawn(async move {
            let count = cascade_rename(&db_c, &root_c, &old_t, &new_t, &app_c)
                .await
                .unwrap_or(0);
            app_c
                .emit("refactor_done", RefactorResult { updated_count: count })
                .ok();
        });
    }

    // Remove old DB record; index_one will create the new one.
    let old_id = note_id_from_path(&vault_root, &old_p);
    let (table, key) = old_id.split_once(':').unwrap();
    let old_rid = RecordId::new(table, key);
    state
        .db
        .query("DELETE FROM tagged_with WHERE in = $rid; DELETE $rid;")
        .bind(("rid", old_rid))
        .await
        .map_err(|e| e.to_string())?;

    let indexer = IndexerService {
        db: state.db.clone(),
        vault_root: vault_root.clone(),
        app_handle: app,
    };
    indexer.index_one(&new_p).await.map_err(|e| e.to_string())?;

    let new_id = note_id_from_path(&vault_root, &new_p);
    db::notes::get_summary(&state.db, &new_id).await
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn sanitize_filename(title: &str) -> String {
    const BAD: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    title
        .chars()
        .filter(|c| !BAD.contains(c))
        .collect::<String>()
        .trim()
        .to_string()
}

#[test]
fn test_sanitize_removes_forbidden_characters() {
    assert_eq!(sanitize_filename("My: Note? <Bad>"), "My Note Bad");
}

#[test]
fn test_sanitize_all_special_chars_returns_untitled_fallback() {
    // Edge case from §3.5.6: sanitize returns empty → caller falls back to "Untitled"
    let result = sanitize_filename("/:*?\"<>|\\");
    assert!(result.is_empty()); // caller handles the empty case
}

#[test]
fn test_sanitize_preserves_spaces_and_unicode() {
    assert_eq!(sanitize_filename("Café Notes"), "Café Notes");
}