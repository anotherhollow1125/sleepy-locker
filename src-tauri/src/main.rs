#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sleepy_locker::{init_event_thread, Event, LockState};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tauri_plugin_autostart::MacosLauncher;

mod system_tray;
use system_tray::{create_systemtray, on_system_tray_event};

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
    let (tx, lock_state, close_dummy_window, dw_handle, event_handle) = init_event_thread();
    let tx_for_quit = tx.clone();

    let clean_up = Arc::new(Mutex::new(Some(move || {
        close_dummy_window();
        dw_handle.join().unwrap();
        tx_for_quit.send(Event::Quit).unwrap();
        event_handle.join().unwrap();
    })));
    let clean_up_for_system_tray = Arc::clone(&clean_up);

    tauri::Builder::default()
        .manage(tx)
        .manage(lock_state)
        .invoke_handler(tauri::generate_handler![
            set_sleep_prevent_enabled,
            get_sleep_prevent_enabled
        ])
        .system_tray(create_systemtray())
        .on_system_tray_event(on_system_tray_event(clean_up_for_system_tray))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .on_window_event(move |event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                let Some(clean_up) = clean_up.lock().unwrap().take() else {
                    return;
                };
                clean_up();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
