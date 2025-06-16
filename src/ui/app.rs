use crate::core::{AppResult, Config, WallpaperType};
use crate::platform::WallpaperManager;
use crate::wallpapers::{AudioWallpaper, ShaderWallpaper, StaticWallpaper, VideoWallpaper, WebWallpaper, Wallpaper};
use eframe::{egui, epi};
use log::{debug, error, info};
use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Main application UI
pub struct AetherDeskApp {
    /// Application configuration
    config: Config,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Current wallpaper
    current_wallpaper: Option<Box<dyn Wallpaper + Send + Sync>>,
    
    /// Selected wallpaper type
    selected_wallpaper_type: WallpaperType,
    
    /// Selected wallpaper path
    selected_wallpaper_path: Option<PathBuf>,
    
    /// Selected web URL
    selected_web_url: String,
}

impl AetherDeskApp {
    /// Create a new application UI
    pub fn new(wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        // Load configuration
        let config = Config::load().unwrap_or_else(|e| {
            error!("Failed to load configuration: {}", e);
            Config::default()
        });
        
        Self {
            config,
            wallpaper_manager,
            current_wallpaper: None,
            selected_wallpaper_type: WallpaperType::Static,
            selected_wallpaper_path: None,
            selected_web_url: String::new(),
        }
    }
    
    /// Show the main UI
    pub fn show(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Aether-Desk");
            
            ui.separator();
            
            // Wallpaper type selection
            ui.horizontal(|ui| {
                ui.label("Wallpaper Type:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.selected_wallpaper_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_wallpaper_type, WallpaperType::Static, "Static");
                        ui.selectable_value(&mut self.selected_wallpaper_type, WallpaperType::Video, "Video");
                        ui.selectable_value(&mut self.selected_wallpaper_type, WallpaperType::Web, "Web");
                        ui.selectable_value(&mut self.selected_wallpaper_type, WallpaperType::Shader, "Shader");
                        ui.selectable_value(&mut self.selected_wallpaper_type, WallpaperType::Audio, "Audio");
                    });
            });
            
            ui.separator();
            
            // Wallpaper selection based on type
            match self.selected_wallpaper_type {
                WallpaperType::Static | WallpaperType::Video | WallpaperType::Shader | WallpaperType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label("Wallpaper Path:");
                        
                        if let Some(path) = &self.selected_wallpaper_path {
                            ui.label(path.to_string_lossy());
                        } else {
                            ui.label("No file selected");
                        }
                        
                        if ui.button("Browse...").clicked() {
                            let file_dialog = match self.selected_wallpaper_type {
                                WallpaperType::Static => {
                                    FileDialog::new()
                                        .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif"])
                                },
                                WallpaperType::Video => {
                                    FileDialog::new()
                                        .add_filter("Videos", &["mp4", "webm", "avi", "mkv"])
                                },
                                WallpaperType::Shader => {
                                    FileDialog::new()
                                        .add_filter("Shaders", &["glsl", "frag", "vert"])
                                },
                                WallpaperType::Audio => {
                                    FileDialog::new()
                                        .add_filter("Shaders", &["glsl", "frag", "vert"])
                                },
                                _ => FileDialog::new(),
                            };
                            
                            if let Some(path) = file_dialog.pick_file() {
                                self.selected_wallpaper_path = Some(path);
                            }
                        }
                    });
                },
                WallpaperType::Web => {
                    ui.horizontal(|ui| {
                        ui.label("Web URL:");
                        ui.text_edit_singleline(&mut self.selected_web_url);
                    });
                },
            }
            
            ui.separator();
            
            // Apply button
            if ui.button("Apply").clicked() {
                self.apply_wallpaper();
            }
            
            // Stop button
            if ui.button("Stop").clicked() {
                self.stop_wallpaper();
            }
        });
    }
    
    /// Apply the selected wallpaper
    fn apply_wallpaper(&mut self) {
        // Stop current wallpaper if any
        if let Some(wallpaper) = &self.current_wallpaper {
            if let Err(e) = wallpaper.stop() {
                error!("Failed to stop current wallpaper: {}", e);
            }
        }
        
        // Create and start new wallpaper
        match self.selected_wallpaper_type {
            WallpaperType::Static => {
                if let Some(path) = &self.selected_wallpaper_path {
                    let wallpaper = StaticWallpaper::new(path, self.wallpaper_manager.clone());
                    if let Err(e) = wallpaper.start() {
                        error!("Failed to start static wallpaper: {}", e);
                    } else {
                        self.current_wallpaper = Some(Box::new(wallpaper));
                        info!("Static wallpaper applied");
                    }
                }
            },
            WallpaperType::Video => {
                if let Some(path) = &self.selected_wallpaper_path {
                    let wallpaper = VideoWallpaper::new(path, self.wallpaper_manager.clone());
                    if let Err(e) = wallpaper.start() {
                        error!("Failed to start video wallpaper: {}", e);
                    } else {
                        self.current_wallpaper = Some(Box::new(wallpaper));
                        info!("Video wallpaper applied");
                    }
                }
            },
            WallpaperType::Web => {
                if !self.selected_web_url.is_empty() {
                    let wallpaper = WebWallpaper::new(&self.selected_web_url, self.wallpaper_manager.clone());
                    if let Err(e) = wallpaper.start() {
                        error!("Failed to start web wallpaper: {}", e);
                    } else {
                        self.current_wallpaper = Some(Box::new(wallpaper));
                        info!("Web wallpaper applied");
                    }
                }
            },
            WallpaperType::Shader => {
                if let Some(path) = &self.selected_wallpaper_path {
                    let wallpaper = ShaderWallpaper::new(path, self.wallpaper_manager.clone());
                    if let Err(e) = wallpaper.start() {
                        error!("Failed to start shader wallpaper: {}", e);
                    } else {
                        self.current_wallpaper = Some(Box::new(wallpaper));
                        info!("Shader wallpaper applied");
                    }
                }
            },
            WallpaperType::Audio => {
                if let Some(path) = &self.selected_wallpaper_path {
                    let wallpaper = AudioWallpaper::new(path, self.wallpaper_manager.clone());
                    if let Err(e) = wallpaper.start() {
                        error!("Failed to start audio wallpaper: {}", e);
                    } else {
                        self.current_wallpaper = Some(Box::new(wallpaper));
                        info!("Audio wallpaper applied");
                    }
                }
            },
        }
    }
    
    /// Stop the current wallpaper
    fn stop_wallpaper(&mut self) {
        if let Some(wallpaper) = &self.current_wallpaper {
            if let Err(e) = wallpaper.stop() {
                error!("Failed to stop wallpaper: {}", e);
            } else {
                self.current_wallpaper = None;
                info!("Wallpaper stopped");
            }
        }
    }
} 