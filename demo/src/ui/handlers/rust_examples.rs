use crate::{AppState, AppWindow, i18n};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn setup(
    app: &AppWindow,
    app_handle: slint::Weak<AppWindow>,
    _app_state: Arc<Mutex<AppState>>,
) {
    let ah = app_handle.clone();
    app.on_example_increment(move || {
        if let Some(app) = ah.upgrade() {
            let next = app.get_example_counter() + 1;
            app.set_example_counter(next);
            app.set_example_status(
                i18n::tr("rust_examples.interaction.no_arg_result", &[next.to_string()]).into(),
            );
        }
    });

    let ah = app_handle.clone();
    app.on_example_report_counter(move |value| {
        if let Some(app) = ah.upgrade() {
            app.set_example_status(
                i18n::tr("rust_examples.interaction.arg_result", &[value.to_string()]).into(),
            );
        }
    });

    let ah = app_handle.clone();
    app.on_example_add_ten(move |value| {
        let next = value + 10;
        if let Some(app) = ah.upgrade() {
            app.set_example_status(
                i18n::tr(
                    "rust_examples.interaction.return_result",
                    &[value.to_string(), next.to_string()],
                )
                .into(),
            );
        }
        next
    });
}
