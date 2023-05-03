// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod templates;
use std::process;
use tauri_plugin_autostart::MacosLauncher;
use templates::*;

use tauri::*;
fn make_tray() -> SystemTray {
    // <- a function that creates the system tray
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        .add_item(CustomMenuItem::new(
            "changeName".to_owned(),
            "Change Officer Name",
        ));
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
        if id.as_str() == "changeName" {
            let window = app.get_window("main").unwrap();
            let menu_item = app.tray_handle().get_item("toggle");
            if !window.is_visible().unwrap() {
                window.show().unwrap();
                window.center().unwrap();
                window.set_focus().unwrap();
                window.set_always_on_top(true).unwrap();
                menu_item.set_title("Hide").unwrap();
            }
            window.eval("localStorage.setItem('officerName', prompt('Enter your badge number and name'));").unwrap();
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
    match key {
        "speedingTicket" => return SPEEDING_TICKET.to_owned(),
        "controlDevices" => return CONTROL_DEVICES.to_owned(),
        "suspendedLicense" => return SUSPENDED_LICENSE.to_owned(),
        "negligentDriving" => return NEGLIGENT_DRIVING.to_owned(),
        "robbery" => return BANK.to_owned(),
        "boost" => return BOOST.to_owned(),
        "methRun" => return METH_RUN.to_owned(),
        "saleOfDrugs" => return SALE_OF_DRUGS.to_owned(),
        "GRS" => return GRS.to_owned(),
        "houseRobbery" => return HOUSE_ROBBERY.to_owned(),
        _ => return "couldn't find the template".to_owned(),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![hide_window, show_window, template])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
