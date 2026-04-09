use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{i18n, AppWindow, Theme};

pub struct ThemeDefinition {
    pub id: &'static str,
    pub label_key: &'static str,
    pub appearance: ThemeAppearance,
}

pub enum ThemeAppearance {
    System,
    Light,
    Dark,
}

pub const THEMES: [ThemeDefinition; 3] = [
    ThemeDefinition {
        id: "system",
        label_key: "settings.system_default",
        appearance: ThemeAppearance::System,
    },
    ThemeDefinition {
        id: "light",
        label_key: "settings.light",
        appearance: ThemeAppearance::Light,
    },
    ThemeDefinition {
        id: "dark",
        label_key: "settings.dark",
        appearance: ThemeAppearance::Dark,
    },
];

pub fn default_theme_id() -> &'static str {
    THEMES[0].id
}

pub fn normalize_theme_id(theme_id: &str) -> &'static str {
    THEMES
        .iter()
        .find(|theme| theme.id == theme_id)
        .map(|theme| theme.id)
        .unwrap_or(default_theme_id())
}

pub fn selected_theme_index(theme_id: &str) -> usize {
    let normalized = normalize_theme_id(theme_id);
    THEMES
        .iter()
        .position(|theme| theme.id == normalized)
        .unwrap_or(0)
}

pub fn theme_id_from_index(index: usize) -> &'static str {
    THEMES
        .get(index)
        .map(|theme| theme.id)
        .unwrap_or(default_theme_id())
}

pub fn theme_labels() -> ModelRc<SharedString> {
    let labels: Vec<SharedString> = THEMES
        .iter()
        .map(|theme| i18n::t(theme.label_key).into())
        .collect();
    ModelRc::from(Rc::new(VecModel::from(labels)))
}

pub fn refresh_theme_options(app: &AppWindow) {
    app.set_theme_options(theme_labels());
}

pub fn apply_theme(app: &AppWindow, theme_id: &str) -> &'static str {
    let normalized = normalize_theme_id(theme_id);
    app.set_theme_id(normalized.into());
    app.set_selected_theme_index(selected_theme_index(normalized) as i32);
    app.global::<Theme>().set_dark_mode(resolve_dark_mode(normalized));
    normalized
}

pub fn resolve_dark_mode(theme_id: &str) -> bool {
    match THEMES[selected_theme_index(theme_id)].appearance {
        ThemeAppearance::Dark => true,
        ThemeAppearance::Light => false,
        ThemeAppearance::System => system_prefers_dark(),
    }
}

fn system_prefers_dark() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(value) = line.split_whitespace().last() {
                    let normalized = value.trim().to_ascii_lowercase();
                    if normalized == "0x0" || normalized == "0" {
                        return true;
                    }
                    if normalized == "0x1" || normalized == "1" {
                        return false;
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            return String::from_utf8_lossy(&output.stdout).trim() == "Dark";
        }
    }

    false
}
