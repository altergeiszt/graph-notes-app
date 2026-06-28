use crate::state::AppState;
use crate::types::{BacklinkEntry, NoteSummary};

#[tauri::command]
pub async fn graph_query_backlinks(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<BacklinkEntry>, String> {
    let results: Vec<serde_json::Value> = state
        .db
        .query(
            "SELECT in.path AS source_path, in.title AS source_title, in.content AS source_content
             FROM links_to WHERE out.path = $path",
        )
        .bind(("path", path.clone()))
        .await
        .map_err(|e| e.to_string())?
        .take(0)
        .map_err(|e| e.to_string())?;

    let entries = results
        .into_iter()
        .map(|row| {
            let source_path = row["source_path"].as_str().unwrap_or("").to_string();
            let source_title = row["source_title"].as_str().unwrap_or("").to_string();
            let content = row["source_content"].as_str().unwrap_or("");
            let snippet = extract_snippet(content, &path);
            BacklinkEntry { source_path, source_title, snippet }
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
pub async fn graph_query_forward_links(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<NoteSummary>, String> {
    let mut resp = state
        .db
        .query(
            "SELECT out.id AS id, out.path AS path, out.title AS title,
                    out.updated_at AS updated_at,
                    array::flatten(out->tagged_with->tag.slug) AS tags
             FROM links_to WHERE in.path = $path AND out IS note",
        )
        .bind(("path", path))
        .await
        .map_err(|e| e.to_string())?;

    let rows: Vec<serde_json::Value> = resp.take(0).map_err(|e| e.to_string())?;
    rows.into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
        .collect()
}

#[tauri::command]
pub async fn graph_query_unlinked_mentions(
    path: String,
    title: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<BacklinkEntry>, String> {
    let results: Vec<serde_json::Value> = state
        .db
        .query(
            "SELECT id, path, title, content FROM note
             WHERE path != $path
               AND string::contains(string::lowercase(content), string::lowercase($title))
               AND id NOT IN (SELECT in FROM links_to WHERE out.path = $path)",
        )
        .bind(("path", path.clone()))
        .bind(("title", title.clone()))
        .await
        .map_err(|e| e.to_string())?
        .take(0)
        .map_err(|e| e.to_string())?;

    let entries = results
        .into_iter()
        .map(|row| BacklinkEntry {
            source_path: row["path"].as_str().unwrap_or("").to_string(),
            source_title: row["title"].as_str().unwrap_or("").to_string(),
            snippet: extract_snippet(row["content"].as_str().unwrap_or(""), &title),
        })
        .collect();

    Ok(entries)
}

/// Extract 1-2 lines around the wikilink occurrence in content.
fn extract_snippet(content: &str, target_title: &str) -> String {
    let target_lower = target_title.to_lowercase();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.to_lowercase().contains(&target_lower) {
            let start = i.saturating_sub(1);
            let end = (i + 2).min(lines.len());
            return lines[start..end].join(" ");
        }
    }
    String::new()
}
