use crate::core::{AppError, AppResult, Config, WallpaperInfo};
use crate::platform::WallpaperManager;
use chrono::{DateTime, Local};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration as StdDuration, Instant};

/// Widget type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetType {
    /// Clock widget
    Clock,
    
    /// Weather widget
    Weather,
    
    /// System monitor widget
    SystemMonitor,
    
    /// Calendar widget
    Calendar,
    
    /// Notes widget
    Notes,
    
    /// Custom widget
    Custom(String),
}

/// Widget position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetPosition {
    /// Top left
    TopLeft,
    
    /// Top right
    TopRight,
    
    /// Bottom left
    BottomLeft,
    
    /// Bottom right
    BottomRight,
    
    /// Custom position (x, y)
    Custom(i32, i32),
}

/// Widget size
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetSize {
    /// Small size
    Small,
    
    /// Medium size
    Medium,
    
    /// Large size
    Large,
    
    /// Custom size (width, height)
    Custom(u32, u32),
}

/// Widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// Widget type
    pub widget_type: WidgetType,
    
    /// Widget position
    pub position: WidgetPosition,
    
    /// Widget size
    pub size: WidgetSize,
    
    /// Widget settings
    pub settings: HashMap<String, String>,
    
    /// Whether the widget is enabled
    pub enabled: bool,
    
    /// Widget background color (RGBA)
    pub background_color: Option<[u8; 4]>,
    
    /// Widget opacity (0.0 to 1.0)
    pub opacity: Option<f32>,
}

/// Widget trait
pub trait Widget: Send + Sync {
    /// Get widget type
    fn get_type(&self) -> WidgetType;
    
    /// Get widget name
    fn get_name(&self) -> String;
    
    /// Get widget description
    fn get_description(&self) -> String;
    
    /// Get widget settings
    fn get_settings(&self) -> HashMap<String, String>;
    
    /// Update widget settings
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()>;
    
    /// Render widget
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()>;
    
    /// Update widget
    fn update(&mut self) -> AppResult<()>;
}

/// Widget manager
pub struct WidgetManager {
    /// Widgets
    widgets: Arc<Mutex<Vec<Box<dyn Widget>>>>,
    
    /// Widget configurations
    widget_configs: Arc<Mutex<HashMap<String, WidgetConfig>>>,
    
    /// Widget update thread handle
    update_thread: Option<thread::JoinHandle<()>>,
    
    /// Whether the widget manager is running
    is_running: Arc<Mutex<bool>>,
}

impl WidgetManager {
    /// Create a new widget manager
    pub fn new() -> Self {
        Self {
            widgets: Arc::new(Mutex::new(Vec::new())),
            widget_configs: Arc::new(Mutex::new(HashMap::new())),
            update_thread: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Load widget configurations
    pub fn load_widgets(&mut self, config: &Config) -> AppResult<()> {
        let widgets_file = config.get_widgets_file();
        
        if !widgets_file.exists() {
            debug!("Widgets file does not exist, creating default widgets");
            self.create_default_widgets(&widgets_file)?;
            return Ok(());
        }
        
        let widgets_content = std::fs::read_to_string(&widgets_file)
            .map_err(|e| AppError::ConfigError(format!("Failed to read widgets file: {}", e)))?;
        
        let widget_configs: HashMap<String, WidgetConfig> = serde_json::from_str(&widgets_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse widgets file: {}", e)))?;
        
        let mut configs = self.widget_configs.lock().unwrap();
        *configs = widget_configs;
        
        // Create widgets from configurations
        self.create_widgets_from_configs()?;
        
        info!("Loaded {} widget configurations", configs.len());
        Ok(())
    }
    
    /// Save widget configurations
    pub fn save_widgets(&self, config: &Config) -> AppResult<()> {
        let widgets_file = config.get_widgets_file();
        let configs = self.widget_configs.lock().unwrap();
        
        let widgets_content = serde_json::to_string_pretty(&*configs)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize widgets: {}", e)))?;
        
        std::fs::write(&widgets_file, widgets_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write widgets file: {}", e)))?;
        
        info!("Saved {} widget configurations", configs.len());
        Ok(())
    }
    
    /// Create default widgets
    fn create_default_widgets(&self, widgets_file: &Path) -> AppResult<()> {
        let default_configs = vec![
            (
                "clock".to_string(),
                WidgetConfig {
                    widget_type: WidgetType::Clock,
                    position: WidgetPosition::TopRight,
                    size: WidgetSize::Medium,
                    settings: HashMap::new(),
                    enabled: true,
                    background_color: None,
                    opacity: None,
                },
            ),
            (
                "weather".to_string(),
                WidgetConfig {
                    widget_type: WidgetType::Weather,
                    position: WidgetPosition::TopRight,
                    size: WidgetSize::Medium,
                    settings: HashMap::new(),
                    enabled: true,
                    background_color: None,
                    opacity: None,
                },
            ),
            (
                "notes".to_string(),
                WidgetConfig {
                    widget_type: WidgetType::Notes,
                    position: WidgetPosition::BottomRight,
                    size: WidgetSize::Medium,
                    settings: {
                        let mut settings = HashMap::new();
                        settings.insert("content".to_string(), "Welcome to Aether-Desk!\n\nThis is a notes widget. You can edit this text to keep notes on your desktop.".to_string());
                        settings.insert("font_size".to_string(), "14".to_string());
                        settings.insert("bg_color".to_string(), "#f0f0f0".to_string());
                        settings
                    },
                    enabled: true,
                    background_color: None,
                    opacity: None,
                },
            ),
        ];
        
        let default_configs_map: HashMap<String, WidgetConfig> = default_configs.into_iter().collect();
        
        let widgets_content = serde_json::to_string_pretty(&default_configs_map)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize default widgets: {}", e)))?;
        
        std::fs::write(widgets_file, widgets_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write default widgets file: {}", e)))?;
        
        let mut configs = self.widget_configs.lock().unwrap();
        *configs = default_configs_map;
        
        info!("Created default widgets");
        Ok(())
    }
    
    /// Create widgets from configurations
    fn create_widgets_from_configs(&mut self) -> AppResult<()> {
        let configs = self.widget_configs.lock().unwrap();
        let mut widgets = self.widgets.lock().unwrap();
        
        widgets.clear();
        
        for (id, config) in configs.iter() {
            if !config.enabled {
                continue;
            }
            
            let widget: Box<dyn Widget> = match config.widget_type {
                WidgetType::Clock => {
                    Box::new(ClockWidget::new(config.settings.clone()))
                },
                WidgetType::Weather => {
                    Box::new(WeatherWidget::new(config.settings.clone()))
                },
                WidgetType::SystemMonitor => {
                    Box::new(SystemMonitorWidget::new(config.settings.clone()))
                },
                WidgetType::Calendar => {
                    Box::new(CalendarWidget::new(config.settings.clone()))
                },
                WidgetType::Notes => {
                    Box::new(NotesWidget::new(config.settings.clone()))
                },
                WidgetType::Custom(ref widget_type) => {
                    // Custom widgets are not implemented in this version
                    debug!("Custom widget not implemented: {}", widget_type);
                    continue;
                },
            };
            
            widgets.push(widget);
        }
        
        info!("Created {} widgets", widgets.len());
        Ok(())
    }
    
    /// Start the widget manager
    pub fn start(&mut self) -> AppResult<()> {
        let is_running = *self.is_running.lock().unwrap();
        if is_running {
            debug!("Widget manager is already running");
            return Ok(());
        }
        
        *self.is_running.lock().unwrap() = true;
        
        let widgets = self.widgets.clone();
        let is_running = self.is_running.clone();
        
        self.update_thread = Some(thread::spawn(move || {
            let update_interval = StdDuration::from_secs(1); // Update every second
            
            while *is_running.lock().unwrap() {
                let widgets = widgets.lock().unwrap();
                for widget in widgets.iter() {
                    if let Err(e) = widget.update() {
                        error!("Failed to update widget: {}", e);
                    }
                }
                
                thread::sleep(update_interval);
            }
        }));
        
        info!("Widget manager started");
        Ok(())
    }
    
    /// Stop the widget manager
    pub fn stop(&mut self) -> AppResult<()> {
        let is_running = *self.is_running.lock().unwrap();
        if !is_running {
            debug!("Widget manager is not running");
            return Ok(());
        }
        
        *self.is_running.lock().unwrap() = false;
        
        if let Some(thread) = self.update_thread.take() {
            thread.join().map_err(|e| {
                AppError::Other(format!("Failed to join widget update thread: {:?}", e))
            })?;
        }
        
        info!("Widget manager stopped");
        Ok(())
    }
    
    /// Add a widget
    pub fn add_widget(&mut self, id: String, config: WidgetConfig) -> AppResult<()> {
        let mut configs = self.widget_configs.lock().unwrap();
        configs.insert(id, config);
        
        // Recreate widgets
        self.create_widgets_from_configs()?;
        
        info!("Added widget");
        Ok(())
    }
    
    /// Remove a widget
    pub fn remove_widget(&mut self, id: &str) -> AppResult<()> {
        let mut configs = self.widget_configs.lock().unwrap();
        configs.remove(id);
        
        // Recreate widgets
        self.create_widgets_from_configs()?;
        
        info!("Removed widget");
        Ok(())
    }
    
    /// Update a widget
    pub fn update_widget(&mut self, id: &str, config: WidgetConfig) -> AppResult<()> {
        let mut configs = self.widget_configs.lock().unwrap();
        configs.insert(id.to_string(), config);
        
        // Recreate widgets
        self.create_widgets_from_configs()?;
        
        info!("Updated widget");
        Ok(())
    }
    
    /// Get all widget configurations
    pub fn get_widget_configs(&self) -> HashMap<String, WidgetConfig> {
        let configs = self.widget_configs.lock().unwrap();
        configs.clone()
    }
    
    /// Get all widgets
    pub fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        let widgets = self.widgets.lock().unwrap();
        widgets.clone()
    }
    
    /// Render all widgets
    pub fn render_widgets(&self, ui: &mut egui::Ui) -> AppResult<()> {
        let widgets = self.widgets.lock().unwrap();
        let configs = self.widget_configs.lock().unwrap();
        
        for widget in widgets.iter() {
            let widget_type = widget.get_type();
            let widget_name = widget.get_name();
            
            // Find the configuration for this widget
            let config = configs.iter().find(|(_, c)| c.widget_type == widget_type);
            
            if let Some((_, config)) = config {
                if !config.enabled {
                    continue;
                }
                
                // Create a frame for the widget
                let frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 0))
                    .rounding(5.0)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 50)));
                
                frame.show(ui, |ui| {
                    ui.heading(&widget_name);
                    if let Err(e) = widget.render(ui) {
                        error!("Failed to render widget: {}", e);
                    }
                });
            }
        }
        
        Ok(())
    }
}

/// Clock widget
pub struct ClockWidget {
    /// Widget settings
    settings: HashMap<String, String>,
}

impl ClockWidget {
    /// Create a new clock widget
    pub fn new(settings: HashMap<String, String>) -> Self {
        Self { settings }
    }
}

impl Widget for ClockWidget {
    fn get_type(&self) -> WidgetType {
        WidgetType::Clock
    }
    
    fn get_name(&self) -> String {
        "Clock".to_string()
    }
    
    fn get_description(&self) -> String {
        "Displays the current time".to_string()
    }
    
    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
    
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()> {
        self.settings = settings;
        Ok(())
    }
    
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()> {
        let now = Local::now();
        let time_format = self.settings.get("time_format").unwrap_or(&"%H:%M:%S".to_string());
        let date_format = self.settings.get("date_format").unwrap_or(&"%Y-%m-%d".to_string());
        
        let time_str = now.format(time_format).to_string();
        let date_str = now.format(date_format).to_string();
        
        ui.horizontal(|ui| {
            ui.label(&time_str);
            ui.label(&date_str);
        });
        
        Ok(())
    }
    
    fn update(&mut self) -> AppResult<()> {
        // Nothing to update
        Ok(())
    }
}

/// Weather widget
pub struct WeatherWidget {
    /// Widget settings
    settings: HashMap<String, String>,
    
    /// Current weather data
    weather_data: Option<WeatherData>,
}

/// Weather data
#[derive(Debug, Clone)]
struct WeatherData {
    /// Temperature in Celsius
    temperature: f32,
    
    /// Weather condition
    condition: String,
    
    /// Weather icon
    icon: String,
}

impl WeatherWidget {
    /// Create a new weather widget
    pub fn new(settings: HashMap<String, String>) -> Self {
        Self {
            settings,
            weather_data: None,
        }
    }
}

impl Widget for WeatherWidget {
    fn get_type(&self) -> WidgetType {
        WidgetType::Weather
    }
    
    fn get_name(&self) -> String {
        "Weather".to_string()
    }
    
    fn get_description(&self) -> String {
        "Displays the current weather".to_string()
    }
    
    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
    
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()> {
        self.settings = settings;
        Ok(())
    }
    
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()> {
        if let Some(weather) = &self.weather_data {
            ui.horizontal(|ui| {
                ui.label(format!("{}°C", weather.temperature));
                ui.label(&weather.condition);
            });
        } else {
            ui.label("Weather data not available");
        }
        
        Ok(())
    }
    
    fn update(&mut self) -> AppResult<()> {
        // In a real implementation, this would fetch weather data from an API
        // For now, we'll just use dummy data
        self.weather_data = Some(WeatherData {
            temperature: 22.5,
            condition: "Sunny".to_string(),
            icon: "☀️".to_string(),
        });
        
        Ok(())
    }
}

/// System monitor widget
pub struct SystemMonitorWidget {
    /// Widget settings
    settings: HashMap<String, String>,
    
    /// System data
    system_data: SystemData,
}

/// System data
#[derive(Debug, Clone)]
struct SystemData {
    /// CPU usage in percent
    cpu_usage: f32,
    
    /// Memory usage in percent
    memory_usage: f32,
    
    /// Disk usage in percent
    disk_usage: f32,
}

impl SystemMonitorWidget {
    /// Create a new system monitor widget
    pub fn new(settings: HashMap<String, String>) -> Self {
        Self {
            settings,
            system_data: SystemData {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
            },
        }
    }
}

impl Widget for SystemMonitorWidget {
    fn get_type(&self) -> WidgetType {
        WidgetType::SystemMonitor
    }
    
    fn get_name(&self) -> String {
        "System Monitor".to_string()
    }
    
    fn get_description(&self) -> String {
        "Displays system resource usage".to_string()
    }
    
    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
    
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()> {
        self.settings = settings;
        Ok(())
    }
    
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()> {
        ui.horizontal(|ui| {
            ui.label(format!("CPU: {:.1}%", self.system_data.cpu_usage));
            ui.label(format!("RAM: {:.1}%", self.system_data.memory_usage));
            ui.label(format!("Disk: {:.1}%", self.system_data.disk_usage));
        });
        
        Ok(())
    }
    
    fn update(&mut self) -> AppResult<()> {
        // In a real implementation, this would fetch system data
        // For now, we'll just use dummy data
        self.system_data.cpu_usage = 25.5;
        self.system_data.memory_usage = 45.2;
        self.system_data.disk_usage = 60.8;
        
        Ok(())
    }
}

/// Calendar widget
pub struct CalendarWidget {
    /// Widget settings
    settings: HashMap<String, String>,
}

impl CalendarWidget {
    /// Create a new calendar widget
    pub fn new(settings: HashMap<String, String>) -> Self {
        Self { settings }
    }
}

impl Widget for CalendarWidget {
    fn get_type(&self) -> WidgetType {
        WidgetType::Calendar
    }
    
    fn get_name(&self) -> String {
        "Calendar".to_string()
    }
    
    fn get_description(&self) -> String {
        "Displays a calendar".to_string()
    }
    
    fn get_settings(&self) -> HashMap<String, String> {
        self.settings.clone()
    }
    
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()> {
        self.settings = settings;
        Ok(())
    }
    
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()> {
        let now = Local::now();
        let month = now.month();
        let year = now.year();
        
        ui.label(format!("{} {}", month, year));
        
        // In a real implementation, this would render a calendar
        // For now, we'll just display the current date
        ui.label(format!("Today: {}", now.format("%Y-%m-%d")));
        
        Ok(())
    }
    
    fn update(&mut self) -> AppResult<()> {
        // Nothing to update
        Ok(())
    }
}

/// Notes widget
pub struct NotesWidget {
    /// Widget settings
    settings: HashMap<String, String>,
    
    /// Notes content
    notes: String,
}

impl NotesWidget {
    /// Create a new notes widget
    pub fn new(settings: HashMap<String, String>) -> Self {
        let notes = settings.get("content").unwrap_or(&"".to_string()).clone();
        
        Self {
            settings,
            notes,
        }
    }
}

impl Widget for NotesWidget {
    fn get_type(&self) -> WidgetType {
        WidgetType::Notes
    }
    
    fn get_name(&self) -> String {
        "Notes".to_string()
    }
    
    fn get_description(&self) -> String {
        "Displays notes on your desktop".to_string()
    }
    
    fn get_settings(&self) -> HashMap<String, String> {
        let mut settings = self.settings.clone();
        settings.insert("content".to_string(), self.notes.clone());
        settings
    }
    
    fn update_settings(&mut self, settings: HashMap<String, String>) -> AppResult<()> {
        // Update notes content if provided
        if let Some(content) = settings.get("content") {
            self.notes = content.clone();
        }
        
        // Update other settings
        self.settings = settings;
        
        Ok(())
    }
    
    fn render(&self, ui: &mut egui::Ui) -> AppResult<()> {
        // Get settings
        let font_size = self.settings.get("font_size")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(14.0);
        
        let bg_color = self.settings.get("bg_color")
            .map(|c| {
                // Parse hex color (#RRGGBB)
                if c.starts_with('#') && c.len() == 7 {
                    let r = u8::from_str_radix(&c[1..3], 16).unwrap_or(255);
                    let g = u8::from_str_radix(&c[3..5], 16).unwrap_or(255);
                    let b = u8::from_str_radix(&c[5..7], 16).unwrap_or(255);
                    egui::Color32::from_rgb(r, g, b)
                } else {
                    egui::Color32::WHITE
                }
            })
            .unwrap_or(egui::Color32::WHITE);
        
        // Create a frame with the background color
        let frame = egui::Frame::none()
            .fill(bg_color)
            .rounding(5.0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 50)));
        
        frame.show(ui, |ui| {
            // Set the font size
            let style = ui.style_mut();
            style.text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = font_size;
            
            // Display the notes content
            ui.label("Notes:");
            
            // Create a text area for the notes
            let mut notes = self.notes.clone();
            if ui.text_edit_multiline(&mut notes).changed() {
                // In a real implementation, we would update the notes content
                // For now, we'll just log the change
                debug!("Notes content changed");
            }
        });
        
        Ok(())
    }
    
    fn update(&mut self) -> AppResult<()> {
        // Nothing to update
        Ok(())
    }
} 