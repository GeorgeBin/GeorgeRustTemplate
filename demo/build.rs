#[cfg(windows)]
extern crate winresource;

fn main() {
    slint_build::compile("ware/slint/ntp.slint").expect("Slint build failed");
    // slint_build::compile("ware/slint/demo.slint").expect("Slint build failed");
    // slint_build::compile("ware/slint/main.slint").expect("Slint build failed");

    // 给 exe 添加图标
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("ware/logo/windows.ico");
        res.compile().unwrap();
    }
}
