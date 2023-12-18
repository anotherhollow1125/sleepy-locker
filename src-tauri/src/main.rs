#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sleepy_locker::{init_control_thread, Event, LockState};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[tauri::command]
fn set_sleep_prevent_enabled(
    tx: tauri::State<'_, Sender<Event>>,
    enabled: bool,
) -> Result<(), String> {
    if enabled {
        tx.send(Event::Prevent)
            .map_err(|e| format!("failed to send Event::Prevent: {}", e))?;
    } else {
        tx.send(Event::Allow)
            .map_err(|e| format!("failed to send Event::Allow: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn get_sleep_prevent_enabled(
    lock_state: tauri::State<'_, Arc<Mutex<LockState>>>,
) -> Result<bool, String> {
    let lock_state = lock_state
        .lock()
        .map_err(|e| format!("failed to lock prevent_sleep_enabled: {}", e))?;
    Ok(lock_state.is_enabled())
}

fn main() {
    let (tx, lock_state) = init_control_thread();

    tauri::Builder::default()
        .manage(tx)
        .manage(lock_state)
        .invoke_handler(tauri::generate_handler![
            set_sleep_prevent_enabled,
            get_sleep_prevent_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
