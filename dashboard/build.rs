#[path = "src/app/constants.rs"]
mod constants;

use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    slint_build::compile("src/ui/slint/app.slint").expect("Slint compilation failed");

    // Read Cargo.toml to get custom 'expire' field
    let cargo_toml_path = Path::new("Cargo.toml");
    let cargo_toml_content =
        fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    let cargo_toml: Value =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    let mut expire_time = cargo_toml
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("expire"))
        .and_then(|e| e.as_integer())
        .unwrap_or(0);

    // If expire is 0, set it to 1 year from now
    if expire_time == 0 && cargo_toml_content.contains("expire = 0") {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        // 365 days later
        expire_time = now + (365 * 24 * 60 * 60 * 1000);
    }

    println!("cargo:rustc-env=APP_EXPIRE_TIME={}", expire_time);

    #[cfg(windows)]
    {
        use image::ImageReader;

        let png_path = Path::new("assets/logo/logo.png");
        let ico_path = Path::new("assets/logo/logo.ico");

        if png_path.exists() {
            let img = ImageReader::open(png_path)
                .expect("Failed to open PNG file")
                .decode()
                .expect("Failed to decode PNG");

            let resized_img = img.resize_to_fill(256, 256, image::imageops::FilterType::Lanczos3);
            resized_img.save(ico_path).expect("Failed to save ICO file");

            let version =
                std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.1".to_string());
            let mut version_parts = version
                .split('.')
                .map(|p| p.parse::<u16>().unwrap_or(0))
                .collect::<Vec<_>>();
            while version_parts.len() < 3 {
                version_parts.push(0);
            }
            let (major, minor, patch) = (version_parts[0], version_parts[1], version_parts[2]);

            let icon_rc_path = Path::new("assets/logo/icon.rc");
            let file_description = format!("{} - Management Tool for WSL", constants::APP_NAME);
            let original_filename = format!("{}.exe", constants::APP_ID);

            std::fs::write(
                icon_rc_path,
                format!(
                    r#"#include <windows.h>

IDI_ICON1 ICON "logo.ico"

VS_VERSION_INFO VERSIONINFO
 FILEVERSION {major},{minor},{patch},0
 PRODUCTVERSION {major},{minor},{patch},0
 FILEFLAGSMASK 0x3fL
#ifdef _DEBUG
 FILEFLAGS 0x1L
#else
 FILEFLAGS 0x0L
#endif
 FILEOS 0x40004L
 FILETYPE 0x1L
 FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName", "{company_name}"
            VALUE "FileDescription", "{file_description}"
            VALUE "FileVersion", "{major}.{minor}.{patch}.0"
            VALUE "InternalName", "{app_id}"
            VALUE "LegalCopyright", "{copyright}"
            VALUE "LegalTrademarks", "{github_url}"
            VALUE "OriginalFilename", "{original_filename}"
            VALUE "ProductName", "{app_name}"
            VALUE "ProductVersion", "{major}.{minor}.{patch}.0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1200
    END
END
"#,
                    company_name = constants::COMPANY_NAME,
                    file_description = file_description,
                    app_id = constants::APP_ID,
                    copyright = constants::LEGAL_COPYRIGHT,
                    github_url = constants::GITHUB_URL,
                    original_filename = original_filename,
                    app_name = constants::APP_NAME,
                    major = major,
                    minor = minor,
                    patch = patch
                ),
            )
            .expect("Failed to write icon.rc");

            embed_resource::compile(icon_rc_path, std::iter::empty::<&std::ffi::OsStr>());
        }
    }
}
