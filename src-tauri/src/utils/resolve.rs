use crate::{ core::Core, utils::init };
use tauri::{ App, AppHandle, Manager };

pub fn resolve_setup(app: &App) {
  init::init_app(app.package_info());

  let core = app.state::<Core>();
  core.set_win(app.get_window("main"));
  core.init(app.app_handle());

  resolve_window(app);
}

pub fn resolve_reset(app_handle: &AppHandle) {
  let core = app_handle.state::<Core>();
  let mut service = core.service.lock();

  crate::log_if_err!(service.stop());
}

fn resolve_window(app: &App) {
  let window = app.get_window("main").unwrap();

  #[cfg(target_os = "windows")]
  {
    use crate::utils::winhelp;
    use window_shadows::set_shadow;
    use window_vibrancy::apply_blur;

    let _ = window.set_decorations(false);
    let _ = set_shadow(&window, true);

    if !winhelp::is_win11() {
      let _ = apply_blur(&window, None);
    }
  }
}

pub fn create_window(app_handle: &AppHandle) {
  if let Some(window) = app_handle.get_window("main") {
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
    return;
  }

  let builder = tauri::window::WindowBuilder
    ::new(app_handle, "main".to_string(), tauri::WindowUrl::App("index.html".into()))
    .title("URLProtocolView")
    .center()
    .fullscreen(false)
    .min_inner_size(600.0, 520.0);

  #[cfg(target_os = "windows")]
  {
    use crate::utils::winhelp;
    use window_shadows::set_shadow;
    use window_vibrancy::apply_blur;

    match builder.decorations(false).transparent(true).inner_size(800.0, 636.0).build() {
      Ok(_) => {
        let app_handle = app_handle.clone();

        tauri::async_runtime::spawn(async move {
          if let Some(window) = app_handle.get_window("main") {
            let _ = set_shadow(&window, true);

            if !winhelp::is_win11() {
              let _ = apply_blur(&window, None);
            }
          }
        });
      }
      Err(err) => log::error!("{err}"),
    }
  }
}