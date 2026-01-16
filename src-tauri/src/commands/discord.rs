use crate::{commands::game::AttachResult, memory::AppState, utils::unguard};
use tauri::State;

#[tauri::command]
pub fn connect_discord_rpc(state: State<'_, AppState>) -> AttachResult {
    let state_arc = state.inner().clone();

    let _ = std::thread::spawn(move || {
        let _ = unguard(&state_arc, |memory| {
            memory.rpc.connect().map_err(|e| e.to_string())
        });
    });

    AttachResult {
        success: true,
        message: "Connecting to Discord RPC...".to_string(),
    }
}

#[tauri::command]
pub fn disconnect_discord_rpc(state: State<'_, AppState>) -> AttachResult {
    let state_arc = state.inner().clone();

    let _ = std::thread::spawn(move || {
        let _ = unguard(&state_arc, |memory| {
            memory.rpc.disconnect().map_err(|e| e.to_string())
        });
    });

    AttachResult {
        success: true,
        message: "Disconnecting from Discord RPC...".to_string(),
    }
}
