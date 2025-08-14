// slint 相关

// 在 Windows 发行版本中，除了 Slint 窗口外，还需阻止控制台窗口。其他平台忽略此设置。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 将 .slint 编译后的 Rust 模块引入到代码里
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    println!("Hello, world!");

    let main_window = MainWindow::new()?;
    main_window.run()
}
