// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
static mut HIDE_TOGGLE: bool = false;
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let connect = CustomMenuItem::new("connect".to_string(), "Connect");
    let disconnect = CustomMenuItem::new("disconnect".to_string(), "Disconnect");
    let hide = CustomMenuItem::new("visibility".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(connect)
        .add_item(disconnect)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(on_system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "visibility" => match unsafe { HIDE_TOGGLE } {
                    true => {
                        unsafe {
                            HIDE_TOGGLE = false;
                        }
                        item_handle.set_title("Show").unwrap();
                        Command::new("sh")
                            .arg("-c")
                            .arg("pkill warp-gui-app &")
                            .spawn()
                            .expect("Cli error");
                    }
                    false => {
                        item_handle.set_title("Hide").unwrap();
                        unsafe {
                            HIDE_TOGGLE = true;
                        }
                        Command::new("sh")
                            .arg("-c")
                            .arg("$HOME/.cargo/bin/warp-gui-app &")
                            .spawn()
                            .expect("Cli error");
                    }
                },
                "quit" => {
                    Command::new("sh")
                        .arg("-c")
                        .arg("pkill warp-gui-app &")
                        .spawn()
                        .expect("Cli error");
                    app.exit(0);
                }
                "connect" => {
                    warp_gui_app::toggle_connection(true);
                }
                "disconnect" => {
                    warp_gui_app::toggle_connection(false);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
