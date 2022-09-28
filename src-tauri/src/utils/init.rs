use crate::utils::{ dirs, tmpl };
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::PackageInfo;

fn init_config(app_dir: &PathBuf) -> std::io::Result<()> {
  let app_yml_path = app_dir.join("config.yaml");

  if !app_yml_path.exists() {
    fs::File::create(app_yml_path)?.write(tmpl::APP_CONFIG)?;
  }

  Ok(())
}

pub fn init_app(_package_info: &PackageInfo) {
  let app_dir = dirs::app_home_dir();

  if !app_dir.exists() {
    fs::create_dir_all(&app_dir).unwrap();
  }

  if let Err(err) = init_config(&app_dir) {
    log::error!("{err}");
  }
}