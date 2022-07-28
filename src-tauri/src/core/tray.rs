
use tauri::{
    AppHandle,
    CustomMenuItem,
    SystemTray,
    SystemTrayEvent,
    SystemTrayMenu,
    SystemTrayMenuItem, Manager
};
use anyhow::Result;
use crate::win32::app_bar;

pub const TRAY_QUIT: &str = "quit";
pub const TRAY_HIDE_ALL: &str = "hide_all";
pub const TRAY_SHOW_ALL: &str = "show_all";

pub fn build_tray() -> SystemTray {
    let quit = CustomMenuItem::new(TRAY_QUIT, "Quit");
    let mut hide = CustomMenuItem::new(TRAY_HIDE_ALL, "Hide All");
    let mut show = CustomMenuItem::new(TRAY_SHOW_ALL, "Show All");

    hide.enabled = false;
    show.enabled = false;

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_event_handler(app: &AppHandle, event: SystemTrayEvent) -> () {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            println!("[Tray] Menu Item '{}' clicked.", id.as_str());

            if let Err(error) = handle_menu_item_click(id.clone(), app) {
                println!("[Tray] Error handling click for Menu Item '{}': {:?}", id.as_str(), error);
            }
        }
        _ => {}
    }
}

fn handle_menu_item_click(menu_id: String, app: &AppHandle) -> Result<()> {
    let windows = app.windows();
    let tray_handle = app.tray_handle();

    match menu_id.as_str() {
        TRAY_QUIT => {
            for (_label, window) in windows {
                app_bar::ab_remove(&window)?;
            }

            println!("\nExiting yasb. Goodbye :)");
            std::process::exit(0);
        }

        TRAY_HIDE_ALL => {
            for (_label, window) in windows {
                window.hide()?;
            }

            tray_handle.get_item(TRAY_HIDE_ALL).set_enabled(false)?;
            tray_handle.get_item(TRAY_SHOW_ALL).set_enabled(true)?;
        },

        TRAY_SHOW_ALL => {
            for (_label, window) in windows {
                window.show()?;
            }

            tray_handle.get_item(TRAY_SHOW_ALL).set_enabled(false)?;
            tray_handle.get_item(TRAY_HIDE_ALL).set_enabled(true)?;
        }
        _ => {}
    };

    Ok(())
}