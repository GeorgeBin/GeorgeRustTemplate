pub fn configured_expire_time() -> Option<i64> {
    let expire_time = env!("APP_EXPIRE_TIME").parse::<i64>().unwrap_or(0);
    if expire_time > 0 {
        Some(expire_time)
    } else {
        None
    }
}

pub async fn is_expired(timezone: &str) -> bool {
    let Some(expire_time) = configured_expire_time() else {
        return false;
    };

    let timezone = timezone.to_string();
    let now = tokio::task::spawn_blocking(move || crate::utils::time::standard_time(&timezone))
        .await
        .unwrap_or(0);
    now > expire_time
}
