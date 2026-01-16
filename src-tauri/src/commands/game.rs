use crate::memory::{AppState, Application};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::State;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachResult {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameStatus {
    pub attached: bool,
    pub process_name: String,
}

static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn attach_to_game(state: State<'_, AppState>) -> AttachResult {
    let process_name = "eldenring.exe";

    if MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return AttachResult {
            success: true,
            message: "Monitor already running".to_string(),
        };
    }

    let state_clone = state.inner().clone();

    thread::spawn(move || {
        eprintln!("Background: Waiting for {}...", process_name);

        loop {
            let mut needs_attach = false;
            {
                let mut guard = state_clone.lock().unwrap();
                if let Some(app) = guard.as_mut() {
                    if !app.check_running() {
                        eprintln!("{} closed, waiting for restart...", process_name);
                        *guard = None;
                        needs_attach = true;
                    }
                } else {
                    needs_attach = true;
                }
            }

            if !needs_attach {
                thread::sleep(Duration::from_millis(500));
                continue;
            }

            match Application::new(process_name) {
                Some(app) => {
                    let mut guard = state_clone.lock().unwrap();
                    *guard = Some(app);
                    eprintln!("Attached to {}!", process_name);
                }
                None => {}
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    AttachResult {
        success: true,
        message: format!("Started monitoring for {}", process_name),
    }
}

#[tauri::command]
pub fn get_game_status(state: State<'_, AppState>) -> GameStatus {
    let guard = state.lock().unwrap();
    let process_name = guard.as_ref().map_or("".to_string(), |r| r.process_name.clone());

    GameStatus {
        attached: guard.is_some(),
        process_name: process_name,
    }
}
