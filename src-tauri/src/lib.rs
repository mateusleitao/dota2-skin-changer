pub mod backup;
pub mod commands;
pub mod game_path;
pub mod installer;
pub mod item_db;
pub mod models;

use commands::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            detect_game,
            install_hook,
            uninstall_hook,
            get_hook_status,
            get_backups,
            restore_backup,
            get_item_catalog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
