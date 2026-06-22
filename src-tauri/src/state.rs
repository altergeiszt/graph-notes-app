use std::sync::Arc;
use tokio::sync::RwLock;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;

/// Shared application state, passed to all Tauri commands.
/// 
/// It owns the database connection (wrapped in Arc + RwLock so commands
/// can read/write concurrently without blocking each other for long).

pub struct AppState {
    /// The SurrealDB connection handle wrapped for thread-safe sharing.
    /// Arc = shared ownership across threads.
    /// RwLock = multiple readers OR one writer (Rust's readers-writer lock).
    pub(crate) db: Arc<RwLock<Surreal<Db>>>,
}

impl AppState {
    /// Creates a new AppState wrapping the given database handle.
    ///
    /// # Example
    /// ```ignore
    /// let db = /* Surreal connection from db::init_db() */;
    /// let state = AppState::new(db);
    /// ```
    pub fn new(db: Surreal<Db>) -> Self {
        Self {
            db: Arc::new(RwLock::new(db)),
        }
    }

    /// Returns a clone of the Arc-wrapped database handle for use in commands.
    ///
    /// # Example
    /// ```ignore
    /// let db_handle = state.db();
    /// let guard = db_handle.read().await;  // acquire read lock
    /// ```

    #[allow(dead_code)]
    pub fn db(&self) -> Arc<RwLock<Surreal<Db>>> {
        self.db.clone()
    }
}