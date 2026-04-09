pub const PROJECT_HOMEPAGE: &str = "https://github.com/xxx/dashboard";
pub const PROJECT_CN_RELEASE_BASE: &str = "https://gitee.com/bye/dashboard";
pub const PROJECT_ISSUES_PATH: &str = "/issues";
pub const PROJECT_RELEASES_PATH: &str = "/releases";
#[cfg(feature = "update-check")]
pub const STATIC_API_FREE: &str = "https://raw.githubusercontent.com/xxx/oss/refs/heads";
#[cfg(feature = "update-check")]
pub const STATIC_API_CN: &str = "https://gitee.com/bye/oss/raw";
#[cfg(feature = "update-check")]
pub const UPDATE_CHECK_API: &str = "/main/dashboard/api/base.json";
pub const ZH_TIMEZONE: &str = "UTC+08:00";

pub fn release_base_url_for_timezone(timezone: &str) -> &'static str {
    if timezone == ZH_TIMEZONE {
        PROJECT_CN_RELEASE_BASE
    } else {
        PROJECT_HOMEPAGE
    }
}

pub fn release_url_for_timezone(timezone: &str) -> String {
    format!("{}{}", release_base_url_for_timezone(timezone), PROJECT_RELEASES_PATH)
}

#[cfg(feature = "update-check")]
pub fn update_manifest_url_for_timezone(timezone: &str, ts: u64) -> String {
    let base_url = if timezone == ZH_TIMEZONE {
        STATIC_API_CN
    } else {
        STATIC_API_FREE
    };
    format!("{}{}?t={}", base_url, UPDATE_CHECK_API, ts)
}
