use super::{Config, SETTINGS_VERSION};
use tracing::info;

// Record new or changed fields for each version to implement global configuration control
//
// v1 global field list (2026-01-10):
//   [application]
//   - name: String
//   - app-version: String (previously version)
//   - setting-version: String (previously settings.version)
//   - startup-time: String
//   [system]
//   - system-language: String
//   - timezone: String
//   [settings]
//   - modify-time: String
//   - distro-location: String
//   - logs-location: String
//   - temp-location: String
//   - ui-language: String
//
// v2 new fields (2026-01-10 16:16):
//   [settings]
//   - check-time: String (millisecond timestamp, default "0")
//   - check-update: u8 (days, default 7)

pub fn migrate_config(config: &mut Config) {
    let old_version = config.application.setting_version as u32;

    if old_version >= SETTINGS_VERSION {
        return;
    }

    info!(
        "Detected configuration version v{}, upgrading to v{}...",
        old_version, SETTINGS_VERSION
    );

    // v0 -> v1 logic
    if old_version < 1 {
        info!("Upgrading to v1: migrating version position, ensuring base fields exist");
    }

    // v1 -> v2 logic
    if old_version < 2 {
        info!("Upgrading to v2: adding [settings] check-time,check-update");
        config.settings.check_time = "0".to_string();
        config.settings.check_update = 7;
    }

    // v2 -> v3 logic
    if old_version < 3 {
        info!("Upgrading to v3: ensuring temp/theme/log settings exist");
        if config.settings.temp_location.is_empty() {
            config.settings.temp_location = crate::config::Config::default().settings.temp_location;
        }
        if config.settings.theme_id.is_empty() {
            config.settings.theme_id = crate::config::models::default_theme_id();
        }
        if config.settings.log_level == 0 {
            config.settings.log_level = crate::config::models::default_log_level();
        }
        if config.settings.log_days == 0 {
            config.settings.log_days = crate::config::models::default_log_days();
        }
    }

    config.application.setting_version = SETTINGS_VERSION as u8;
    info!(
        "✅ Configuration migration complete, current version: v{}",
        SETTINGS_VERSION
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_legacy_settings_to_v3() {
        let mut config = crate::config::Config::default();
        config.application.setting_version = 2;
        config.settings.temp_location.clear();
        config.settings.theme_id.clear();
        config.settings.log_level = 0;
        config.settings.log_days = 0;

        migrate_config(&mut config);

        assert_eq!(config.application.setting_version, SETTINGS_VERSION as u8);
        assert!(!config.settings.temp_location.is_empty());
        assert!(!config.settings.theme_id.is_empty());
        assert_eq!(
            config.settings.log_level,
            crate::config::models::default_log_level()
        );
        assert_eq!(
            config.settings.log_days,
            crate::config::models::default_log_days()
        );
    }
}
