//! Gallery view for wallpapers
use crate::core::WallpaperType;
use crate::platform::WallpaperManager;
use crate::wallpapers::{AudioWallpaper, ShaderWallpaper, StaticWallpaper, VideoWallpaper, WebWallpaper, Wallpaper};
use eframe::egui;
use log::{error, info};
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Gallery view for browsing and selecting wallpapers
pub struct GalleryView {
    /// Available wallpapers
    wallpapers: Vec<GalleryItem>,
    /// Selected wallpaper index
    selected_index: Option<usize>,
    /// Runtime for async operations
    runtime: Arc<Runtime>,
    /// Wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
}

/// Information about a wallpaper in the gallery
#[derive(Debug, Clone)]
pub struct GalleryItem {
    /// Name of the wallpaper
    pub name: String,
    /// Description of the wallpaper
    pub description: String,
    /// Path to the wallpaper file
    pub path: Option<PathBuf>,
    /// URL for web wallpapers
    pub url: Option<String>,
    /// Type of wallpaper
    pub wallpaper_type: WallpaperType,
    /// Thumbnail path (if available)
    pub thumbnail_path: Option<PathBuf>,
    /// Author of the wallpaper
    pub author: String,
    /// Version of the wallpaper
    pub version: String,
}

impl GalleryItem {
    /// Create a new gallery item from a path
    pub fn from_path(path: PathBuf, wallpaper_type: WallpaperType) -> Self {
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let description = format!("{} wallpaper ({})", wallpaper_type.as_str(), extension);
        
        Self {
            name,
            description,
            path: Some(path),
            url: None,
            wallpaper_type,
            thumbnail_path: None, // Would be generated in a real implementation
            author: "Unknown".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    /// Create a new gallery item from a URL
    pub fn from_url(url: String, wallpaper_type: WallpaperType) -> Self {
        Self {
            name: url.clone(),
            description: format!("Web wallpaper: {}", url),
            path: None,
            url: Some(url),
            wallpaper_type,
            thumbnail_path: None,
            author: "Unknown".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl GalleryView {
    /// Create a new gallery view
    pub fn new(wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            wallpapers: Vec::new(),
            selected_index: None,
            runtime: Arc::new(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime")
            ),
            wallpaper_manager,
        }
    }
    
    /// Load wallpapers from a directory
    pub fn load_from_directory(&mut self, directory: &PathBuf, wallpaper_type: WallpaperType) {
        if let Ok(entries) = std::fs::read_dir(directory) {
            for entry in entries.flatten() {
                if let Some(file_type) = entry.file_type().ok() {
                    if file_type.is_file() {
                        let path = entry.path();
                        
                        // Check if the file extension matches the wallpaper type
                        if self.is_valid_extension(&path, &wallpaper_type) {
                            let gallery_item = GalleryItem::from_path(path, wallpaper_type.clone());
                            self.wallpapers.push(gallery_item);
                        }
                    }
                }
            }
        }
    }
    
    /// Check if a file has a valid extension for the wallpaper type
    fn is_valid_extension(&self, path: &PathBuf, wallpaper_type: &WallpaperType) -> bool {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        match wallpaper_type {
            WallpaperType::Static => {
                extension == "png" || extension == "jpg" || extension == "jpeg" || 
                extension == "bmp" || extension == "gif"
            },
            WallpaperType::Video => {
                extension == "mp4" || extension == "webm" || extension == "avi" || 
                extension == "mkv" || extension == "mov" || extension == "wmv"
            },
            WallpaperType::Web => {
                // Web wallpapers are typically URLs, not files
                false
            },
            WallpaperType::Shader => {
                extension == "glsl" || extension == "frag" || extension == "vert" || 
                extension == "shader"
            },
            WallpaperType::Audio => {
                extension == "glsl" || extension == "frag" || extension == "vert" || 
                extension == "shader"
            },
        }
    }
    
    /// Add a wallpaper to the gallery
    pub fn add_wallpaper(&mut self, item: GalleryItem) {
        self.wallpapers.push(item);
    }
    
    /// Remove a wallpaper from the gallery
    pub fn remove_wallpaper(&mut self, index: usize) -> Option<GalleryItem> {
        if index < self.wallpapers.len() {
            Some(self.wallpapers.remove(index))
        } else {
            None
        }
    }
    
    /// Get the selected wallpaper
    pub fn get_selected_wallpaper(&self) -> Option<&GalleryItem> {
        if let Some(index) = self.selected_index {
            self.wallpapers.get(index)
        } else {
            None
        }
    }
    
    /// Set the selected wallpaper by index
    pub fn select_wallpaper(&mut self, index: usize) {
        if index < self.wallpapers.len() {
            self.selected_index = Some(index);
        } else {
            self.selected_index = None;
        }
    }
    
    /// Apply the selected wallpaper
    pub fn apply_selected_wallpaper(&self) -> Result<(), String> {
        if let Some(index) = self.selected_index {
            if let Some(item) = self.wallpapers.get(index) {
                // Create and start the appropriate wallpaper type
                let result = match item.wallpaper_type {
                    WallpaperType::Static => {
                        if let Some(path) = &item.path {
                            let wallpaper = StaticWallpaper::new(path, self.wallpaper_manager.clone());
                            self.runtime.block_on(async {
                                wallpaper.start().await
                            })
                        } else {
                            return Err("Static wallpaper requires a path".to_string());
                        }
                    },
                    WallpaperType::Video => {
                        if let Some(path) = &item.path {
                            let wallpaper = VideoWallpaper::new(path, self.wallpaper_manager.clone());
                            self.runtime.block_on(async {
                                wallpaper.start().await
                            })
                        } else {
                            return Err("Video wallpaper requires a path".to_string());
                        }
                    },
                    WallpaperType::Web => {
                        if let Some(url) = &item.url {
                            let wallpaper = WebWallpaper::new(url, self.wallpaper_manager.clone());
                            self.runtime.block_on(async {
                                wallpaper.start().await
                            })
                        } else {
                            return Err("Web wallpaper requires a URL".to_string());
                        }
                    },
                    WallpaperType::Shader => {
                        if let Some(path) = &item.path {
                            let wallpaper = ShaderWallpaper::new(path, self.wallpaper_manager.clone());
                            self.runtime.block_on(async {
                                wallpaper.start().await
                            })
                        } else {
                            return Err("Shader wallpaper requires a path".to_string());
                        }
                    },
                    WallpaperType::Audio => {
                        if let Some(path) = &item.path {
                            let wallpaper = AudioWallpaper::new(path, self.wallpaper_manager.clone());
                            self.runtime.block_on(async {
                                wallpaper.start().await
                            })
                        } else {
                            return Err("Audio wallpaper requires a path".to_string());
                        }
                    },
                };
                
                match result {
                    Ok(_) => {
                        info!("Applied wallpaper: {}", item.name);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to apply wallpaper: {}", e);
                        Err(e.to_string())
                    }
                }
            } else {
                Err("Selected wallpaper not found".to_string())
            }
        } else {
            Err("No wallpaper selected".to_string())
        }
    }
    
    /// Show the gallery view in the UI
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Wallpaper Gallery");
        
        // Controls
        ui.horizontal(|ui| {
            if ui.button("Refresh Gallery").clicked() {
                // In a real implementation, this would reload from configured directories
                info!("Gallery refresh requested");
            }
            
            if ui.button("Add Wallpaper").clicked() {
                // Open file dialog to add a wallpaper
                if let Some(path) = FileDialog::new().pick_file() {
                    // Determine wallpaper type based on extension
                    let wallpaper_type = self.determine_wallpaper_type(&path);
                    
                    if wallpaper_type != WallpaperType::Web {
                        let gallery_item = GalleryItem::from_path(path, wallpaper_type);
                        self.add_wallpaper(gallery_item);
                    }
                }
            }
            
            if let Some(_) = self.get_selected_wallpaper() {
                if ui.button("Apply Selected").clicked() {
                    if let Err(e) = self.apply_selected_wallpaper() {
                        ui.label(egui::RichText::new(format!("Error: {}", e)).color(egui::Color32::RED));
                    }
                }
            }
        });
        
        ui.separator();
        
        // Gallery grid
        let item_size = egui::Vec2::new(150.0, 200.0);
        let spacing = egui::Vec2::new(10.0, 10.0);
        
        // Calculate how many items fit in a row
        let available_width = ui.available_width();
        let item_width_with_spacing = item_size.x + spacing.x;
        let items_per_row = (available_width / item_width_with_spacing).floor() as usize;
        let items_per_row = items_per_row.max(1); // At least 1 item per row
        
        // Create a grid
        let mut clicked_index = None;

        egui::Grid::new("wallpaper_gallery")
            .num_columns(items_per_row)
            .spacing(spacing)
            .show(ui, |ui| {
                for (index, item) in self.wallpapers.iter().enumerate() {
                    ui.group(|ui| {
                        // Calculate aspect ratio for thumbnail
                        let aspect_ratio = 1.0; // Square thumbnails for now

                        // Create a square area for the thumbnail
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(item_size.x, item_size.x * aspect_ratio),
                            egui::Sense::click()
                        );

                        // Draw a placeholder for the thumbnail
                        painter.rect_filled(
                            response.rect,
                            egui::Rounding::same(4.0),
                            ui.visuals().extreme_bg_color
                        );

                        // Draw a symbol representing the wallpaper type
                        let text = match item.wallpaper_type {
                            WallpaperType::Static => "🖼️",
                            WallpaperType::Video => "🎬",
                            WallpaperType::Web => "🌐",
                            WallpaperType::Shader => "🎨",
                            WallpaperType::Audio => "🎵",
                        };

                        painter.text(
                            response.rect.center(),
                            egui::Align2::CENTER_CENTER,
                            text,
                            egui::TextStyle::Heading.resolve(&ui.style()),
                            ui.visuals().text_color()
                        );

                        // Handle selection
                        if response.clicked() {
                            clicked_index = Some(index);
                        }

                        // Draw selection border if selected
                        if self.selected_index == Some(index) {
                            painter.rect_stroke(
                                response.rect,
                                egui::Rounding::same(4.0),
                                egui::Stroke::new(2.0, ui.visuals().selection.stroke.color)
                            );
                        }

                        // Draw item info
                        ui.label(egui::RichText::new(&item.name).strong());

                        // Truncate description to fit
                        let desc = if item.description.len() > 50 {
                            format!("{}...", &item.description[..50])
                        } else {
                            item.description.clone()
                        };

                        ui.label(egui::RichText::new(desc).size(10.0));

                        // Show type badge
                        let type_text = match item.wallpaper_type {
                            WallpaperType::Static => "Static",
                            WallpaperType::Video => "Video",
                            WallpaperType::Web => "Web",
                            WallpaperType::Shader => "Shader",
                            WallpaperType::Audio => "Audio",
                        };

                        ui.label(egui::RichText::new(type_text)
                            .monospace()
                            .color(egui::Color32::WHITE)
                        );
                    });

                    // Move to next column, add row break if needed
                    if (index + 1) % items_per_row != 0 {
                        ui.end_row();
                    }
                }
            });

        // Handle any clicks after the UI is drawn
        if let Some(index) = clicked_index {
            self.select_wallpaper(index);
        }
        
        // Show details of selected wallpaper
        if let Some(item) = self.get_selected_wallpaper() {
            ui.separator();
            ui.heading("Selected Wallpaper Details");
            
            ui.label(format!("Name: {}", item.name));
            ui.label(format!("Type: {:?}", item.wallpaper_type));
            ui.label(format!("Description: {}", item.description));
            ui.label(format!("Author: {}", item.author));
            ui.label(format!("Version: {}", item.version));
            
            if let Some(path) = &item.path {
                ui.label(format!("Path: {}", path.display()));
            }
            
            if let Some(url) = &item.url {
                ui.label(format!("URL: {}", url));
            }
        }
    }
    
    /// Determine wallpaper type based on file extension
    fn determine_wallpaper_type(&self, path: &PathBuf) -> WallpaperType {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        match extension.as_str() {
            "png" | "jpg" | "jpeg" | "bmp" | "gif" => WallpaperType::Static,
            "mp4" | "webm" | "avi" | "mkv" | "mov" | "wmv" => WallpaperType::Video,
            "glsl" | "frag" | "vert" | "shader" => WallpaperType::Shader,
            _ => WallpaperType::Static, // Default fallback
        }
    }
}

impl WallpaperType {
    /// Get a string representation of the wallpaper type
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperType::Static => "Static",
            WallpaperType::Video => "Video",
            WallpaperType::Web => "Web",
            WallpaperType::Shader => "Shader",
            WallpaperType::Audio => "Audio",
        }
    }
}