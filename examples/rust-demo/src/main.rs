#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use george_base_log::LogLevel;
use george_platform_std::{
    CleanupConfig, ConsoleLogConfig, FileLogConfig, StdLogConfig, install_global_tracing,
};
#[cfg(target_os = "macos")]
use slint::winit_030::winit::platform::macos::WindowAttributesExtMacOS;
use slint::{ComponentHandle, Model};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

// Define application modules
mod app;
mod config;
mod i18n;
mod ui;
mod update;
mod utils;

// Import Slint UI components
slint::include_modules!();

use app::{APP_ID, APP_NAME, AppState, COMPANY_NAME, LEGAL_COPYRIGHT};
use ui::handlers;

#[tokio::main]
async fn main() {
    configure_platform_backend();

    let app_state = initialize_app_state().await;
    let app = build_app_window();
    let app_handle = app.as_weak();

    bootstrap_ui(&app, &app_state).await;
    register_ui_handlers(&app, app_handle.clone(), app_state.clone());
    update::spawn_startup_tasks(app_handle.clone(), app_state.clone());

    app::window::show_and_center(&app);
    app.run().expect("Failed to run app");

    handle_app_exit().await;
}

fn configure_platform_backend() {
    #[cfg(target_os = "macos")]
    slint::BackendSelector::new()
        .with_winit_window_attributes_hook(|attributes| {
            attributes
                .with_title_hidden(true)
                .with_titlebar_transparent(true)
                .with_fullsize_content_view(true)
        })
        .select()
        .expect("Failed to configure macOS window backend");
}

async fn initialize_app_state() -> Arc<Mutex<AppState>> {
    let config_manager = config::ConfigManager::new().await;
    let settings = config_manager.get_settings().clone();
    let startup_language = startup_language(&config_manager, &settings);

    i18n::load_resources(&startup_language);
    #[cfg(debug_assertions)]
    i18n::verify_translations();

    let logging_config = build_logging_config(&settings);
    let logging_system =
        install_global_tracing(logging_config.clone()).expect("Failed to initialize logging");

    info!("Starting {} (ID: {})...", APP_NAME, APP_ID);

    Arc::new(Mutex::new(AppState::new(config_manager, logging_system)))
}

fn build_logging_config(settings: &config::UserSettings) -> StdLogConfig {
    StdLogConfig {
        enabled: true,
        level: log_level_from_u8(settings.log_level),
        console: ConsoleLogConfig { enabled: true },
        file: FileLogConfig {
            enabled: true,
            directory: PathBuf::from(&settings.logs_location),
            file_prefix: "demo".to_string(),
        },
        cleanup: CleanupConfig {
            enabled: true,
            max_retention_days: settings.log_days as u16,
        },
    }
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

fn startup_language(
    config_manager: &config::ConfigManager,
    settings: &config::UserSettings,
) -> String {
    if settings.ui_language == "auto" {
        config_manager.get_config().system.system_language.clone()
    } else {
        settings.ui_language.clone()
    }
}

fn build_app_window() -> AppWindow {
    let app = AppWindow::new().expect("Failed to create app");
    app.set_use_native_window_chrome(cfg!(target_os = "linux"));
    app.set_is_macos(cfg!(target_os = "macos"));
    app.set_show_window_controls(cfg!(target_os = "windows"));
    app.global::<Theme>()
        .set_default_font_family("Source Han Sans SC".into());
    app::theme::refresh_theme_options(&app);
    register_i18n_callback(&app);

    debug!(
        "App Metadata - Company: {}, Copyright: {}",
        COMPANY_NAME, LEGAL_COPYRIGHT
    );

    app
}

fn register_i18n_callback(app: &AppWindow) {
    app.global::<AppI18n>().on_t(|key, args| {
        let args_vec: Vec<String> = args
            .iter()
            .map(|value: slint::SharedString| value.to_string())
            .collect();
        i18n::tr(&key, &args_vec).into()
    });
    app.global::<AppI18n>().set_version(1);
}

async fn bootstrap_ui(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    load_settings_to_ui(app, app_state).await;
    update::configure_app_info(app, app_state).await;
}

fn register_ui_handlers(
    app: &AppWindow,
    app_handle: slint::Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
) {
    handlers::common::setup(app, app_handle.clone(), app_state.clone());
    handlers::rust_examples::setup(app, app_handle.clone(), app_state.clone());
    handlers::window::setup(app, app_handle.clone());
    handlers::settings::setup(app, app_handle.clone(), app_state.clone());
    update::register_ui_handlers(app, app_handle, app_state);
}

async fn load_settings_to_ui(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    let (raw_settings, settings, system_language) = {
        let state = app_state.lock().await;
        (
            state.config_manager.get_settings().clone(),
            state
                .config_manager
                .normalized_settings(state.config_manager.get_settings()),
            state
                .config_manager
                .get_config()
                .system
                .system_language
                .clone(),
        )
    };

    app.set_ui_language(settings.ui_language.clone().into());
    app.set_window_maximized(app.window().is_maximized());
    let theme_id = app::theme::apply_theme(app, &settings.theme_id);
    app.set_distro_location(settings.distro_location.clone().into());
    app.set_logs_location(settings.logs_location.clone().into());
    app.set_log_level(settings.log_level as i32);
    app.set_log_days(settings.log_days as i32);
    app.set_check_update_interval(settings.check_update as i32);

    if settings != raw_settings {
        let mut state_mut = app_state.lock().await;
        let _ = state_mut.config_manager.update_settings(settings.clone());
    }

    info!(
        "Configuration loaded to UI (Language: {}, SystemLanguage: {}, ThemeId: {}, LogLevel: {}, LogDays: {})",
        settings.ui_language, system_language, theme_id, settings.log_level, settings.log_days
    );
}

// Processing after application exit
async fn handle_app_exit() {
    debug!("Application exited");
}
