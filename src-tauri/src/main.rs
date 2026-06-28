// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;

use std::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default().setup(|app| {
        let app_data = app.path().app_data_dir()?;
        std::fs::create_dir_all(&app_data)?;

    let _db = tauri_async_runtime::block_on(async {
        let app_data = tauri::utils::platform::app_data_dir(&tauri::Config::default())?;
        let db = Surreal::new::<SurrealKv>(app_data.join("graphnotes.db")).await?;
        db.use_ns("graphnotes").use_db("vault").await?;
        Ok::<_, surrealdb::Error>(Arc::new(db))
    })?;
    app.manage(AppState {
        db,
        vault_path: Arc::new(RwLock::new(None)),
        watcher: Arc::new(RwLock::new(None)),
    });
    Ok(())
})
.invoke_handler(tauri::generate_handler![
    commands::vault::vault_open,
    commands::vault::vault_close,
    commands::vault::vault_rebuild,
    commands::notes::note_list,
    commands::notes::note_read,
    commands::notes::notes_save,
    commands::notes::notes_create,
    commands::notes::notes_delete,
    commands::notes::note_rename,
    commands::graph::graph_query_back_links,
    commands::graph::graph_query_forward_links,
    commands::graph::graph_query_unlinked_mentions,])
    .run(tauri::generate_context!())?;

    Ok(())
}
