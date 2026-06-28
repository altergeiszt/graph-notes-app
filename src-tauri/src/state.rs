use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use notify::RecommendedWatcher;

pub struct AppState {
    pub db: Arc<Surreal<Mem>>,
    pub vault_path: Arc<RwLock<Option<PathBuf>>>,
    pub watcher: Arc<RwLock<Option<RecommendedWatcher>>>,
}