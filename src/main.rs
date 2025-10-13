// slint 相关

// 在 Windows 发行版本中，除了 Slint 窗口外，还需阻止控制台窗口。其他平台忽略此设置。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod demo;
mod util;
mod ui;
mod protos;

// 将 .slint 编译后的 Rust 模块引入到代码里
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    println!("Hello main!");

    let window = DemoWindow::new()?;
    let window_weak = window.as_weak();

    window.on_increase_value(move || {
        let window = window_weak.unwrap();
        let i = window.get_counter();
        window.set_counter(i + 1)
    });

    // // 独立 Backend
    // let backend = window.global::<Backend>();
    // 
    // backend.on_btn_clicked(move || {
    //     println!("Hello btn_clicked!");
    // });
    // 
    // backend.on_btn_clicked2(|number| {
    //     println!("Hello btn_clicked2：number={number}");
    // });
    // 
    // backend.on_btn_clicked3(|i2| {
    //     println!("Hello btn_clicked3: {}", i2);
    //     i2 + 10
    // });

    {
        let valid=util::is_valid_ipv4("192.168.1.1".to_string());
        println!("is valid ipv4 {valid}");
        demo::impl_logic_for_backend(window.as_weak().clone());
    }

    ui::view::res(); // 引用方式一：在目录同层，创建同名 rs 文件，声明 mod
    protos::test::test_protos(); // 引用方式二：在目录内，创建 mod.rs 文件，声明 mod
    
    window.run()
}
