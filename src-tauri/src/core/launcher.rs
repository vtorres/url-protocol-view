use anyhow::{ Result };
use auto_launch::{ AutoLaunch, AutoLaunchBuilder };
use tauri::{ utils::platform::current_exe };

pub struct Launcher {
  auto_launch: Option<AutoLaunch>,
}

impl Launcher {
  pub fn new() -> Launcher {
    Launcher {
      auto_launch: None,
    }
  }

  pub fn init_launch(&mut self, _: Option<bool>) -> Result<()> {
    let app_exe = current_exe().unwrap();
    let app_exe = dunce::canonicalize(app_exe).unwrap();
    let app_name = app_exe.file_stem().unwrap().to_str().unwrap();
    let app_path = app_exe.as_os_str().to_str().unwrap();

    #[cfg(target_os = "windows")]
    let app_path = format!("\"{app_path}\"");

    let auto = AutoLaunchBuilder::new().set_app_name(app_name).set_app_path(&app_path).build()?;

    self.auto_launch = Some(auto);

    Ok(())
  }

  pub fn update_launch(&mut self, enable: Option<bool>) -> Result<()> {
    if enable.is_none() {
      return Ok(());
    }

    let auto_launch = self.auto_launch.as_ref().unwrap();

    match enable.unwrap() {
      true => auto_launch.enable().is_ok(),
      false => auto_launch.disable().is_ok(),
    };

    Ok(())
  }
}