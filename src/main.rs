#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod platform;
mod wallpapers;
mod ui;

use anyhow::Result;
use log::{error, info};
use ui::AetherDeskApp;
use eframe::egui;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    info!("Starting Aether-Desk");
    
    // Create wallpaper manager
    let wallpaper_manager = platform::create_wallpaper_manager()?;
    
    // Create application UI
    let app = AetherDeskApp::new(wallpaper_manager);
    
    // Run application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    if let Err(e) = eframe::run_native(
        "Aether-Desk",
        options,
        Box::new(|cc| Box::new(app))
    ) {
        error!("Failed to run application: {}", e);
        return Err(e.into());
    }
    
    info!("Aether-Desk stopped");
    Ok(())
}
