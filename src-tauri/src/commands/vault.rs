use tauri::{AppHandle, State};

use crate::engine::indexer;
use crate::state::AppState;

/// The shape returned to the frontend after `vault_open` is called.
///
/// `note_count` is 0 on the initial response because indexing runs in the
/// background. The true count arrives via the `vault_index_done` event.
#[derive(Debug, serde::Serialize)]
pub struct VaultInfo {
    /// Absolute path of the opened vault directory.
    pub path: String,
    /// Number of notes found. Always 0 on the initial response.
    pub note_count: u32,
}

/// Opens a vault directory, stores its path in `AppState`, persists it so
/// the vault reopens on next launch, and spawns a background indexing task.
///
/// This command returns immediately with a `VaultInfo` placeholder. The actual
/// indexing runs in a Tokio background task and reports progress through two
/// Tauri events:
///
/// - `vault_index_progress` — emitted after each file is processed.
///   Payload: `{ pct: u8, scanned: u32, total: u32 }`
/// - `vault_index_done` — emitted once all files have been upserted.
///   Payload: `{ note_count: u32 }`
///
/// The frontend should listen for these events before calling this command
/// (see `App.tsx`), because the events may fire before any subsequent JS
/// `await listen(...)` call has time to register.
///
/// # Arguments
///
/// * `path`  — Absolute path string selected by the user via the folder picker.
/// * `app`   — Tauri `AppHandle`; cloned into the background task so it can
///             emit events after this command returns.
/// * `state` — Shared `AppState`; used to store the vault path and to clone
///             the `DbHandle` for the background indexer task.
///
/// # Errors
///
/// Returns `Err(String)` if:
/// - `path` does not exist on disk.
/// - `path` is not a directory.
/// - Writing the persisted path to `tauri-plugin-store` fails.
#[tauri::command]
pub async fn vault_open(
    path: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<VaultInfo, String> {
    todo!()
    // Steps to implement:
    //
    // 1. Validate that `path` exists and is a directory.
    //    Hint: std::path::Path::new(&path).is_dir()
    //
    // 2. Persist `path` using tauri-plugin-store so it reopens on next launch.
    //    Hint: use `tauri_plugin_store::StoreExt` — open the store, insert the
    //    key "vault_path", and save.
    //
    // 3. Store `path` in AppState so other commands can access it.
    //    Hint: acquire a write lock on state.vault_path and update it.
    //    (You will need to add a `vault_path` field to AppState first.)
    //
    // 4. Clone `app` and `state.db` to move into the background task.
    //
    // 5. Spawn a Tokio background task:
    //    tokio::spawn(async move {
    //        indexer::index_vault(vault_path, db, app_clone).await.ok();
    //    });
    //
    // 6. Return Ok(VaultInfo { path, note_count: 0 }) immediately.
}
