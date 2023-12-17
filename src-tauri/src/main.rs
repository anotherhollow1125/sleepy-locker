#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sleepy_locker::sleep_prevent::{allow_sleep, prevent_sleep};
use std::sync::Mutex;

#[tauri::command]
fn set_sleep_prevent_enabled(
    prevent_sleep_enabled: tauri::State<'_, Mutex<bool>>,
    enabled: bool,
) -> Result<bool, String> {
    let mut prevent_sleep_enabled = prevent_sleep_enabled
        .lock()
        .map_err(|e| format!("failed to lock prevent_sleep_enabled: {}", e))?;
    *prevent_sleep_enabled = enabled;
    if enabled {
        prevent_sleep();
        dbg!("Sleep prevention enabled");
    } else {
        allow_sleep();
        dbg!("Sleep prevention disabled");
    }
    Ok(*prevent_sleep_enabled)
}

#[tauri::command]
fn get_sleep_prevent_enabled(
    prevent_sleep_enabled: tauri::State<'_, Mutex<bool>>,
) -> Result<bool, String> {
    let prevent_sleep_enabled = prevent_sleep_enabled
        .lock()
        .map_err(|e| format!("failed to lock prevent_sleep_enabled: {}", e))?;
    Ok(*prevent_sleep_enabled)
}

fn main() {
    let prevent_sleep_enabled = Mutex::new(false);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_sleep_prevent_enabled,
            get_sleep_prevent_enabled
        ])
        .manage(prevent_sleep_enabled)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
