use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::Emitter;
use crate::db::DbHandle;

pub async fn cascade_rename(
    db: &Arc<DbHandle>,
    _vault_root: &Path,
    old_title: &str,
    new_title: &str,
    app: &tauri::AppHandle,
) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    // Find all notes whose links_to edges point to the old title
    let sources: Vec<serde_json::Value> = db
        .query("SELECT in.path AS path FROM links_to WHERE out.title = $old")
        .bind(("old", old_title))
        .await?
        .take(0)?;

    let total = sources.len() as u32;
    let mut updated = 0u32;

    let pattern = format!(
        r"(!?)\[\[{}((?:#[^\]]*)?(?:\|[^\]]*)?)\]\]",
        regex::escape(old_title)
    );
    let re = regex::Regex::new(&pattern)?;

    for (i, src) in sources.iter().enumerate() {
        let path_str = src["path"].as_str().unwrap_or("");
        let path = PathBuf::from(path_str);

        let content = tokio::fs::read_to_string(&path).await?;
        let new_content = re
            .replace_all(&content, |caps: &regex::Captures| {
                let prefix = &caps[1]; // '!' or ''
                let suffix = &caps[2]; // '#Section' or '|alias' or ''
                format!("{}[[{}{}]]", prefix, new_title, suffix)
            })
            .to_string();

        if new_content != content {
            tokio::fs::write(&path, &new_content).await?;
            updated += 1;
        }

        let pct = ((i + 1) as f32 / total as f32 * 100.0) as u8;
        app.emit(
            "refactor_progress",
            serde_json::json!({ "pct": pct, "updated": updated, "total": total }),
        )
        .ok();
    }

    Ok(updated)
}
