mod commands;
mod db;
mod engine;
mod state;

use state::AppState;
use std::error::Error;
use tauri::{AppHandle, Manager, async_runtime::block_on};

/// Example command—keep or remove as you like.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Initializes the app state by calling db::init_db and wrapping
/// the result in an AppState struct.
///
/// This runs during Tauri setup, before any commands are invoked.
async fn initialize_app_state(app: &AppHandle) -> Result<AppState, String> {
    // TODO: call db::init_db(app).await, handle errors with .map_err(|e| e.to_string())?
    let db = db::init_db(app)
        .await
        .map_err(|err| String::from(err.to_string()))?;

    // TODO: wrap the db in AppState and return it
    Ok(AppState::new(db))
}

/// Starts the Tauri application with all plugins and the setup hook.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| -> Result<(), Box<dyn Error>> {
            // initialize the app state synchronously during setup
            let state = block_on(initialize_app_state(&app.handle()))
                .map_err(|e| {
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
        .invoke_handler(tauri::generate_handler![greet, commands::vault::vault_open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}