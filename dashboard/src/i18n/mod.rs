use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::Mutex;
use toml::Value;
use tracing::{debug, error};

#[derive(RustEmbed)]
#[folder = "assets/i18n/"]
struct Asset;

// Global storage for translations: "key" -> "translation"
// We flatten nested TOML: "section.key"
static TRANSLATIONS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn normalize_language_code(lang: &str) -> String {
    let lower = lang.to_lowercase();
    let lower = lower.replace("_", "-");

    if lower == "en" || lower.starts_with("en-") {
        return "en".to_string();
    }
    if lower == "zh" || lower == "zh-cn" || lower == "zh-sg" || lower.starts_with("zh-hans") {
        return "zh-CN".to_string();
    }

    "en".to_string()
}

pub fn load_resources(lang_code: &str) {
    let normalized = normalize_language_code(lang_code);
    let mut map = TRANSLATIONS.lock().unwrap();
    map.clear();

    // 1. Load English (Base)
    load_file_to_map("en", &mut map);

    // 2. Load Target (if not en)
    if normalized != "en" {
        load_file_to_map(&normalized, &mut map);
    }

    println!("i18n: Map populated with {} keys", map.len());
}

fn load_file_to_map(lang: &str, map: &mut HashMap<String, String>) {
    let filename = format!("{}.toml", lang);

    if let Some(content) = load_translation_content(&filename) {
        parse_translation_content(lang, &content, map);
    } else {
        error!("i18n: content not found for {}", lang);
    }
}

fn load_translation_content(filename: &str) -> Option<String> {
    #[cfg(debug_assertions)]
    {
        let path = std::path::Path::new("assets/i18n").join(filename);
        if let Ok(content) = std::fs::read_to_string(&path) {
            debug!("i18n: loaded {} from filesystem", path.display());
            return Some(strip_bom(content));
        }
        debug!(
            "i18n: failed to load {} from filesystem, falling back to embedded",
            path.display()
        );
    }

    Asset::get(filename).and_then(|file| {
        std::str::from_utf8(file.data.as_ref())
            .ok()
            .map(|content| strip_bom(content.to_string()))
    })
}

fn parse_translation_content(lang: &str, content: &str, map: &mut HashMap<String, String>) {
    match toml::from_str::<toml::Table>(content) {
        Ok(table) => flatten_toml("", &Value::Table(table), map),
        Err(e) => {
            error!("i18n: failed to parse TOML for {}: {}", lang, e);
            if let Ok(value) = content.parse::<Value>() {
                flatten_toml("", &value, map);
            }
        }
    }
}

fn strip_bom(mut content: String) -> String {
    if content.starts_with('\u{feff}') {
        content.remove(0);
    }
    content
}

#[cfg(debug_assertions)]
pub fn verify_translations() {
    if !cfg!(debug_assertions) {
        return;
    }

    println!("--- i18n Integrity Check ---");
    let mut en_map = HashMap::new();
    load_file_to_map("en", &mut en_map);
    println!("Base (en) keys: {}", en_map.len());

    let langs = ["zh-CN"];

    for lang in &langs {
        let mut lang_map = HashMap::new();
        load_file_to_map(lang, &mut lang_map);

        let mut missing = Vec::new();
        for key in en_map.keys() {
            if !lang_map.contains_key(key) {
                missing.push(key);
            }
        }

        if !missing.is_empty() {
            println!("[!] Language '{}' is missing {} keys:", lang, missing.len());
            for key in missing {
                println!("    - {}", key);
            }
        } else {
            println!(
                "[+] Language '{}' is fully translated ({} keys).",
                lang,
                lang_map.len()
            );
        }
    }
    println!("----------------------------");
}

fn flatten_toml(prefix: &str, value: &Value, map: &mut HashMap<String, String>) {
    match value {
        Value::Table(table) => {
            for (k, v) in table {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml(&new_key, v, map);
            }
        }
        Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        _ => {} // Ignore other types for now
    }
}

pub fn t(key: &str) -> String {
    let map = TRANSLATIONS.lock().unwrap();
    map.get(key).cloned().unwrap_or_else(|| key.to_string())
}

pub fn tr(key: &str, args: &[String]) -> String {
    let mut text = t(key);
    for (i, arg) in args.iter().enumerate() {
        text = text.replace(&format!("{{{}}}", i), arg);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chinese_translation_covers_english_keys() {
        let mut en_map = HashMap::new();
        let mut zh_map = HashMap::new();

        load_file_to_map("en", &mut en_map);
        load_file_to_map("zh-CN", &mut zh_map);

        let missing: Vec<_> = en_map
            .keys()
            .filter(|key| !zh_map.contains_key(*key))
            .cloned()
            .collect();

        assert!(
            missing.is_empty(),
            "zh-CN is missing translation keys: {:?}",
            missing
        );
    }
}
