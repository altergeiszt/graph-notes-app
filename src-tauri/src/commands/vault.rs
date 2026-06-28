use std::path::PathBuf;

use tauri::Emitter;
use crate::engine::indexer::index_vault;
use crate::state::AppState;
use crate::types::VaultInfo;

#[tauri::command]
pub async fn vault_open(
    path: String,
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<VaultInfo, String> {
    let vault_root = PathBuf::from(&path);
    if !vault_root.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }
    {
        let mut w = state.watcher.write().await;
        *w = None;
    }
    {
        let mut vp = state.vault_path.write().await;
        *vp = Some(vault_root.clone());
    }

    let db = state.db.clone();
    let app_handle = app.clone();
    let root = vault_root.clone();
    tokio::spawn(async move {
        if let Err(e) = index_vault(root, db, app_handle.clone()).await {
            app_handle.emit("vault_index_error", e.to_string()).ok();
        }
    });

    Ok(VaultInfo { path, note_count: 0 })
}
