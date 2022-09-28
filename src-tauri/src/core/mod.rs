use self::notice::Notice;
use self::launcher::Launcher;
use crate::log_if_err;
use anyhow::{ Result };
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{ AppHandle, Manager, Window };

mod notice;
mod service;
mod launcher;
mod app;

pub use self::service::*;
pub use self::app::*;

#[derive(Clone)]
pub struct Core {
  pub app: Arc<Mutex<URLProtocolView>>,
  pub service: Arc<Mutex<Service>>,
  pub launcher: Arc<Mutex<Launcher>>,
  pub window: Arc<Mutex<Option<Window>>>,
}

impl Core {
  pub fn new() -> Core {
    let app = URLProtocolView::new();
    let service = Service::new();

    Core {
      app: Arc::new(Mutex::new(app)),
      service: Arc::new(Mutex::new(service)),
      launcher: Arc::new(Mutex::new(Launcher::new())),
      window: Arc::new(Mutex::new(None)),
    }
  }

  pub fn init(&self, app_handle: tauri::AppHandle) {
    let app = self.app.lock();
    let mut service = self.service.lock();

    log_if_err!(service.start());

    drop(app);
    drop(service);

    let app = self.app.lock();

    let silent_start = app.enable_silent_start.clone();
    let auto_launch = app.enable_auto_launch.clone();

    if silent_start.unwrap_or(false) {
      let window = self.window.lock();
      window.as_ref().map(|win| { win.hide().unwrap() });
    }

    let mut launcher = self.launcher.lock();

    drop(app);

    log_if_err!(launcher.init_launch(auto_launch));
    log_if_err!(self.update_systray(&app_handle));
  }

  pub fn set_win(&self, win: Option<Window>) {
    let mut window = self.window.lock();
    *window = win;
  }

  pub fn patch_app_config(&self, patch: URLProtocolView, _app_handle: &AppHandle) -> Result<()> {
    let auto_launch = patch.enable_auto_launch.clone();

    if auto_launch.is_some() {
      let mut launcher = self.launcher.lock();
      launcher.update_launch(auto_launch)?;
    }

    let mut app = self.app.lock();
    app.patch_app_config(patch)?;
    drop(app);

    Ok(())
  }

  pub fn update_systray(&self, app_handle: &AppHandle) -> Result<()> {
    let app = self.app.lock();
    let _tray = app_handle.tray_handle();
    let window = app_handle.get_window("main");
    let notice = Notice::from(window);

    notice.refresh_config();
    drop(app);

    Ok(())
  }
}