use crate::utils::{ dirs };
use anyhow::{ bail, Result };
use serde::{ Deserialize, Serialize };

#[derive(Debug)]
pub struct Service {
  service_mode: bool,
}

impl Service {
  pub fn new() -> Service {
    Service {
      service_mode: false,
    }
  }

  #[allow(unused)]
  pub fn set_mode(&mut self, enable: bool) {
    self.service_mode = enable;
  }

  pub fn start(&mut self) -> Result<()> {
    Ok(())
  }

  pub fn stop(&mut self) -> Result<()> {
    Ok(())
  }

  #[allow(unused)]
  pub fn restart(&mut self) -> Result<()> {
    self.stop()?;
    self.start()
  }
}

#[cfg(windows)]
pub mod win_service {
  use super::*;
  use anyhow::Context;
  use deelevate::{ PrivilegeLevel, Token };
  use runas::Command as RunasCommand;
  use std::os::windows::process::CommandExt;
  use std::{ process::Command as StdCommand };

  const SERVICE_NAME: &str = "protocolview";

  const SERVICE_URL: &str = "http://0.0.0.0:33211";

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct ResponseBody {
    pub bin_path: String,
    pub config_dir: String,
    pub log_file: String,
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct JsonResponse {
    pub code: u64,
    pub msg: String,
    pub data: Option<ResponseBody>,
  }

  impl Service {
    pub async fn install_service() -> Result<()> {
      let binary_path = dirs::service_path();
      let install_path = binary_path.with_file_name("install-service.exe");

      if !install_path.exists() {
        bail!("installer exe not found");
      }

      let token = Token::with_current_process()?;
      let level = token.privilege_level()?;

      let status = match level {
        PrivilegeLevel::NotPrivileged => RunasCommand::new(install_path).status()?,
        _ => StdCommand::new(install_path).creation_flags(0x08000000).status()?,
      };

      if !status.success() {
        bail!("failed to install service with status {}", status.code().unwrap());
      }

      Ok(())
    }

    pub async fn uninstall_service() -> Result<()> {
      let binary_path = dirs::service_path();
      let uninstall_path = binary_path.with_file_name("uninstall-service.exe");

      if !uninstall_path.exists() {
        bail!("Uninstaller exe not found");
      }

      let token = Token::with_current_process()?;
      let level = token.privilege_level()?;

      let status = match level {
        PrivilegeLevel::NotPrivileged => RunasCommand::new(uninstall_path).status()?,
        _ => StdCommand::new(uninstall_path).creation_flags(0x08000000).status()?,
      };

      if !status.success() {
        bail!("Failed to uninstall service with status {}", status.code().unwrap());
      }

      Ok(())
    }

    pub async fn start_service() -> Result<()> {
      let token = Token::with_current_process()?;
      let level = token.privilege_level()?;

      let args = ["start", SERVICE_NAME];

      let status = match level {
        PrivilegeLevel::NotPrivileged => RunasCommand::new("sc").args(&args).status()?,
        _ => StdCommand::new("sc").args(&args).status()?,
      };

      match status.success() {
        true => Ok(()),
        false => bail!("Failed to start service with status {}", status.code().unwrap()),
      }
    }

    pub async fn stop_service() -> Result<()> {
      let url = format!("{SERVICE_URL}/stop_service");
      let res = reqwest::ClientBuilder
        ::new()
        .no_proxy()
        .build()?
        .post(url)
        .send().await?
        .json::<JsonResponse>().await
        .context("failed to connect to the service")?;

      if res.code != 0 {
        bail!(res.msg);
      }

      Ok(())
    }
  }
}