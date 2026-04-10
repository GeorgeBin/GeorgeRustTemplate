use std::sync::Arc;

use tokio::sync::Mutex;
#[cfg(any(feature = "update-check", feature = "expiry-check"))]
use tracing::info;
#[cfg(feature = "update-check")]
use tracing::warn;

use crate::{AppState, AppWindow};

#[cfg(feature = "update-check")]
use super::checker;
#[cfg(feature = "expiry-check")]
use super::expiry;
#[cfg(any(feature = "update-check", feature = "expiry-check"))]
use super::ui;

pub fn spawn_startup_tasks(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let current_v = env!("CARGO_PKG_VERSION").to_string();
        let (last_check_time, check_update_days, timezone) = {
            let state = app_state.lock().await;
            let settings = state.config_manager.get_settings();
            (
                settings.check_time.parse::<i64>().unwrap_or(0),
                settings.check_update as i64,
                state.config_manager.get_config().system.timezone.clone(),
            )
        };

        #[cfg(feature = "expiry-check")]
        {
            if expiry::configured_expire_time().is_some() {
                info!("Checking expiration on startup");
                if expiry::is_expired(&timezone).await {
                    ui::show_expired(&app_handle);
                    return;
                }
            }
        }

        #[cfg(feature = "update-check")]
        {
            let now_ms = chrono::Utc::now().timestamp_millis();
            let interval_ms = check_update_days * 24 * 60 * 60 * 1000;
            let should_check_update = (now_ms - last_check_time) >= interval_ms;

            info!(
                "Check-update: last={}, now={}, interval={}, should_check_update={}",
                last_check_time, now_ms, interval_ms, should_check_update
            );

            if !should_check_update {
                info!("Skipping startup update check (interval not reached)");
                return;
            }

            ui::set_checking_update(&app_handle, true);

            match checker::check_update(&current_v, &timezone).await {
                Ok(result) => {
                    ui::apply_update_result(
                        &app_handle,
                        result.has_update,
                        result.latest_version.clone(),
                    );
                    if result.has_update {
                        ui::show_update_available(&app_handle);
                    }
                }
                Err(e) => {
                    warn!("Auto check update failed: {}", e);
                    ui::set_checking_update(&app_handle, false);
                }
            }

            let mut state = app_state.lock().await;
            let _ = state.config_manager.update_check_time();
        }

        #[cfg(not(feature = "update-check"))]
        let _ = (current_v, last_check_time, check_update_days, timezone);

        #[cfg(not(any(feature = "update-check", feature = "expiry-check")))]
        let _ = (&app_handle, &app_state);
    });
}
