use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tracing::{error, info, warn};

mod migration;
pub mod models;

pub use models::*;

// Configuration manager, responsible for loading, saving, and managing application configuration
pub struct ConfigManager {
    // Configuration file path
    config_path: PathBuf,
    // Current configuration data
    config: Config,
}

impl ConfigManager {
    // Get configuration file path
    fn get_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".dashboard").join("settings.toml")
    }

    // Initialize configuration manager
    pub async fn new() -> Self {
        let config_path = Self::get_config_path();

        let config = if config_path.exists() {
            Self::load_existing_config(&config_path).await
        } else {
            Self::initialize_config(&config_path).await
        };

        Self {
            config_path,
            config,
        }
    }

    // Create default configuration and populate system information
    async fn create_default_config() -> Config {
        let mut config = Config::default();
        Self::refresh_system_info(&mut config, true).await;
        config
    }

    async fn load_existing_config(config_path: &PathBuf) -> Config {
        info!("Configuration file exists, loading...");
        match Self::load_config(config_path).await {
            Ok(mut config) => {
                let now = chrono::Utc::now().timestamp_millis();
                let last_modify = config.settings.modify_time.parse::<i64>().unwrap_or(0);
                let should_refresh_system = (now - last_modify) >= 604_800_000;

                Self::migrate_config(&mut config);

                let force_system = should_refresh_system
                    || config.system.system_language.is_empty()
                    || config.system.timezone.is_empty();

                Self::refresh_system_info(&mut config, force_system).await;
                config.settings = Self::normalize_settings(config.settings.clone());

                Self::ensure_config_directory(config_path);
                Self::ensure_settings_directories(&config.settings);

                if let Err(e) = Self::save_config(config_path, &mut config) {
                    error!("Failed to save config: {}", e);
                }

                config
            }
            Err(e) => {
                error!(
                    "Failed to load configuration file: {}, using default configuration",
                    e
                );
                Self::initialize_config(config_path).await
            }
        }
    }

    async fn initialize_config(config_path: &PathBuf) -> Config {
        info!("Configuration file does not exist, initializing...");
        let mut config = Self::create_default_config().await;
        config.settings = Self::normalize_settings(config.settings.clone());

        Self::ensure_config_directory(config_path);
        Self::ensure_settings_directories(&config.settings);

        if let Err(e) = Self::save_config(config_path, &mut config) {
            error!("Failed to save initial configuration: {}", e);
        } else {
            info!(
                "✅ Configuration file initialized successfully: {}",
                config_path.display()
            );
        }

        config
    }

    // Refresh system information fields
    async fn refresh_system_info(config: &mut Config, refresh_system: bool) {
        // Update startup time field
        config.application.startup_time = chrono::Utc::now().timestamp_millis().to_string();
        config.application.app_version = env!("CARGO_PKG_VERSION").to_string();

        if !refresh_system {
            info!("Skipping system environment query (less than 7 days since last update)");
            return;
        }

        info!("Refreshing system language and timezone information...");

        if let Some(system_language) = Self::detect_system_language() {
            info!("Detected system language: {}", system_language);
            config.system.system_language = system_language;
        } else if config.system.system_language.is_empty() {
            warn!("Failed to detect system language; falling back to empty value");
        } else {
            warn!(
                "Failed to detect system language; keeping existing value: {}",
                config.system.system_language
            );
        }

        let timezone = Self::detect_system_timezone();
        info!("Detected system timezone: {}", timezone);
        config.system.timezone = timezone;
    }

    fn detect_system_language() -> Option<String> {
        sys_locale::get_locale().and_then(|locale| {
            let locale = locale.trim().to_string();
            if locale.is_empty() {
                None
            } else {
                Some(locale)
            }
        })
    }

    fn detect_system_timezone() -> String {
        let seconds = chrono::Local::now().offset().local_minus_utc();
        let sign = if seconds >= 0 { '+' } else { '-' };
        let abs_seconds = seconds.abs();
        let hours = abs_seconds / 3600;
        let minutes = (abs_seconds % 3600) / 60;
        format!("UTC{}{:02}:{:02}", sign, hours, minutes)
    }

    // Load configuration file
    async fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    // Save configuration file
    fn save_config(path: &PathBuf, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
        // Update modify-time each time saving
        config.settings.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        let toml_string = toml::to_string_pretty(config)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    // Migrate configuration (version compatibility)
    fn migrate_config(config: &mut Config) {
        migration::migrate_config(config);
    }

    fn ensure_config_directory(path: &Path) {
        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            error!("Failed to create configuration directory: {}", e);
        }
    }

    fn ensure_settings_directories(settings: &UserSettings) {
        let _ = fs::create_dir_all(&settings.distro_location);
        let _ = fs::create_dir_all(&settings.logs_location);
        let _ = fs::create_dir_all(&settings.temp_location);
    }

    fn normalize_settings(mut settings: UserSettings) -> UserSettings {
        if settings.log_days != 7 && settings.log_days != 15 && settings.log_days != 30 {
            settings.log_days = 7;
        }

        if settings.check_update != 1
            && settings.check_update != 7
            && settings.check_update != 15
            && settings.check_update != 30
        {
            settings.check_update = 7;
        }

        settings.theme_id = crate::app::theme::normalize_theme_id(&settings.theme_id).to_string();
        if settings.ui_language != "auto" {
            settings.ui_language = crate::i18n::normalize_language_code(&settings.ui_language);
        }

        settings
    }

    pub fn normalized_settings(&self, settings: &UserSettings) -> UserSettings {
        Self::normalize_settings(settings.clone())
    }

    // Get configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    // Update user settings and save
    pub fn update_settings(
        &mut self,
        settings: UserSettings,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let settings = Self::normalize_settings(settings);
        Self::ensure_settings_directories(&settings);

        self.config.settings = settings;
        self.config.application.setting_version = SETTINGS_VERSION as u8;

        Self::save_config(&self.config_path, &mut self.config)?;
        info!("✅ Configuration saved successfully");
        Ok(())
    }

    // Get user settings
    pub fn get_settings(&self) -> &UserSettings {
        &self.config.settings
    }

    // Update popup detection timestamp
    #[cfg(feature = "update-check")]
    pub fn update_check_time(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.settings.check_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Updated check-time to: {}", self.config.settings.check_time);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_invalid_settings_to_supported_values() {
        let mut settings = Config::default().settings;
        settings.log_days = 99;
        settings.check_update = 99;
        settings.theme_id = "invalid".to_string();
        settings.ui_language = "en_US".to_string();

        let normalized = ConfigManager::normalize_settings(settings);

        assert_eq!(normalized.log_days, 7);
        assert_eq!(normalized.check_update, 7);
        assert_eq!(normalized.theme_id, crate::app::theme::default_theme_id());
        assert_eq!(normalized.ui_language, "en");
    }
}
