mod commands;
mod db;
mod engine;
mod state;
mod types;

use state::AppState;
use std::error::Error;
use tauri::{AppHandle, Manager, async_runtime::block_on};

async fn initialize_app_state(app: &AppHandle) -> Result<AppState, String> {
    let db = db::init_db(app).await.map_err(|e| e.to_string())?;
    Ok(AppState::new(db))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| -> Result<(), Box<dyn Error>> {
            let state = block_on(initialize_app_state(&app.handle())).map_err(|e| {
                eprintln!("Failed to initialize app state: {}", e);
                Box::<dyn Error>::from(e)
            })?;
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::vault::vault_open,
            commands::notes::note_list,
            commands::notes::note_read,
            commands::notes::note_save,
            commands::notes::note_create,
            commands::notes::note_delete,
            commands::notes::note_rename,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
