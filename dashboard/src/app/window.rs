#[cfg(target_os = "windows")]
use tracing::debug;
use crate::AppWindow;

// Windows platform window centering function
#[cfg(target_os = "windows")]
pub fn center_window(_app: &AppWindow) {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, SetWindowPos, FindWindowW, SWP_NOSIZE, SWP_NOZORDER, HWND_TOP};
    use windows::Win32::Graphics::Gdi::{MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTOPRIMARY};
    use windows::Win32::Foundation::RECT;
    use windows::core::HSTRING;
    
    let title = crate::app::APP_NAME;
    let title_h = HSTRING::from(title);
    
    unsafe {
        // Find window handle by window title
        if let Ok(hwnd) = FindWindowW(None, &title_h) {
            if !hwnd.0.is_null() {
                // Get the monitor where the window is located
                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                
                // Get monitor information
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                
                if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() {
                    // Get window rectangle
                    let mut window_rect = RECT::default();
                    if GetWindowRect(hwnd, &mut window_rect).is_ok() {
                        let monitor_rect = monitor_info.rcWork; // Use work area (excluding taskbar)
                        
                        let window_width = window_rect.right - window_rect.left;
                        let window_height = window_rect.bottom - window_rect.top;
                        let monitor_width = monitor_rect.right - monitor_rect.left;
                        let monitor_height = monitor_rect.bottom - monitor_rect.top;
                        
                        // Calculate centered position
                        let x = monitor_rect.left + (monitor_width - window_width) / 2;
                        let y = monitor_rect.top + (monitor_height - window_height) / 2;
                        
                        // Set window position
                        let _ = SetWindowPos(hwnd, HWND_TOP, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
                        
                        debug!("Window centered at ({}, {}) on monitor with work area: {}x{}", 
                              x, y, monitor_width, monitor_height);
                    }
                }
            }
        }
    }
}

// Show window and center it
pub fn show_and_center(app: &AppWindow) {
    #[cfg(target_os = "windows")]
    {
        use slint::ComponentHandle;
        let app_weak = app.as_weak();
        app.show().unwrap();
        
        // Execute centering logic with 100ms delay in event loop to ensure window is fully visible
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    center_window(&app);
                }
            });
        });
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use slint::ComponentHandle;
        app.show().unwrap();
    }
}
