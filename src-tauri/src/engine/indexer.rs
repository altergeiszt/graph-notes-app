use sha2::{Sha256, Digest};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tauri::Emitter;
use crate::db::{self, DbHandle};
use crate::engine::parser::{parse_note, extract_wikilinks, WikilinkMatch};

pub fn note_id_from_path(vault_root: &Path, note_path: &Path) -> String {
    let relative = note_path
        .strip_prefix(vault_root)
        .expect("Note path must be under the vault root");
    let normalized = relative.to_string_lossy().replace('\\', "/");
    let hash = Sha256::digest(normalized.as_bytes());
    format!("note:{}", hex::encode(&hash[..16]))
}

pub struct IndexerService {
    pub db: Arc<DbHandle>,
    pub vault_root: PathBuf,
    pub app_handle: tauri::AppHandle,
}

pub async fn index_vault(
    vault_root: PathBuf,
    db: Arc<DbHandle>,
    app_handle: tauri::AppHandle,
) -> Result<u64, IndexerError> {
    let service = IndexerService { db, vault_root, app_handle };
    service.index_full().await
}

pub async fn resolve_and_upsert_links(
    db: &Arc<DbHandle>,
    source_id: &str,
    wikilinks: &[WikilinkMatch],
) -> Result<(), surrealdb::Error> {
    // Delete existing links FROM this source (full re-index on every save) 
    db.query("DELETE links_to WHERE in = $src") 
        .bind(("src", source_id)) 
        .await?; 
     for wl in wikilinks { 
        let target_id = resolve_target(db, &wl.target).await?; 
        let (out_table, out_id) = match target_id { 
            Some(id) => ("note", id),
                       None => { 
                // Upsert dangling_node 
                let dn: Vec<serde_json::Value> = db.query(
                    "INSERT INTO dangling_node { name: $name } ON DUPLICATE KEY IGNORE"
                ).bind(("name", &wl.target)).await?.take(0)?;
                let dn_id = dn.first() 
                    .and_then(|v| v.get("id")) 
                    .and_then(|v| v.as_str()) 
                    .unwrap_or_default().to_string(); 
                ("dangling_node", dn_id) 
            } 
        }; 
         db.query( 
            "RELATE $src -> links_to -> $tgt CONTENT { 
                alias: $alias, 
                section_anchor: $section, 
                block_id: $block, 
                line_number: $line 
            }" 
        ) 
        .bind(("src", source_id)) 
        .bind(("tgt", &format!("{out_table}:{out_id}"))) 
        .bind(("alias", &wl.alias)) 
        .bind(("section", &wl.section_anchor)) 
        .bind(("block", &wl.block_id)) 
        .bind(("line", wl.line_number)) 
        .await?; 
    } 
    Ok(())

}
 
/// Case-insensitive title lookup; returns note ID if found.
async fn resolve_target(
    db: &Arc<DbHandle>,
    target: &str,
) -> Result<Option<String>, surrealdb::Error> {
    let results: Vec<serde_json::Value> = db.query(
        "SELECT id FROM note WHERE string::lowercase(title) = string::lowercase($t) LIMIT 1"
    ).bind(("t", target)).await?.take(0)?; 
    Ok(results.first() 
        .and_then(|v| v.get("id")) 
        .and_then(|v| v.as_str()) 
        .map(|s| s.to_string())) 
}

impl IndexerService {
    pub async fn index_full(&self) -> Result<u64, IndexerError> {
        let paths = self.collect_md_paths().await?;
        let total = paths.len() as u64;

        for (i, path) in paths.iter().enumerate() {
            self.index_one(path).await?;
            let pct = ((i + 1) as f32 / total as f32 * 100.0) as u8;
            self.app_handle
                .emit(
                    "vault_index_progress",
                    serde_json::json!({ "pct": pct, "scanned": i + 1, "total": total }),
                )
                .ok();
        }

        self.app_handle
            .emit("vault_index_done", serde_json::json!({ "note_count": total }))
            .ok();
        Ok(total)
    }

    pub async fn index_one(&self, path: &Path) -> Result<(), IndexerError> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| IndexerError::ReadFailed(path.to_path_buf(), e))?;

        let parsed = parse_note(path, &content);
        let id = note_id_from_path(&self.vault_root, path);

        db::notes::upsert(&self.db, &id, path, &parsed, &content)
            .await
            .map_err(IndexerError::Unexpected)?;

        db::notes::upsert_tags(&self.db, &id, &parsed)
            .await
            .map_err(IndexerError::Unexpected)?;

        let wikilinks = extract_wikilinks(&content);
        resolve_and_upsert_links(&self.db, &id, &wikilinks)
            .await
            .map_err(|e| IndexerError::Unexpected(e.to_string()))?;

        Ok(())
    }

    async fn collect_md_paths(&self) -> Result<Vec<PathBuf>, IndexerError> {
        let mut result = Vec::new();
        let mut queue = vec![self.vault_root.clone()];
        while let Some(dir) = queue.pop() {
            let mut entries = tokio::fs::read_dir(&dir)
                .await
                .map_err(|e| IndexerError::ReadFailed(dir.clone(), e))?;
            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| IndexerError::ReadFailed(dir.clone(), e))?
            {
                let path = entry.path();
                if path.is_dir() {
                    queue.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                    result.push(path);
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("Failed to read file {0:?}: {1}")]
    ReadFailed(PathBuf, std::io::Error),

    #[error("Database error: {0}")]
    Unexpected(String),
}

impl From<IndexerError> for String {
    fn from(e: IndexerError) -> Self {
        e.to_string()
    }
}
