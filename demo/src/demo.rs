// use crate::{Backend, DemoWindow};
// use slint::{ComponentHandle, Weak};
// 
// pub fn impl_logic_for_backend(ui_weak: Weak<DemoWindow>) {
//     if let Some(strong_ui) = ui_weak.clone().upgrade() {
//         // 独立 Backend
//         let backend = strong_ui.global::<Backend>();
//         backend.on_btn_clicked(move || {
//             println!("Hello btn_clicked!");
//         });
// 
//         backend.on_btn_clicked2(|number| {
//             println!("Hello btn_clicked2：number={number}");
//         });
// 
//         backend.on_btn_clicked3(|i2| {
//             println!("Hello btn_clicked3: {}", i2);
//             i2 + 10
//         });
//     };
// }
