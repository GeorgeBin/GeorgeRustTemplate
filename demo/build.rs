use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    slint_build::compile("assets/slint/app.slint").expect("Slint compilation failed");

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
        let icon_rc_path = Path::new("assets/logo/icon.rc");

        if !ico_path.exists() {
            let img = ImageReader::open(png_path)
                .expect("Failed to open PNG file when logo.ico is missing")
                .decode()
                .expect("Failed to decode PNG");

            let resized_img = img.resize_to_fill(256, 256, image::imageops::FilterType::Lanczos3);
            resized_img
                .save(ico_path)
                .expect("Failed to save generated ICO file");
        }

        if !ico_path.exists() {
            panic!("No Windows icon found: expected assets/logo/logo.ico or assets/logo/logo.png");
        }

        embed_resource::compile(icon_rc_path, std::iter::empty::<&std::ffi::OsStr>());
    }
}
