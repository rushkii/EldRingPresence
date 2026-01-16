use crate::{
    logic::players::get_player_info,
    memory::AppState,
    models::response::BaseAttribute,
    utils::{render_template, unguard},
};
use discord_rich_presence::activity::Activity;
use serde_json::json;
use tauri::{Emitter, State, Window};

#[tauri::command]
pub fn memory_listener(state: State<'_, AppState>, window: Window) -> Result<(), String> {
    let state_arc = state.inner().clone();
    let window_clone = window.clone();

    std::thread::spawn(move || loop {
        let player = unguard(&state_arc, |memory| get_player_info(memory));

        match player {
            Ok(player) => {
                if let Err(e) = window_clone.emit("memory_event", player) {
                    eprintln!("Failed to emit event: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to get player info: {}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    Ok(())
}

#[tauri::command]
pub fn update_discord_rpc(state: State<'_, AppState>, window: Window) -> Result<(), String> {
    let state_arc = state.inner().clone();

    let _window_clone = window.clone();

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(4));

        let player_result = unguard(&state_arc, |memory| get_player_info(memory));

        match player_result {
            Ok(player) => {
                let _ = unguard(&state_arc, |memory| {
                    let config_details = memory.rpc.config.details.as_deref();
                    let config_state = memory.rpc.config.state.as_deref();

                    let binding_if_none = BaseAttribute::default();
                    let base_attr = player.base_attributes.as_ref().unwrap_or(&binding_if_none);

                    let details_str = render_template(
                        config_details,
                        &json!({
                            "nickname": player.nickname.clone().unwrap_or_else(|| "Unknown".to_string()),
                            "level": player.level.unwrap_or(1)
                        }),
                    );

                    let state_str = render_template(
                        config_state,
                        &json!({
                            "hp": base_attr.hp.unwrap_or(0),
                            "max_hp": base_attr.max_hp.unwrap_or(0),
                            "fp": base_attr.fp.unwrap_or(0),
                            "max_fp": base_attr.max_fp.unwrap_or(0),
                            "sp": base_attr.stamina.unwrap_or(0),
                            "max_sp": base_attr.max_stamina.unwrap_or(0),
                        }),
                    );

                    eprintln!("{} | {}", details_str, state_str);

                    if !details_str.is_empty() || !state_str.is_empty() {
                        let activity = Activity::new().details(&details_str).state(&state_str);
                        if let Err(e) = memory.rpc.set_activity(activity) {
                            eprintln!("Failed to set RPC activity: {}", e);
                        }
                    }

                    Ok(())
                });
            }
            Err(e) => {
                if e != "Game process not attached" {
                    eprintln!("Failed to get player info: {}", e);
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    Ok(())
}
