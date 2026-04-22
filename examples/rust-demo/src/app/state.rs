use crate::config::ConfigManager;
use george_base_log::LoggingHandle;

// Define application state
pub struct AppState {
    pub config_manager: ConfigManager,
    pub logging_system: Option<&'static LoggingHandle>,
}

impl AppState {
    pub fn new(config_manager: ConfigManager, logging_system: &'static LoggingHandle) -> Self {
        Self {
            config_manager,
            logging_system: Some(logging_system),
        }
    }
}
