use std::path::{Path, PathBuf};

use tauri::AppHandle;

use crate::db::DbHandle;

/// Payload for the `vault_index_progress` event emitted during indexing.
///
/// Sent to the frontend after each `.md` file is processed so the UI can
/// display a progress bar.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexProgressPayload {
    /// Percentage complete, 0–100.
    pub pct: u8,
    /// Number of files processed so far.
    pub scanned: u32,
    /// Total number of `.md` files discovered in the vault.
    pub total: u32,
}

/// Payload for the `vault_index_done` event emitted when indexing finishes.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexDonePayload {
    /// Total number of notes that were upserted into SurrealDB.
    pub note_count: u32,
}

/// Recursively walks `vault_dir` and collects the absolute path of every
/// `.md` file it contains, including files in nested subdirectories.
///
/// This is a pure discovery function — it does not read file contents.
/// Call it first to get the total file count, then iterate the results
/// in `index_vault` to drive the progress percentage.
///
/// # Arguments
///
/// * `vault_dir` — Root directory of the vault to scan.
///
/// # Returns
///
/// `Ok(Vec<PathBuf>)` — a flat list of absolute paths to every `.md` file
/// found under `vault_dir`.
///
/// `Err(String)` — if `tokio::fs::read_dir` fails on any directory (e.g.,
/// permission denied).
///
/// # Example (pseudocode)
///
/// ```ignore
/// let md_files = collect_md_files(Path::new("/home/user/vault")).await?;
/// println!("Found {} notes", md_files.len());
/// ```
pub async fn collect_md_files(vault_dir: &Path) -> Result<Vec<PathBuf>, String> {
    todo!()
    // Steps to implement:
    //
    // 1. Create a mutable Vec<PathBuf> to accumulate results.
    //
    // 2. Create a queue (VecDeque<PathBuf>) seeded with `vault_dir.to_path_buf()`.
    //
    // 3. Loop while the queue is non-empty:
    //    a. Pop a directory path from the front.
    //    b. Call `tokio::fs::read_dir(&dir_path).await` to open it.
    //    c. Loop over entries with `dir.next_entry().await`:
    //       - If the entry is a directory → push its path onto the queue.
    //       - If the entry is a file with extension ".md" → push to results Vec.
    //
    // 4. Return Ok(results).
}

/// Reads, parses, and upserts every `.md` file in the vault into SurrealDB,
/// emitting `vault_index_progress` events during the scan and a final
/// `vault_index_done` event when complete.
///
/// This function is designed to run inside a `tokio::spawn` background task
/// so that `vault_open` can return immediately. Use `app.emit(...)` to push
/// progress events to the frontend; no return value is needed by the caller.
///
/// # Arguments
///
/// * `vault_path` — Root directory of the vault (owned, so it can be moved
///                  into the async background task).
/// * `db`         — Cloned `DbHandle` from `AppState`. Cloning a SurrealDB
///                  handle is cheap — it shares the underlying connection.
/// * `app`        — Cloned `AppHandle`. Used to emit Tauri events. Must be
///                  cloned before calling `tokio::spawn` in the caller.
///
/// # Errors
///
/// Returns `Err(String)` if `collect_md_files` fails. Individual file read
/// or parse failures should be logged and skipped rather than aborting the
/// entire index run.
///
/// # Events emitted
///
/// - `"vault_index_progress"` with [`IndexProgressPayload`] — after each file.
/// - `"vault_index_done"`     with [`IndexDonePayload`]     — once all done.
pub async fn index_vault(
    vault_path: PathBuf,
    db: DbHandle,
    app: AppHandle,
) -> Result<(), String> {
    todo!()
    // Steps to implement:
    //
    // 1. Call `collect_md_files(&vault_path).await?` to get the full file list.
    //    Store the length as `total: u32`.
    //
    // 2. Iterate over the file list with `enumerate()` so you have an index:
    //    for (index, file_path) in md_files.iter().enumerate() { ... }
    //
    // 3. For each file:
    //    a. Read content: `tokio::fs::read_to_string(&file_path).await`
    //       — on error, log with `eprintln!` and `continue`.
    //    b. Parse frontmatter and derive a title (Phase 1.3 adds a real parser;
    //       for now, use the filename stem as the title).
    //    c. Upsert the note into SurrealDB:
    //       db.query("CREATE OR UPDATE note SET path = $path, title = $title, ...")
    //         .bind(("path", file_path.to_string_lossy()))
    //         ...
    //         .await
    //       — on error, log and continue.
    //    d. Compute progress:
    //       let scanned = (index + 1) as u32;
    //       let pct = ((scanned * 100) / total) as u8;
    //    e. Emit the progress event:
    //       app.emit("vault_index_progress", IndexProgressPayload { pct, scanned, total }).ok();
    //
    // 4. After the loop, emit the done event:
    //    app.emit("vault_index_done", IndexDonePayload { note_count: total }).ok();
    //
    // 5. Return Ok(()).
}
