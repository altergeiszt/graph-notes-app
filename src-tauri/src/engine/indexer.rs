use sha2::{Sha256, Digest};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tauri::Emitter;
use crate::db::{self, DbHandle};
use crate::engine::parser::parse_note;

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
