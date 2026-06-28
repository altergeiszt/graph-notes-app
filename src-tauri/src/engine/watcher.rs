use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event, EventKind, Config};
use tokio::sync::mpsc;

pub async fn start(
    app: &tauri::AppHandle,
    vault_root: &PathBuf,
    state: tauri::State<'_, AppState>,
) -> Result<(), notify::Error> {
    let (tx, mut rx) = mpsc::channel::<Result<Event, notify::Error>>(32);
    let app_handle = app.clone();
    let db = state.db.clone();
    let root = vault_root.clone();

    let mut watcher = RecommendedWatcher::new(move |res| { let _ = tx.blocking_send(res); },notify::Config::default(),)?;
    watcher.watch(vault_root, RecursiveMode::Recursive)?;

    *state.watcher.write().await = Some(watcher);

 tokio::spawn(async move { 
        while let Some(Ok(event)) = rx.recv().await { 
            match event.kind { 
                EventKind::Create(_) | EventKind::Modify(_) => { 
                    for path in event.paths.iter().filter(|p| p.extension() 
                        .and_then(|e| e.to_str()) == Some("md")) { 
                        let indexer = IndexerService { 
                            db: db.clone(), vault_root: root.clone(),
                                    app_handle: app_handle.clone(), 
                        }; 
                        indexer.index_one(path).await.ok(); 
                    } 
                } 
                EventKind::Remove(_) => { 
                    // Remove from DB — path is gone, use it to find the record 
                    for path in &event.paths { 
                        let path_str = path.to_string_lossy().to_string(); 
                        db.query("DELETE note WHERE path = $p") 
                            .bind(("p", path_str)).await.ok(); 
                    } 
                } 
                _ => {} // Ignore access events 
            } 
        } 
    }); 
    Ok(()) 
}