use std::path::Path;
use std::sync::Arc;
use serde_json::Value as JsonValue;
use surrealdb::types::RecordId;

use crate::db::DbHandle;
use crate::engine::parser::ParsedNote;
use crate::types::NoteSummary;

fn split_id(id: &str) -> (&str, &str) {
    id.split_once(':').expect("ID must be table:key")
}

fn rid(id: &str) -> RecordId {
    let (table, key) = split_id(id);
    RecordId::new(table, key)
}

/// Insert-or-update a note record. Preserves `created_at` on re-index.
pub async fn upsert(
    db: &Arc<DbHandle>,
    id: &str,
    path: &Path,
    parsed: &ParsedNote,
    content: &str,
) -> Result<NoteSummary, String> {
    let (table, key) = split_id(id);
    let path_str = path.to_string_lossy().to_string();

    // Use type::thing to construct the record ID inside SurrealQL so the key is
    // treated as a plain string regardless of how the Rust client serializes RecordId.
    db.query(
        "UPSERT type::record($table, $key) SET
           path = $path,
           title = $title,
           content = $content,
           frontmatter = $frontmatter,
           created_at = IF created_at THEN created_at ELSE time::now() END,
           updated_at = time::now();",
    )
    .bind(("table", table.to_string()))
    .bind(("key", key.to_string()))
    .bind(("path", path_str))
    .bind(("title", parsed.title.clone()))
    .bind(("content", content.to_string()))
    .bind(("frontmatter", serde_json::json!(parsed.frontmatter)))
    .await
    .map_err(|e| e.to_string())?
    .take::<Vec<serde_json::Value>>(0)
    .map_err(|e| e.to_string())?;

    get_summary(db, id).await
}

/// Rebuild tagged_with edges for a note from its parsed tags.
pub async fn upsert_tags(
    db: &Arc<DbHandle>,
    id: &str,
    parsed: &ParsedNote,
) -> Result<(), String> {
    let record_id = rid(id);

    db.query("DELETE FROM tagged_with WHERE in = $rid")
        .bind(("rid", record_id.clone()))
        .await
        .map_err(|e| e.to_string())?;

    for tag_name in parsed.tags_frontmatter.iter() {
        insert_tag(db, &record_id, tag_name, "frontmatter").await?;
    }
    for tag_name in parsed.tags_inline.iter() {
        insert_tag(db, &record_id, tag_name, "inline").await?;
    }

    Ok(())
}

async fn insert_tag(
    db: &Arc<DbHandle>,
    rid: &RecordId,
    tag_name: &str,
    source: &str,
) -> Result<(), String> {
    let slug = tag_name.to_lowercase().replace(' ', "-");
    db.query(
        "INSERT IGNORE INTO tag { name: $name, slug: $slug };
         LET $tag_id = (SELECT VALUE id FROM tag WHERE slug = $slug LIMIT 1)[0];
         INSERT IGNORE INTO tagged_with { in: $rid, out: $tag_id, source: $source };",
    )
    .bind(("name", tag_name.to_string()))
    .bind(("slug", slug))
    .bind(("rid", rid.clone()))
    .bind(("source", source.to_string()))
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Fetch a NoteSummary by record ID string (e.g. "note:abc123").
pub async fn get_summary(db: &Arc<DbHandle>, id: &str) -> Result<NoteSummary, String> {
    let (table, key) = split_id(id);

    let mut resp = db
        .query(
            "SELECT id, path, title, updated_at,
               array::flatten(->tagged_with->tag.slug) AS tags
             FROM type::record($table, $key) LIMIT 1",
        )
        .bind(("table", table.to_string()))
        .bind(("key", key.to_string()))
        .await
        .map_err(|e| e.to_string())?;

    let rows: Vec<JsonValue> = resp.take(0).map_err(|e| e.to_string())?;
    rows.into_iter()
        .next()
        .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
        .transpose()?
        .ok_or_else(|| format!("Note not found: {id}"))
}
