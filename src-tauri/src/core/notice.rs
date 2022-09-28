use crate::log_if_err;
use tauri::Window;

#[derive(Debug, Default, Clone)]
pub struct Notice {
  win: Option<Window>,
}

impl Notice {
  pub fn from(win: Option<Window>) -> Notice {
    Notice { win }
  }

  pub fn refresh_config(&self) {
    if let Some(window) = self.win.as_ref() {
      log_if_err!(window.emit("protocolview://refresh-app-config", "yes"));
    }
  }
}