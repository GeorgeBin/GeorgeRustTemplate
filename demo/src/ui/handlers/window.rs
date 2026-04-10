use crate::AppWindow;
use slint::ComponentHandle;
#[cfg(not(target_os = "macos"))]
use slint::winit_030::WinitWindowAccessor;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>) {
    let ah = app_handle.clone();
    app.on_drag_window(move || {
        if let Some(_app) = ah.upgrade() {
            #[cfg(not(target_os = "macos"))]
            {
                let _ = _app.window().with_winit_window(|window| {
                    let _ = window.drag_window();
                });
            }
        }
    });

    let ah = app_handle.clone();
    app.on_toggle_maximize_window(move || {
        if let Some(app) = ah.upgrade() {
            let next = !app.get_window_maximized();
            app.set_window_maximized(next);
            app.window().set_maximized(next);
        }
    });

    let ah = app_handle.clone();
    app.on_minimize_window(move || {
        if let Some(app) = ah.upgrade() {
            app.window().set_minimized(true);
        }
    });

    let ah = app_handle.clone();
    app.on_close_window(move || {
        if let Some(app) = ah.upgrade() {
            let _ = app.hide();
            slint::quit_event_loop().ok();
        }
    });

    #[cfg(not(target_os = "macos"))]
    {
        let ah = app_handle.clone();
        app.window().on_winit_window_event(move |_window, event| {
            use slint::winit_030::winit::event::WindowEvent;

            if let Some(app) = ah.upgrade() {
                match event {
                    WindowEvent::Resized(_) => {
                        let maximized = app.window().is_maximized();
                        if app.get_window_maximized() != maximized {
                            app.set_window_maximized(maximized);
                        }
                    }
                    WindowEvent::Occluded(_) => {
                        let maximized = app.window().is_maximized();
                        if app.get_window_maximized() != maximized {
                            app.set_window_maximized(maximized);
                        }
                    }
                    _ => {}
                }
            }

            slint::winit_030::EventResult::Propagate
        });
    }
}
