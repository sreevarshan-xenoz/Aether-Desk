#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod platform;
mod wallpapers;
mod ui;

use anyhow::Result;
use log::{error, info};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use aether_desk::core::{AppResult, Config};
use aether_desk::platform::{self, WallpaperManager};
use aether_desk::ui::AetherDeskApp;
use eframe::{egui, epi};
use std::sync::Arc;

/// Main application
struct AetherDesk {
    /// Application UI
    app: AetherDeskApp,
}

impl epi::App for AetherDesk {
    fn name(&self) -> &str {
        "Aether-Desk"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        self.app.show(ctx);
    }
}

fn main() -> AppResult<()> {
    // Initialize logger
    env_logger::init();
    info!("Starting Aether-Desk");
    
    // Create platform-specific wallpaper manager
    let wallpaper_manager = platform::create_wallpaper_manager()?;
    
    // Create application
    let app = AetherDeskApp::new(wallpaper_manager);
    
    // Run application
    let app = AetherDesk { app };
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    if let Err(e) = eframe::run_native(Box::new(app), native_options) {
        error!("Application error: {}", e);
        return Err(e.into());
    }
    
    info!("Aether-Desk stopped");
    Ok(())
} 