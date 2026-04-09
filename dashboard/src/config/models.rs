use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Configuration file version constant
pub const SETTINGS_VERSION: u32 = 3;

// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub homepage: String,
    #[serde(rename = "app-version", alias = "version")]
    pub app_version: String,
    #[serde(rename = "setting-version", default)]
    pub setting_version: u8,
    #[serde(rename = "startup-time")]
    pub startup_time: String,
}

// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    #[serde(rename = "system-language")]
    pub system_language: String,
    #[serde(rename = "timezone")]
    pub timezone: String,
}

// User settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserSettings {
    #[serde(rename = "modify-time", default)]
    pub modify_time: String,
    #[serde(rename = "check-time", default)]
    pub check_time: String,
    #[serde(rename = "check-update", default = "default_check_update")]
    pub check_update: u8,
    #[serde(rename = "distro-location")]
    pub distro_location: String,
    #[serde(rename = "logs-location")]
    pub logs_location: String,
    #[serde(rename = "temp-location", default)]
    pub temp_location: String,
    #[serde(rename = "ui-language")]
    pub ui_language: String,
    #[serde(rename = "theme-id", default = "default_theme_id")]
    pub theme_id: String,
    #[serde(rename = "log-level", default = "default_log_level")]
    pub log_level: u8,
    #[serde(rename = "log-days", default = "default_log_days")]
    pub log_days: u8,
}

pub fn default_log_level() -> u8 { 3 }
pub fn default_log_days() -> u8 { 7 }
pub fn default_check_update() -> u8 { 7 }
pub fn default_theme_id() -> String { crate::app::theme::default_theme_id().to_string() }

// Complete configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub application: ApplicationConfig,
    pub system: SystemConfig,
    pub settings: UserSettings,
}

impl Config {
    // Get default distribution installation path (prefer D drive)
    pub fn get_default_distro_location() -> String {
        #[cfg(target_os = "windows")]
        {
            if std::path::Path::new("D:\\").exists() {
                "D:\\linux".to_string()
            } else {
                "C:\\linux".to_string()
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            home_dir.join("dashboard").join("instances").to_string_lossy().to_string()
        }
    }

    // Create default configuration
    pub fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        #[cfg(target_os = "windows")]
        let app_data_dir = PathBuf::from(format!("{}\\.dashboard", home_dir.to_string_lossy()));

        #[cfg(not(target_os = "windows"))]
        let app_data_dir = dirs::data_dir()
            .unwrap_or_else(|| home_dir.clone())
            .join("dashboard");
        
        Self {
            application: ApplicationConfig {
                name: crate::app::APP_NAME.to_string(),
                homepage: crate::update::constants::PROJECT_HOMEPAGE.to_string(),
                app_version: env!("CARGO_PKG_VERSION").to_string(),
                setting_version: SETTINGS_VERSION as u8,
                startup_time: chrono::Utc::now().timestamp_millis().to_string(),
            },
            system: SystemConfig {
                system_language: String::new(),
                timezone: String::new(),
            },
            settings: UserSettings {
                modify_time: chrono::Utc::now().timestamp_millis().to_string(),
                check_time: "0".to_string(),
                check_update: 7,
                distro_location: Self::get_default_distro_location(),
                logs_location: app_data_dir.join("logs").to_string_lossy().to_string(),
                temp_location: app_data_dir.join("temp").to_string_lossy().to_string(),
                ui_language: "auto".to_string(),
                theme_id: default_theme_id(),
                log_level: 3,
                log_days: 7,
            },
        }
    }
}
