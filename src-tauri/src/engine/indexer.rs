use sha2::{Sha256, Digest};
use surrealdb::engine::local::SurrealKv;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use crate::db;

pub fn note_id_from_path(vault_root: &Path, note_path: &Path) -> String {
    let relative = note_path.strip_prefix(vault_root).expect("Note path must be under the vault root");

    let normalized = relative.to_string_lossy().replace("\\", "/");
    let hash = Sha256::digest(normalized.as_bytes());
    format!("note:{}", hex::encode(&hash[..16]))
}

pub struct IndexerService {
    pub db: Arc<SurrealKv<SurrealKv>>,
    pub vault_root: PathBuf,
    pub app_handle: tauri::AppHandle,
}

impl IndexerService {
    pub async fn index_full(&self) -> Result<u64, IndexError> {
        let paths = self.collect_md_paths().await?;
        let total = paths.len() as u64;

        for (i, path) in paths.iter().enumerate() {
            self.index_one(path).await?;
            let pct = ((i + 1) as f32 / total as f32 * 100.0) as u8;
            self.app_handle.emit("Vault_index_progress", ProgressPayload { pct, message: format! ("Indexed {}/{}", i + 1, total),}).ok();
        }
            Ok(total)
        }
    pub async fn index_one(&self, path: &Path) -> Result<(), IndexerError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| IndexerError::ReadFailed(path.to_path_buf(), e))?;
        let parsed = parse_note(path, &content);
        let id = note_id_from_path(&self.vault_root, path);

        db::notes::upsert(&self.db, &id, path, &parsed, &content).await?;
        db::notes::upsert_tags(&self.db, &id, &parsed).await?;
        Ok(())
    }

    async fn collect_md_paths(&self) -> Result<Vec<PathBuf>, IndexerError> {
        let mut result = Vec::new();
        let mut queue = vec![self.vault_root.clone()];
        while let Some(dir) = queue.pop() {
            let mut entries = tokio::fs::read_dir(&dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.is_dir() { queue.push(path);}
                else if path.extension().and_then(|e| e.to_str()) == Some("md") {result.push(path);}
            }
        }
        Ok(result)
    }

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("Failed to read file {0:?}: {1}")]
    ReadFailed(PathBuf, std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] surrealdb::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<IndexerError> for String {
    fn from(e: IndexerError) -> Self{e.to_string()}
}

}