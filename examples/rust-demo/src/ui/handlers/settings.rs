use crate::{AppI18n, AppState, AppWindow, app::theme, config, i18n};
use george_base_log::{LogLevel, RuntimeLogConfig};
use slint::ComponentHandle;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_save_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let mut state = as_ptr.lock().await;
                let current_settings = state.config_manager.get_settings().clone();
                let user_settings = collect_user_settings(&app, &current_settings);

                apply_runtime_settings(&app, &mut state, &user_settings);

                match state.config_manager.update_settings(user_settings) {
                    Ok(_) => {
                        drop(state);
                        show_settings_message(&ah, i18n::t("settings.saved_success"));
                    }
                    Err(e) => {
                        let error_msg = i18n::tr("settings.saved_failed", &[e.to_string()]);
                        drop(state);
                        error!("{}", error_msg);
                        show_settings_message(&ah, error_msg);
                    }
                }
            }
        });
    });

    let ah = app_handle.clone();
    app.on_select_distro_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("settings.select_distro_dir"))
            .pick_folder()
            && let Some(app) = ah.upgrade()
        {
            app.set_distro_location(path.display().to_string().into());
        }
    });

    let ah = app_handle.clone();
    app.on_select_logs_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("settings.select_log_dir"))
            .pick_folder()
            && let Some(app) = ah.upgrade()
        {
            app.set_logs_location(path.display().to_string().into());
        }
    });

    let ah = app_handle.clone();
    app.on_theme_selected(move |idx| {
        if let Some(app) = ah.upgrade() {
            let theme_id = theme::theme_id_from_index(idx as usize);
            theme::apply_theme(&app, theme_id);
        }
    });
}

fn collect_user_settings(
    app: &AppWindow,
    current_settings: &config::UserSettings,
) -> config::UserSettings {
    config::UserSettings {
        modify_time: chrono::Utc::now().timestamp_millis().to_string(),
        check_time: current_settings.check_time.clone(),
        check_update: app.get_check_update_interval() as u8,
        distro_location: app.get_distro_location().to_string(),
        logs_location: app.get_logs_location().to_string(),
        temp_location: current_settings.temp_location.clone(),
        ui_language: app.get_ui_language().to_string(),
        theme_id: app.get_theme_id().to_string(),
        log_level: app.get_log_level() as u8,
        log_days: app.get_log_days() as u8,
    }
}

fn apply_runtime_settings(app: &AppWindow, state: &mut AppState, settings: &config::UserSettings) {
    let current_logs_location = state.config_manager.get_settings().logs_location.clone();
    if let Some(logging_system) = state.logging_system {
        if current_logs_location != settings.logs_location
            && let Err(err) =
                logging_system.update_file_directory(PathBuf::from(&settings.logs_location))
        {
            error!(
                "Failed to update log directory '{}': {}",
                settings.logs_location, err
            );
        }

        let current = logging_system.current_runtime_config();
        let updated = RuntimeLogConfig {
            enabled: current.enabled,
            level: log_level_from_u8(settings.log_level),
            console_enabled: current.console_enabled,
            file_enabled: current.file_enabled,
        };

        if let Err(err) = logging_system.apply_runtime_config(updated) {
            error!(
                "Failed to update log level '{}': {}",
                settings.log_level, err
            );
        }
    }

    let system_language = state
        .config_manager
        .get_config()
        .system
        .system_language
        .clone();
    let language_to_load = if settings.ui_language == "auto" {
        system_language
    } else {
        settings.ui_language.clone()
    };

    i18n::load_resources(&language_to_load);
    app.global::<AppI18n>()
        .set_version(app.global::<AppI18n>().get_version() + 1);
    theme::refresh_theme_options(app);
}

fn log_level_from_u8(level: u8) -> LogLevel {
    match level {
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        3 => LogLevel::Info,
        4 => LogLevel::Debug,
        5 => LogLevel::Trace,
        _ => LogLevel::Info,
    }
}

fn show_settings_message(app_handle: &slint::Weak<AppWindow>, message: String) {
    let app_handle = app_handle.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = app_handle.upgrade() {
            app.set_current_message(message.into());
            app.set_show_message_dialog(true);
        }
    });
}
