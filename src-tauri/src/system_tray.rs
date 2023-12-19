use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn create_systemtray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tray
}

pub fn on_system_tray_event(
    clean_up: Arc<Mutex<Option<impl FnOnce() -> ()>>>,
) -> impl Fn(&AppHandle, SystemTrayEvent) -> () {
    move |app: &AppHandle, event: SystemTrayEvent| match event {
        SystemTrayEvent::LeftClick { .. } => {
            let window = app.get_window("main").unwrap();
            if let Ok(true) = window.is_visible() {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { ref id, .. } if id == "quit" => {
            let Some(clean_up) = clean_up.lock().unwrap().take() else {
                return;
            };
            clean_up();
            app.exit(0);
        }
        _ => {}
    }
}
