mod db;
mod state;

use state::AppState;
use tauri::AppHandle;

/// Example command used to verify Tauri invocation wiring.
///
/// Keep this only if you still want a simple smoke-test command.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Initializes the application state by opening the SurrealDB database
/// and applying the schema before any commands run.
async fn initialize_app_state(app: &AppHandle) -> Result<AppState, String> {
    let db = db::init_db(app)
        .await
        .map_err(|err| err.to_string())?;

    Ok(AppState::new(db))
}

/// Starts the Tauri application, registers plugins, initializes shared state,
/// and exposes Rust commands to the frontend.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let state = tauri::async_runtime::block_on(async {
                initialize_app_state(&app_handle).await
            })?;

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}