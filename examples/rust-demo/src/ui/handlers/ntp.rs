use crate::{AppState, AppWindow, i18n};
use std::sync::Arc;
use tokio::sync::Mutex;

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

            let _ = host;
            app.set_ntp_loading(false);
            app.set_ntp_result(i18n::t("ntp.unavailable").into());
        }
    });
}
