use super::config::{CleanupConfig, FileLogConfig};
use super::error::StdLogInstallError;
use chrono::{Local, TimeDelta};

pub fn cleanup_old_logs(
    file: &FileLogConfig,
    cleanup: &CleanupConfig,
) -> Result<(), StdLogInstallError> {
    if !file.enabled || !cleanup.enabled {
        return Ok(());
    }

    if cleanup.max_retention_days < 1 {
        return Err(StdLogInstallError::InvalidConfig(
            "max_retention_days must be >= 1 when cleanup is enabled".to_string(),
        ));
    }

    let log_dir = &file.directory;
    if !log_dir.exists() || !log_dir.is_dir() {
        return Ok(());
    }

    let expiration_date =
        Local::now().date_naive() - TimeDelta::days(cleanup.max_retention_days as i64);
    let entries = match std::fs::read_dir(log_dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!(
                "warning: failed to read log directory '{}': {}",
                log_dir.display(),
                err
            );
            return Ok(());
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        let Some(date_part) = extract_date_part(file_name, &file.file_prefix) else {
            continue;
        };

        let Ok(file_date) = chrono::NaiveDate::parse_from_str(&date_part, "%Y-%m-%d") else {
            continue;
        };

        if file_date <= expiration_date
            && let Err(err) = std::fs::remove_file(&path)
        {
            eprintln!(
                "warning: failed to delete expired log file '{}': {}",
                path.display(),
                err
            );
        }
    }

    Ok(())
}

fn extract_date_part(file_name: &str, prefix: &str) -> Option<String> {
    if !file_name.starts_with(prefix) || !file_name.ends_with(".log") {
        return None;
    }

    let date_part = file_name
        .strip_prefix(prefix)?
        .strip_prefix('.')?
        .strip_suffix(".log")?;

    Some(date_part.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn file_config(dir: PathBuf) -> FileLogConfig {
        FileLogConfig {
            enabled: true,
            directory: dir,
            file_prefix: "app".to_string(),
        }
    }

    #[test]
    fn rejects_invalid_retention_days() {
        let cleanup = CleanupConfig {
            enabled: true,
            max_retention_days: 0,
        };

        let result = cleanup_old_logs(&file_config(std::env::temp_dir()), &cleanup);

        assert!(matches!(result, Err(StdLogInstallError::InvalidConfig(_))));
    }

    #[test]
    fn deletes_only_expired_matching_files() {
        let unique = format!(
            "platform-std-logging-cleanup-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&dir).expect("temp log dir should be created");

        let expired = dir.join("app.2000-01-01.log");
        let fresh = dir.join(format!(
            "app.{}.log",
            Local::now().date_naive().format("%Y-%m-%d")
        ));
        let foreign = dir.join("other.2000-01-01.log");

        std::fs::write(&expired, "expired").expect("expired file should be created");
        std::fs::write(&fresh, "fresh").expect("fresh file should be created");
        std::fs::write(&foreign, "foreign").expect("foreign file should be created");

        let cleanup = CleanupConfig {
            enabled: true,
            max_retention_days: 7,
        };

        cleanup_old_logs(&file_config(dir.clone()), &cleanup).expect("cleanup should succeed");

        assert!(!expired.exists());
        assert!(fresh.exists());
        assert!(foreign.exists());

        let _ = std::fs::remove_file(fresh);
        let _ = std::fs::remove_file(foreign);
        let _ = std::fs::remove_dir_all(dir);
    }
}
