#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod platform;
mod wallpapers;
mod ui;

use anyhow::Result;
use log::{error, info};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Aether-Desk...");

    // Create system tray menu
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Build and run the application
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    info!("Quitting application...");
                    app.exit(0);
                }
                "settings" => {
                    info!("Opening settings...");
                    // TODO: Open settings window
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            info!("Application setup...");
            
            // Initialize platform-specific wallpaper manager
            #[cfg(target_os = "windows")]
            platform::windows::init()?;
            
            #[cfg(target_os = "linux")]
            platform::linux::init()?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Add command handlers here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
} 