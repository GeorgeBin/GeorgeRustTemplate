use crate::NTPWindow;
use corelib::ntp::NtpError;
use slint::{ComponentHandle, Weak};
use std::time::SystemTime;

pub fn get_time(ui_weak: Weak<NTPWindow>) {
    // 注册点击事件
    if let Some(ui) = ui_weak.upgrade() {
        ui.on_click(move || {
            // 克隆弱引用以供线程中使用
            let ui_weak_clone = ui_weak.clone();

            // 获取输入框内容
            if let Some(window) = ui_weak_clone.upgrade() {
                let input = window.get_input_text();
                println!("点击按钮，输入内容：{input}");

                // 执行 NTP 请求
                get_ntp_time(&input, move |result| {
                    // 回调完成后更新 UI
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak_clone.upgrade() {
                            match result {
                                Ok(time) => {
                                    let text = format!("同步成功：{}", format_system_time(time));
                                    ui.set_show_text(text.into());
                                }
                                Err(e) => {
                                    ui.set_show_text(format!("同步失败：{e:?}").into());
                                }
                            }
                        }
                    })
                    .unwrap();
                });
            }
        });
    }
}

/// 工具函数：格式化 SystemTime
pub fn format_system_time(t: SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = t.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 执行 NTP 同步任务
fn get_ntp_time<F>(ip: &str, f: F)
where
    F: FnOnce(Result<SystemTime, NtpError>) + Send + 'static,
{
    let ip = ip.to_string();
    std::thread::spawn(move || {
        let address = format!("{ip}:123");
        let client = corelib::ntp::NtpClient::new(&address);
        let result = client.sync_time();
        f(result);
    });
}
