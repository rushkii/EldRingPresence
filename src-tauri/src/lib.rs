use std::sync::{Arc, Mutex};
use tauri::Manager;

use crate::memory::AppState;

mod commands;
mod constants;
mod logic;
mod memory;
mod models;
mod utils;
mod discord;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state: AppState = Arc::new(Mutex::new(None));
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::game::attach_to_game,
            commands::game::get_game_status,
            commands::listener::memory_listener,
            commands::listener::update_discord_rpc,
            commands::discord::connect_discord_rpc,
            commands::discord::disconnect_discord_rpc,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
