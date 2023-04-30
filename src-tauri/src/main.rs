// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process;

use tauri::{api::file::read_string, *};
fn make_tray() -> SystemTray {
    // <- a function that creates the system tray
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    return SystemTray::new().with_menu(menu);
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            process::exit(0);
        }
        if id.as_str() == "toggle" {
            let window = app.get_window("main").unwrap();
            let menu_item = app.tray_handle().get_item("toggle");
            if window.is_visible().unwrap() {
                window.hide().unwrap();
                menu_item.set_title("Show").unwrap();
            } else {
                window.show().unwrap();
                window.center().unwrap();
                window.set_focus().unwrap();
                window.set_always_on_top(true).unwrap();
                menu_item.set_title("Hide").unwrap();
            }
        }
    }
}


#[tauri::command]
fn hide_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    let menu_item = app.tray_handle().get_item("toggle");
    window.hide().unwrap();
    menu_item.set_title("Show").unwrap();
}

#[tauri::command]
fn show_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    let menu_item = app.tray_handle().get_item("toggle");
    window.show().unwrap();
    window.center().unwrap();
    window.set_focus().unwrap();
    window.set_always_on_top(true).unwrap();
    menu_item.set_title("Hide").unwrap();
}

#[tauri::command]
fn template(key: &str) -> String {
    let path = format!("./templates/{}.txt", key);
    let text = read_string(path).unwrap();
    return text;
}

fn main() {
    tauri::Builder::default()
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![
            hide_window,
            show_window,
            template
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
