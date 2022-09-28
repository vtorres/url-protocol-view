use crate::utils::{ config, dirs };
use anyhow::Result;
use serde::{ Deserialize, Serialize };

// `config.yaml` schema
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct URLProtocolView {
  pub language: Option<String>,
  pub theme_mode: Option<String>,
  pub theme_blur: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enable_auto_launch: Option<bool>,
  pub enable_silent_start: Option<bool>,
  pub theme_setting: Option<AppTheme>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AppTheme {
  pub primary_color: Option<String>,
  pub secondary_color: Option<String>,
  pub primary_text: Option<String>,
  pub secondary_text: Option<String>,
  pub info_color: Option<String>,
  pub error_color: Option<String>,
  pub warning_color: Option<String>,
  pub success_color: Option<String>,
  pub font_family: Option<String>,
  pub css_injection: Option<String>,
}

impl URLProtocolView {
  pub fn new() -> Self {
    config::read_yaml::<URLProtocolView>(dirs::app_path())
  }

  pub fn save_app_config(&self) -> Result<()> {
    config::save_yaml(dirs::app_path(), self, Some("# The Config\n\n"))
  }

  pub fn patch_app_config(&mut self, patch: URLProtocolView) -> Result<()> {
    if patch.language.is_some() {
      self.language = patch.language;
    }
    if patch.theme_mode.is_some() {
      self.theme_mode = patch.theme_mode;
    }
    if patch.theme_blur.is_some() {
      self.theme_blur = patch.theme_blur;
    }
    if patch.theme_setting.is_some() {
      self.theme_setting = patch.theme_setting;
    }
    if patch.enable_silent_start.is_some() {
      self.enable_silent_start = patch.enable_silent_start;
    }
    if patch.enable_auto_launch.is_some() {
      self.enable_auto_launch = patch.enable_auto_launch;
    }

    self.save_app_config()
  }
}