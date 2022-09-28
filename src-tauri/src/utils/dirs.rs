use std::path::PathBuf;
use tauri::api::path::{ home_dir };

static APP_DIR: &str = "url-protocol-view";
static APP_CONFIG: &str = "config.yaml";

static mut RESOURCE_DIR: Option<PathBuf> = None;

static mut PORTABLE_FLAG: bool = false;

pub unsafe fn init_portable_flag() {
  #[cfg(target_os = "windows")]
  {
    use tauri::utils::platform::current_exe;

    let exe = current_exe().unwrap();
    let dir = exe.parent().unwrap();
    let dir = PathBuf::from(dir).join(".config/PORTABLE");

    if dir.exists() {
      PORTABLE_FLAG = true;
    }
  }
}

pub fn app_home_dir() -> PathBuf {
  #[cfg(target_os = "windows")]
  unsafe {
    use tauri::utils::platform::current_exe;

    if !PORTABLE_FLAG {
      home_dir().unwrap().join(".config").join(APP_DIR)
    } else {
      let app_exe = current_exe().unwrap();
      let app_exe = dunce::canonicalize(app_exe).unwrap();
      let app_dir = app_exe.parent().unwrap();
      PathBuf::from(app_dir).join(".config").join(APP_DIR)
    }
  }

  #[cfg(not(target_os = "windows"))]
  home_dir().unwrap().join(".config").join(APP_DIR)
}

pub fn app_path() -> PathBuf {
  app_home_dir().join(APP_CONFIG)
}

#[cfg(windows)]
static SERVICE_PATH: &str = "url-protocol-view-service.exe";

#[cfg(windows)]
pub fn service_path() -> PathBuf {
  unsafe {
    let res_dir = RESOURCE_DIR.clone().unwrap();

    res_dir.join(SERVICE_PATH)
  }
}