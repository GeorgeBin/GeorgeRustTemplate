use crate::{AppState, AppWindow, i18n};
use chrono::{DateTime, Local};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tracing::{info, warn};

pub fn setup(
    app: &AppWindow,
    app_handle: slint::Weak<AppWindow>,
    _app_state: Arc<Mutex<AppState>>,
) {
    let ah = app_handle.clone();
    app.on_request_ntp_time(move || {
        if let Some(app) = ah.upgrade() {
            let host = app.get_ntp_host().trim().to_string();
            if host.is_empty() {
                app.set_ntp_result(i18n::t("ntp.empty_host").into());
                app.set_ntp_loading(false);
                return;
            }

            app.set_ntp_loading(true);
            app.set_ntp_result("".into());

            let app_handle = ah.clone();
            std::thread::spawn(move || {
                let server = if host.contains(':') {
                    host.clone()
                } else {
                    format!("{host}:123")
                };

                info!("Starting NTP query against {}", server);
                let result = corelib::ntp::NtpClient::new(&server).sync_time();
                let message = match result {
                    Ok(time) => i18n::tr("ntp.success", &[format_system_time(time)]),
                    Err(error) => {
                        warn!("NTP query failed for {}: {}", server, error);
                        i18n::tr("ntp.failure", &[error.to_string()])
                    }
                };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_handle.upgrade() {
                        app.set_ntp_loading(false);
                        app.set_ntp_result(message.into());
                    }
                });
            });
        }
    });
}

fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
