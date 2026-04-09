use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{AppWindow, AppState};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, _app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    app.on_select_tab(move |tab| {
        if let Some(app) = ah.upgrade() {
            app.set_selected_tab(tab);
        }
    });

    app.on_open_url(move |url| {
        let _ = open::that(url.as_str());
    });
}
