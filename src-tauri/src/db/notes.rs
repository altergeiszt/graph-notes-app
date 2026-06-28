use std::path::Path;
use std::sync::Arc;
use serde_json::Value as JsonValue;
use surrealdb::types::RecordId;

use crate::db::DbHandle;
use crate::engine::parser::ParsedNote;
use crate::types::NoteSummary;

fn rid(id: &str) -> RecordId {
    let (table, key) = id.split_once(':').expect("ID must be table:key");
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
    let record_id = rid(id);
    let path_str = path.to_string_lossy().to_string();

    // INSERT IGNORE creates the record (with created_at) only when it doesn't exist.
    // The following UPDATE refreshes all mutable fields without touching created_at.
    db.query(
        "INSERT IGNORE INTO note {
           id: $rid,
           path: $path,
           title: $title,
           content: $content,
           frontmatter: $frontmatter,
           created_at: time::now(),
           updated_at: time::now()
         };
         UPDATE $rid SET
           path = $path,
           title = $title,
           content = $content,
           frontmatter = $frontmatter,
           updated_at = time::now();",
    )
    .bind(("rid", record_id))
    .bind(("path", path_str))
    .bind(("title", parsed.title.clone()))
    .bind(("content", content.to_string()))
    .bind(("frontmatter", serde_json::json!(parsed.frontmatter)))
    .await
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
        "LET $tag = (INSERT INTO tag { name: $name, slug: $slug }
                     ON DUPLICATE KEY UPDATE name = $name)[0];
         RELATE $rid->tagged_with->$tag.id CONTENT { source: $source };",
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
    let record_id = rid(id);

    let mut resp = db
        .query(
            "SELECT id, path, title, updated_at,
               array::flatten(->tagged_with->tag.slug) AS tags
             FROM $rid LIMIT 1",
        )
        .bind(("rid", record_id))
        .await
        .map_err(|e| e.to_string())?;

    let rows: Vec<JsonValue> = resp.take(0).map_err(|e| e.to_string())?;
    rows.into_iter()
        .next()
        .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
        .transpose()?
        .ok_or_else(|| format!("Note not found: {id}"))
}
