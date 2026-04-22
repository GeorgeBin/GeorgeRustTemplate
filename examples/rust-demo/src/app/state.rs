use crate::config::ConfigManager;
use george_platform_std::StdLoggingHandle;

// Define application state
pub struct AppState {
    pub config_manager: ConfigManager,
    pub logging_system: Option<&'static StdLoggingHandle>,
}

impl AppState {
    pub fn new(config_manager: ConfigManager, logging_system: &'static StdLoggingHandle) -> Self {
        Self {
            config_manager,
            logging_system: Some(logging_system),
        }
    }
}
