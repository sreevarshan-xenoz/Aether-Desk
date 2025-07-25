use aether_desk::core::{Config, WallpaperType, AppResult};
use std::path::PathBuf;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert_eq!(config.wallpaper.wallpaper_type, WallpaperType::Static);
        assert!(!config.wallpaper.auto_change.enabled);
        assert_eq!(config.wallpaper.auto_change.interval, 30);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).expect("Failed to serialize config");
        let deserialized: Config = serde_json::from_str(&json).expect("Failed to deserialize config");
        
        assert_eq!(config.wallpaper.wallpaper_type, deserialized.wallpaper.wallpaper_type);
        assert_eq!(config.app.start_with_system, deserialized.app.start_with_system);
    }

    #[test]
    fn test_wallpaper_types() {
        let types = vec![
            WallpaperType::Static,
            WallpaperType::Video,
            WallpaperType::Web,
            WallpaperType::Shader,
            WallpaperType::Audio,
        ];

        for wallpaper_type in types {
            let json = serde_json::to_string(&wallpaper_type).expect("Failed to serialize wallpaper type");
            let deserialized: WallpaperType = serde_json::from_str(&json).expect("Failed to deserialize wallpaper type");
            assert_eq!(wallpaper_type, deserialized);
        }
    }

    #[tokio::test]
    async fn test_config_file_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let config_path = temp_dir.path().join("test_config.json");
        
        let mut config = Config::default();
        config.app.start_with_system = true;
        config.wallpaper.wallpaper_type = WallpaperType::Video;
        
        // This would need to be implemented in the Config struct
        // config.save_to_path(&config_path).expect("Failed to save config");
        
        // let loaded_config = Config::load_from_path(&config_path).expect("Failed to load config");
        // assert_eq!(config.app.start_with_system, loaded_config.app.start_with_system);
        // assert_eq!(config.wallpaper.wallpaper_type, loaded_config.wallpaper.wallpaper_type);
    }
}
