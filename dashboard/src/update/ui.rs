use std::sync::Arc;

use slint::ComponentHandle;
use tokio::sync::Mutex;
#[cfg(feature = "update-check")]
use tracing::{info, warn};

#[cfg(feature = "update-check")]
use crate::i18n;
use crate::{update::constants, AppInfo, AppState, AppWindow};

#[cfg(feature = "update-check")]
use super::checker;

pub async fn configure_app_info(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    app.global::<AppInfo>()
        .set_version(env!("CARGO_PKG_VERSION").into());
    app.global::<AppInfo>()
        .set_homepage(constants::PROJECT_HOMEPAGE.into());
    app.global::<AppInfo>().set_issues_url(
        format!("{}{}", constants::PROJECT_HOMEPAGE, constants::PROJECT_ISSUES_PATH).into(),
    );

    let timezone = {
        let state = app_state.lock().await;
        state.config_manager.get_config().system.timezone.clone()
    };
    app.global::<AppInfo>()
        .set_release_url(constants::release_url_for_timezone(&timezone).into());
}

pub fn register_ui_handlers(
    app: &AppWindow,
    app_handle: slint::Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
) {
    let ah = app_handle.clone();
    let as_check = app_state.clone();
    app.on_check_update(move || {
        #[cfg(feature = "update-check")]
        {
            info!("Manual check update triggered from UI");
            let ah = ah.clone();
            let current_v = env!("CARGO_PKG_VERSION").to_string();
            let as_ptr = as_check.clone();

            tokio::spawn(async move {
                info!("Starting manual check for version: {}", current_v);
                set_checking_update(&ah, true);

                let timezone = {
                    let state = as_ptr.lock().await;
                    state.config_manager.get_config().system.timezone.clone()
                };

                match checker::check_update(&current_v, &timezone).await {
                    Ok(result) => {
                        info!("Update check result: has_update={}", result.has_update);
                        apply_update_result(&ah, result.has_update, result.latest_version.clone());
                        if result.has_update {
                            show_update_available(&ah);
                        } else {
                            show_update_latest_message(&ah);
                        }
                    }
                    Err(e) => {
                        warn!("Manual check update failed: {}", e);
                        show_update_error(&ah, e);
                    }
                }
            });
        }

        #[cfg(not(feature = "update-check"))]
        let _ = (&ah, &as_check);
    });

    let as_download = app_state.clone();
    app.on_download_update(move || {
        #[cfg(any(feature = "update-check", feature = "expiry-check"))]
        {
            let as_ptr = as_download.clone();
            slint::spawn_local(async move {
                let timezone = {
                    let state = as_ptr.lock().await;
                    state.config_manager.get_config().system.timezone.clone()
                };
                let _ = open::that(constants::release_url_for_timezone(&timezone));
            })
            .unwrap();
        }

        #[cfg(not(any(feature = "update-check", feature = "expiry-check")))]
        let _ = &as_download;
    });

    let ah = app_handle.clone();
    app.on_close_expire_dialog(move || {
        if let Some(app) = ah.upgrade() {
            app.set_show_expire_dialog(false);
        }
    });
}

fn run_on_app<F>(app_handle: &slint::Weak<AppWindow>, update: F)
where
    F: FnOnce(AppWindow) + Send + 'static,
{
    let ah = app_handle.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah.upgrade() {
            update(app);
        }
    });
}

#[cfg(feature = "update-check")]
pub(crate) fn set_checking_update(app_handle: &slint::Weak<AppWindow>, checking: bool) {
    run_on_app(app_handle, move |app| {
        app.global::<AppInfo>().set_checking_update(checking);
    });
}

#[cfg(feature = "update-check")]
pub(crate) fn apply_update_result(
    app_handle: &slint::Weak<AppWindow>,
    has_update: bool,
    latest_version: String,
) {
    run_on_app(app_handle, move |app| {
        app.global::<AppInfo>().set_checking_update(false);
        app.global::<AppInfo>().set_has_update(has_update);
        app.global::<AppInfo>()
            .set_latest_version(latest_version.into());
    });
}

#[cfg(feature = "update-check")]
pub(crate) fn show_update_available(app_handle: &slint::Weak<AppWindow>) {
    run_on_app(app_handle, move |app| {
        app.set_show_update_dialog(true);
    });
}

#[cfg(feature = "update-check")]
pub(crate) fn show_update_latest_message(app_handle: &slint::Weak<AppWindow>) {
    run_on_app(app_handle, move |app| {
        app.set_current_message(i18n::t("dialog.update_latest").into());
        app.set_show_message_dialog(true);
    });
}

#[cfg(feature = "update-check")]
pub(crate) fn show_update_error(app_handle: &slint::Weak<AppWindow>, err: String) {
    run_on_app(app_handle, move |app| {
        app.global::<AppInfo>().set_checking_update(false);

        let message = if err == "RequestTimeOut" {
            i18n::t("dialog.update_timeout")
        } else {
            i18n::tr("dialog.update_failed", &[err])
        };

        app.set_current_message(message.into());
        app.set_show_message_dialog(true);
    });
}

#[cfg(feature = "expiry-check")]
pub(crate) fn show_expired(app_handle: &slint::Weak<AppWindow>) {
    run_on_app(app_handle, move |app| {
        app.set_show_expire_dialog(true);
    });
}
