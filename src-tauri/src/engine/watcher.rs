use std::path::PathBuf;
use std::sync::Arc;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::{mpsc, RwLock};
use tauri::Emitter;
use crate::db::DbHandle;
use crate::engine::indexer::IndexerService;

pub async fn start(
    app: tauri::AppHandle,
    vault_root: PathBuf,
    db: Arc<DbHandle>,
    watcher_slot: Arc<RwLock<Option<RecommendedWatcher>>>,
) -> Result<(), notify::Error> {
    let (tx, mut rx) = mpsc::channel::<notify::Result<Event>>(32);
    let app_handle = app.clone();
    let db_clone = db.clone();
    let root = vault_root.clone();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.blocking_send(res);
        },
        notify::Config::default(),
    )?;
    watcher.watch(&vault_root, RecursiveMode::Recursive)?;

    *watcher_slot.write().await = Some(watcher);

    tokio::spawn(async move {
        while let Some(Ok(event)) = rx.recv().await {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {
                    for path in event.paths.iter().filter(|p| {
                        p.extension().and_then(|e| e.to_str()) == Some("md")
                    }) {
                        let indexer = IndexerService {
                            db: db_clone.clone(),
                            vault_root: root.clone(),
                            app_handle: app_handle.clone(),
                        };
                        indexer.index_one(path).await.ok();
                    }
                }
                EventKind::Remove(_) => {
                    for path in &event.paths {
                        let path_str = path.to_string_lossy().to_string();
                        db_clone
                            .query("DELETE note WHERE path = $p")
                            .bind(("p", path_str))
                            .await
                            .ok();
                    }
                }
                _ => {}
            }
        }
    });

    Ok(())
}
