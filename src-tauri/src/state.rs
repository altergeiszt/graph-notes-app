use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use notify::RecommendedWatcher;
use crate::db::DbHandle;

pub struct AppState {
    pub db: Arc<DbHandle>,
    pub vault_path: Arc<RwLock<Option<PathBuf>>>,
    pub watcher: Arc<RwLock<Option<RecommendedWatcher>>>,
}

impl AppState {
    pub fn new(db: DbHandle) -> Self {
        Self {
            db: Arc::new(db),
            vault_path: Arc::new(RwLock::new(None)),
            watcher: Arc::new(RwLock::new(None)),
        }
    }
}
