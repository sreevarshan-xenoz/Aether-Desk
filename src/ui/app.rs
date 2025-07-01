use crate::core::{AppResult, Config, Plugin, PluginConfig, PluginManager, ScheduleItem, TriggerType, WallpaperScheduler, Widget, WidgetConfig, WidgetManager, WidgetPosition, WidgetSize, WidgetType, WallpaperType, Theme, ThemeConfig};
use crate::platform::WallpaperManager;
use crate::wallpapers::{AudioWallpaper, ShaderWallpaper, StaticWallpaper, VideoWallpaper, WebWallpaper, Wallpaper};
use chrono::{NaiveTime, Timelike};
use eframe::{egui, epi};
use log::{debug, error, info};
use rfd::FileDialog;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Main application UI
pub struct AetherDeskApp {
    /// Application configuration
    config: Config,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Plugin manager
    plugin_manager: PluginManager,
    
    /// Wallpaper scheduler
    scheduler: WallpaperScheduler,
    
    /// Widget manager
    widget_manager: WidgetManager,
    
    /// Current wallpaper
    current_wallpaper: Option<Box<dyn Wallpaper + Send + Sync>>,
    
    /// Selected wallpaper type
    selected_wallpaper_type: WallpaperType,
    
    /// Selected wallpaper path
    selected_wallpaper_path: Option<PathBuf>,
    
    /// Selected web URL
    selected_web_url: String,
    
    /// Selected tab
    selected_tab: Tab,
    
    /// New schedule item
    new_schedule_item: Option<ScheduleItem>,
    
    /// Editing schedule item index
    editing_schedule_index: Option<usize>,
    
    /// New widget
    new_widget: Option<WidgetConfig>,
    
    /// Editing widget ID
    editing_widget_id: Option<String>,
}

/// UI tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    /// Wallpaper tab
    Wallpaper,
    
    /// Scheduler tab
    Scheduler,
    
    /// Widgets tab
    Widgets,
    
    /// Plugins tab
    Plugins,
    
    /// Settings tab
    Settings,
}

impl AetherDeskApp {
    /// Create a new application UI
    pub fn new(wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        // Load configuration
        let config = Config::load().unwrap_or_else(|e| {
            error!("Failed to load configuration: {}", e);
            Config::default()
        });
        
        // Create plugin manager
        let plugin_dir = config.get_plugin_dir();
        let mut plugin_manager = PluginManager::new(&plugin_dir);
        
        // Load plugins
        if let Err(e) = plugin_manager.load_plugins(&config) {
            error!("Failed to load plugins: {}", e);
        }
        
        // Create scheduler
        let mut scheduler = WallpaperScheduler::new(wallpaper_manager.clone());
        
        // Load schedule
        if let Err(e) = scheduler.load_schedule(&config) {
            error!("Failed to load schedule: {}", e);
        }
        
        // Start scheduler
        if let Err(e) = scheduler.start() {
            error!("Failed to start scheduler: {}", e);
        }
        
        // Create widget manager
        let mut widget_manager = WidgetManager::new();
        
        // Load widgets
        if let Err(e) = widget_manager.load_widgets(&config) {
            error!("Failed to load widgets: {}", e);
        }
        
        // Start widget manager
        if let Err(e) = widget_manager.start() {
            error!("Failed to start widget manager: {}", e);
        }
        
        Self {
            config,
            wallpaper_manager,
            plugin_manager,
            scheduler,
            widget_manager,
            current_wallpaper: None,
            selected_wallpaper_type: WallpaperType::Static,
            selected_wallpaper_path: None,
            selected_web_url: String::new(),
            selected_tab: Tab::Wallpaper,
            new_schedule_item: None,
            editing_schedule_index: None,
            new_widget: None,
            editing_widget_id: None,
        }
    }
    
    /// Show the main UI
    pub fn show(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Aether-Desk");
            
            // Tab selection
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == Tab::Wallpaper, "Wallpaper").clicked() {
                    self.selected_tab = Tab::Wallpaper;
                }
                
                if ui.selectable_label(self.selected_tab == Tab::Scheduler, "Scheduler").clicked() {
                    self.selected_tab = Tab::Scheduler;
                }
                
                if ui.selectable_label(self.selected_tab == Tab::Widgets, "Widgets").clicked() {
                    self.selected_tab = Tab::Widgets;
                }
                
                if ui.selectable_label(self.selected_tab == Tab::Plugins, "Plugins").clicked() {
                    self.selected_tab = Tab::Plugins;
                }
                
                if ui.selectable_label(self.selected_tab == Tab::Settings, "Settings").clicked() {
                    self.selected_tab = Tab::Settings;
                }
            });
            
            ui.separator();
            
            // Tab content
            match self.selected_tab {
                Tab::Wallpaper => self.show_wallpaper_tab(ui),
                Tab::Scheduler => self.show_scheduler_tab(ui),
                Tab::Widgets => self.show_widgets_tab(ui),
                Tab::Plugins => self.show_plugins_tab(ui),
                Tab::Settings => self.show_settings_tab(ui),
            }
        });
    }
    
    /// Show wallpaper tab
    fn show_wallpaper_tab(&mut self, ui: &mut egui::Ui) {
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
    }
    
    /// Show scheduler tab
    fn show_scheduler_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Wallpaper Scheduler");
        
        // Schedule items
        let schedule_items = self.scheduler.get_schedule_items();
        
        if schedule_items.is_empty() {
            ui.label("No schedule items. Add a new schedule item to automatically change wallpapers.");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, item) in schedule_items.iter().enumerate() {
                    ui.horizontal(|ui| {
                        // Enable/disable checkbox
                        let mut enabled = item.enabled;
                        if ui.checkbox(&mut enabled, "").changed() {
                            let mut updated_item = item.clone();
                            updated_item.enabled = enabled;
                            if let Err(e) = self.scheduler.update_schedule_item(index, updated_item) {
                                error!("Failed to update schedule item: {}", e);
                            }
                        }
                        
                        // Trigger type
                        ui.label(format!("{:?}", item.trigger));
                        
                        // Wallpaper name
                        ui.label(&item.wallpaper.name);
                        
                        // Edit button
                        if ui.button("Edit").clicked() {
                            self.editing_schedule_index = Some(index);
                            self.new_schedule_item = Some(item.clone());
                        }
                        
                        // Delete button
                        if ui.button("Delete").clicked() {
                            if let Err(e) = self.scheduler.remove_schedule_item(index) {
                                error!("Failed to remove schedule item: {}", e);
                            }
                        }
                    });
                }
            });
        }
        
        ui.separator();
        
        // Add new schedule item
        if ui.button("Add Schedule Item").clicked() {
            self.new_schedule_item = Some(ScheduleItem {
                trigger: TriggerType::Time(NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
                wallpaper: crate::core::WallpaperInfo {
                    name: "New Schedule".to_string(),
                    description: "New schedule item".to_string(),
                    author: "Aether-Desk".to_string(),
                    version: "1.0.0".to_string(),
                    r#type: WallpaperType::Static,
                    path: None,
                    url: None,
                },
                enabled: true,
            });
            self.editing_schedule_index = None;
        }
        
        // Edit schedule item
        if let Some(item) = &mut self.new_schedule_item {
            ui.separator();
            ui.heading(if self.editing_schedule_index.is_some() {
                "Edit Schedule Item"
            } else {
                "Add Schedule Item"
            });
            
            // Trigger type
            ui.horizontal(|ui| {
                ui.label("Trigger Type:");
                egui::ComboBox::from_label("")
                    .selected_text(match &item.trigger {
                        TriggerType::Time(_) => "Time",
                        TriggerType::Interval(_) => "Interval",
                        TriggerType::SystemEvent(_) => "System Event",
                        TriggerType::Custom(_) => "Custom",
                    })
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(matches!(item.trigger, TriggerType::Time(_)), "Time").clicked() {
                            item.trigger = TriggerType::Time(NaiveTime::from_hms_opt(8, 0, 0).unwrap());
                        }
                        if ui.selectable_label(matches!(item.trigger, TriggerType::Interval(_)), "Interval").clicked() {
                            item.trigger = TriggerType::Interval(chrono::Duration::hours(1));
                        }
                        if ui.selectable_label(matches!(item.trigger, TriggerType::SystemEvent(_)), "System Event").clicked() {
                            item.trigger = TriggerType::SystemEvent("startup".to_string());
                        }
                        if ui.selectable_label(matches!(item.trigger, TriggerType::Custom(_)), "Custom").clicked() {
                            item.trigger = TriggerType::Custom("custom".to_string());
                        }
                    });
            });
            
            // Trigger details
            match &mut item.trigger {
                TriggerType::Time(time) => {
                    ui.horizontal(|ui| {
                        ui.label("Time:");
                        let mut hour = time.hour() as u32;
                        let mut minute = time.minute() as u32;
                        
                        if ui.add(egui::DragValue::new(&mut hour).speed(1).clamp_range(0..=23)).changed() {
                            *time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
                        }
                        
                        ui.label(":");
                        
                        if ui.add(egui::DragValue::new(&mut minute).speed(1).clamp_range(0..=59)).changed() {
                            *time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
                        }
                    });
                },
                TriggerType::Interval(interval) => {
                    ui.horizontal(|ui| {
                        ui.label("Interval:");
                        let mut hours = interval.num_hours() as u32;
                        let mut minutes = (interval.num_minutes() % 60) as u32;
                        
                        if ui.add(egui::DragValue::new(&mut hours).speed(1)).changed() {
                            *interval = chrono::Duration::hours(hours as i64) + chrono::Duration::minutes(minutes as i64);
                        }
                        
                        ui.label("hours");
                        
                        if ui.add(egui::DragValue::new(&mut minutes).speed(1).clamp_range(0..=59)).changed() {
                            *interval = chrono::Duration::hours(hours as i64) + chrono::Duration::minutes(minutes as i64);
                        }
                        
                        ui.label("minutes");
                    });
                },
                TriggerType::SystemEvent(event) => {
                    ui.horizontal(|ui| {
                        ui.label("Event:");
                        ui.text_edit_singleline(event);
                    });
                },
                TriggerType::Custom(trigger) => {
                    ui.horizontal(|ui| {
                        ui.label("Trigger:");
                        ui.text_edit_singleline(trigger);
                    });
                },
            }
            
            // Wallpaper type
            ui.horizontal(|ui| {
                ui.label("Wallpaper Type:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", item.wallpaper.r#type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut item.wallpaper.r#type, WallpaperType::Static, "Static");
                        ui.selectable_value(&mut item.wallpaper.r#type, WallpaperType::Video, "Video");
                        ui.selectable_value(&mut item.wallpaper.r#type, WallpaperType::Web, "Web");
                        ui.selectable_value(&mut item.wallpaper.r#type, WallpaperType::Shader, "Shader");
                        ui.selectable_value(&mut item.wallpaper.r#type, WallpaperType::Audio, "Audio");
                    });
            });
            
            // Wallpaper selection based on type
            match item.wallpaper.r#type {
                WallpaperType::Static | WallpaperType::Video | WallpaperType::Shader | WallpaperType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label("Wallpaper Path:");
                        
                        if let Some(path) = &item.wallpaper.path {
                            ui.label(path.to_string_lossy());
                        } else {
                            ui.label("No file selected");
                        }
                        
                        if ui.button("Browse...").clicked() {
                            let file_dialog = match item.wallpaper.r#type {
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
                                item.wallpaper.path = Some(path);
                            }
                        }
                    });
                },
                WallpaperType::Web => {
                    ui.horizontal(|ui| {
                        ui.label("Web URL:");
                        let mut url = item.wallpaper.url.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut url).changed() {
                            item.wallpaper.url = Some(url);
                        }
                    });
                },
            }
            
            // Wallpaper name
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut item.wallpaper.name);
            });
            
            // Wallpaper description
            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_singleline(&mut item.wallpaper.description);
            });
            
            // Enable/disable
            ui.checkbox(&mut item.enabled, "Enabled");
            
            // Save button
            if ui.button("Save").clicked() {
                if let Some(index) = self.editing_schedule_index {
                    if let Err(e) = self.scheduler.update_schedule_item(index, item.clone()) {
                        error!("Failed to update schedule item: {}", e);
                    }
                } else {
                    if let Err(e) = self.scheduler.add_schedule_item(item.clone()) {
                        error!("Failed to add schedule item: {}", e);
                    }
                }
                
                // Save schedule
                if let Err(e) = self.scheduler.save_schedule(&self.config) {
                    error!("Failed to save schedule: {}", e);
                }
                
                self.new_schedule_item = None;
                self.editing_schedule_index = None;
            }
            
            // Cancel button
            if ui.button("Cancel").clicked() {
                self.new_schedule_item = None;
                self.editing_schedule_index = None;
            }
        }
    }
    
    /// Show widgets tab
    fn show_widgets_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Widgets");
        
        // Widget list
        let widget_configs = self.widget_manager.get_widget_configs();
        
        if widget_configs.is_empty() {
            ui.label("No widgets installed. Add a new widget to display information on your desktop.");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, config) in widget_configs.iter() {
                    ui.horizontal(|ui| {
                        // Enable/disable checkbox
                        let mut enabled = config.enabled;
                        if ui.checkbox(&mut enabled, "").changed() {
                            let mut updated_config = config.clone();
                            updated_config.enabled = enabled;
                            if let Err(e) = self.widget_manager.update_widget(id, updated_config) {
                                error!("Failed to update widget: {}", e);
                            }
                        }
                        
                        // Widget type
                        ui.label(format!("{:?}", config.widget_type));
                        
                        // Widget position
                        ui.label(format!("{:?}", config.position));
                        
                        // Widget size
                        ui.label(format!("{:?}", config.size));
                        
                        // Edit button
                        if ui.button("Edit").clicked() {
                            self.editing_widget_id = Some(id.clone());
                            self.new_widget = Some(config.clone());
                        }
                        
                        // Delete button
                        if ui.button("Delete").clicked() {
                            if let Err(e) = self.widget_manager.remove_widget(id) {
                                error!("Failed to remove widget: {}", e);
                            }
                        }
                    });
                }
            });
        }
        
        ui.separator();
        
        // Add new widget
        if ui.button("Add Widget").clicked() {
            self.new_widget = Some(WidgetConfig {
                widget_type: WidgetType::Clock,
                position: WidgetPosition::TopRight,
                size: WidgetSize::Medium,
                settings: HashMap::new(),
                enabled: true,
            });
            self.editing_widget_id = None;
        }
        
        // Edit widget
        if let Some(config) = &mut self.new_widget {
            ui.separator();
            ui.heading(if self.editing_widget_id.is_some() {
                "Edit Widget"
            } else {
                "Add Widget"
            });
            
            // Widget type
            ui.horizontal(|ui| {
                ui.label("Widget Type:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", config.widget_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.widget_type, WidgetType::Clock, "Clock");
                        ui.selectable_value(&mut config.widget_type, WidgetType::Weather, "Weather");
                        ui.selectable_value(&mut config.widget_type, WidgetType::SystemMonitor, "System Monitor");
                        ui.selectable_value(&mut config.widget_type, WidgetType::Calendar, "Calendar");
                        ui.selectable_value(&mut config.widget_type, WidgetType::Notes, "Notes");
                        ui.selectable_value(&mut config.widget_type, WidgetType::Custom("custom".to_string()), "Custom");
                    });
            });
            
            // Widget position
            ui.horizontal(|ui| {
                ui.label("Position:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", config.position))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.position, WidgetPosition::TopLeft, "Top Left");
                        ui.selectable_value(&mut config.position, WidgetPosition::TopRight, "Top Right");
                        ui.selectable_value(&mut config.position, WidgetPosition::BottomLeft, "Bottom Left");
                        ui.selectable_value(&mut config.position, WidgetPosition::BottomRight, "Bottom Right");
                        ui.selectable_value(&mut config.position, WidgetPosition::Custom(0, 0), "Custom");
                    });
            });
            
            // Widget size
            ui.horizontal(|ui| {
                ui.label("Size:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", config.size))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.size, WidgetSize::Small, "Small");
                        ui.selectable_value(&mut config.size, WidgetSize::Medium, "Medium");
                        ui.selectable_value(&mut config.size, WidgetSize::Large, "Large");
                        ui.selectable_value(&mut config.size, WidgetSize::Custom(100, 100), "Custom");
                    });
            });
            
            // Widget settings
            ui.heading("Settings");
            
            match config.widget_type {
                WidgetType::Clock => {
                    ui.horizontal(|ui| {
                        ui.label("Time Format:");
                        let mut time_format = config.settings.get("time_format").unwrap_or(&"%H:%M:%S".to_string()).clone();
                        if ui.text_edit_singleline(&mut time_format).changed() {
                            config.settings.insert("time_format".to_string(), time_format);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Date Format:");
                        let mut date_format = config.settings.get("date_format").unwrap_or(&"%Y-%m-%d".to_string()).clone();
                        if ui.text_edit_singleline(&mut date_format).changed() {
                            config.settings.insert("date_format".to_string(), date_format);
                        }
                    });
                },
                WidgetType::Weather => {
                    ui.horizontal(|ui| {
                        ui.label("API Key:");
                        let mut api_key = config.settings.get("api_key").unwrap_or(&"".to_string()).clone();
                        if ui.text_edit_singleline(&mut api_key).changed() {
                            config.settings.insert("api_key".to_string(), api_key);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Location:");
                        let mut location = config.settings.get("location").unwrap_or(&"".to_string()).clone();
                        if ui.text_edit_singleline(&mut location).changed() {
                            config.settings.insert("location".to_string(), location);
                        }
                    });
                },
                WidgetType::SystemMonitor => {
                    ui.horizontal(|ui| {
                        ui.label("Update Interval (seconds):");
                        let mut interval = config.settings.get("interval").unwrap_or(&"1".to_string()).clone();
                        if ui.text_edit_singleline(&mut interval).changed() {
                            config.settings.insert("interval".to_string(), interval);
                        }
                    });
                },
                WidgetType::Calendar => {
                    ui.horizontal(|ui| {
                        ui.label("Show Week Numbers:");
                        let mut show_week_numbers = config.settings.get("show_week_numbers").unwrap_or(&"false".to_string()).clone();
                        if ui.checkbox(&mut (show_week_numbers == "true"), "").changed() {
                            config.settings.insert("show_week_numbers".to_string(), show_week_numbers);
                        }
                    });
                },
                WidgetType::Notes => {
                    ui.horizontal(|ui| {
                        ui.label("Notes Content:");
                        let mut content = config.settings.get("content").unwrap_or(&"".to_string()).clone();
                        if ui.text_edit_multiline(&mut content).changed() {
                            config.settings.insert("content".to_string(), content);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Font Size:");
                        let mut font_size = config.settings.get("font_size").unwrap_or(&"14".to_string()).clone();
                        if ui.text_edit_singleline(&mut font_size).changed() {
                            config.settings.insert("font_size".to_string(), font_size);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Background Color:");
                        let mut bg_color = config.settings.get("bg_color").unwrap_or(&"#ffffff".to_string()).clone();
                        if ui.text_edit_singleline(&mut bg_color).changed() {
                            config.settings.insert("bg_color".to_string(), bg_color);
                        }
                    });
                },
                WidgetType::Custom(_) => {
                    ui.label("Custom widget settings are not supported in this version.");
                },
            }
            
            // Enable/disable
            ui.checkbox(&mut config.enabled, "Enabled");
            
            // Save button
            if ui.button("Save").clicked() {
                if let Some(id) = &self.editing_widget_id {
                    if let Err(e) = self.widget_manager.update_widget(id, config.clone()) {
                        error!("Failed to update widget: {}", e);
                    }
                } else {
                    // Generate a unique ID for the new widget
                    let id = format!("widget_{}", chrono::Utc::now().timestamp_millis());
                    if let Err(e) = self.widget_manager.add_widget(id, config.clone()) {
                        error!("Failed to add widget: {}", e);
                    }
                }
                
                // Save widgets
                if let Err(e) = self.widget_manager.save_widgets(&self.config) {
                    error!("Failed to save widgets: {}", e);
                }
                
                self.new_widget = None;
                self.editing_widget_id = None;
            }
            
            // Cancel button
            if ui.button("Cancel").clicked() {
                self.new_widget = None;
                self.editing_widget_id = None;
            }
        }
        
        // Widget preview
        ui.separator();
        ui.heading("Widget Preview");

        // Compute theme colors
        let (bg_color, accent_color) = {
            use crate::core::{Theme, ThemeConfig};
            let theme_config = &self.config.app.theme;
            match theme_config.theme {
                Theme::Light => (
                    egui::Color32::from_rgb(245, 245, 245),
                    egui::Color32::from_rgb(33, 150, 243),
                ),
                Theme::Dark => (
                    egui::Color32::from_rgb(32, 34, 37),
                    egui::Color32::from_rgb(0, 188, 212),
                ),
                Theme::Custom => {
                    let bg = theme_config.background_color.as_ref().and_then(|c| parse_hex_color(c)).unwrap_or(egui::Color32::from_rgb(32, 34, 37));
                    let accent = theme_config.accent_color.as_ref().and_then(|c| parse_hex_color(c)).unwrap_or(egui::Color32::from_rgb(0, 188, 212));
                    (bg, accent)
                }
            }
        };

        if let Err(e) = self.widget_manager.render_widgets(ui, bg_color, accent_color) {
            error!("Failed to render widgets: {}", e);
        }
    }
    
    /// Show plugins tab
    fn show_plugins_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Plugins");
        
        if self.plugin_manager.get_plugins().is_empty() {
            ui.label("No plugins installed. Plugins will be available in a future release.");
            return;
        }
        
        // Plugin list
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (name, plugin) in self.plugin_manager.get_plugins() {
                ui.collapsing(format!("{} v{}", name, plugin.metadata().version), |ui| {
                    ui.label(format!("Author: {}", plugin.metadata().author));
                    ui.label(format!("Description: {}", plugin.metadata().description));
                    
                    if let Some(homepage) = &plugin.metadata().homepage {
                        ui.hyperlink_to("Homepage", homepage);
                    }
                    
                    if let Some(license) = &plugin.metadata().license {
                        ui.label(format!("License: {}", license));
                    }
                    
                    ui.separator();
                    
                    // Plugin settings
                    ui.heading("Settings");
                    
                    let config = plugin.get_settings();
                    let mut enabled = config.enabled;
                    
                    if ui.checkbox(&mut enabled, "Enabled").changed() {
                        if enabled {
                            if let Err(e) = self.plugin_manager.enable_plugin(name) {
                                error!("Failed to enable plugin: {}", e);
                            }
                        } else {
                            if let Err(e) = self.plugin_manager.disable_plugin(name) {
                                error!("Failed to disable plugin: {}", e);
                            }
                        }
                    }
                    
                    // TODO: Add more plugin settings
                });
            }
        });
    }
    
    /// Show settings tab
    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        
        // General settings
        ui.collapsing("General", |ui| {
            // TODO: Add general settings
            ui.label("General settings will be available in a future release.");
        });
        
        // Wallpaper settings
        ui.collapsing("Wallpaper", |ui| {
            // TODO: Add wallpaper settings
            ui.label("Wallpaper settings will be available in a future release.");
        });
        
        // Plugin settings
        ui.collapsing("Plugins", |ui| {
            // TODO: Add plugin settings
            ui.label("Plugin settings will be available in a future release.");
        });
        
        // Theme settings
        ui.collapsing("Theme", |ui| {
            let theme_config = &mut self.config.app.theme;
            let mut selected_theme = theme_config.theme.clone();

            ui.horizontal(|ui| {
                ui.label("Theme:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", selected_theme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected_theme, Theme::Light, "Light");
                        ui.selectable_value(&mut selected_theme, Theme::Dark, "Dark");
                        ui.selectable_value(&mut selected_theme, Theme::Custom, "Custom");
                    });
            });

            if selected_theme != theme_config.theme {
                theme_config.theme = selected_theme.clone();
                if let Err(e) = self.config.save() {
                    error!("Failed to save config: {}", e);
                }
            }

            if selected_theme == Theme::Custom {
                ui.horizontal(|ui| {
                    ui.label("Accent Color (hex):");
                    let mut accent = theme_config.accent_color.clone().unwrap_or("#00bcd4".to_string());
                    if ui.text_edit_singleline(&mut accent).changed() {
                        theme_config.accent_color = Some(accent);
                        if let Err(e) = self.config.save() {
                            error!("Failed to save config: {}", e);
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Background Color (hex):");
                    let mut bg = theme_config.background_color.clone().unwrap_or("#181818".to_string());
                    if ui.text_edit_singleline(&mut bg).changed() {
                        theme_config.background_color = Some(bg);
                        if let Err(e) = self.config.save() {
                            error!("Failed to save config: {}", e);
                        }
                    }
                });
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

// Helper function to parse hex color
fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some(egui::Color32::from_rgb(r, g, b))
    } else {
        None
    }
} 