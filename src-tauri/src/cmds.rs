use crate::{ core::{ Core, URLProtocolView }, utils::{ dirs } };
use crate::{ wrap_err };
use anyhow::Result;
use tauri::{ State };
use serde::{ Deserialize, Serialize };
use winreg::enums::*;
use winreg::RegKey;

type CmdResult<T = ()> = Result<T, String>;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Registry {
  path: String,
  description: String,
  command: String,
  r#type: String,
  modified_at: String,
}

#[tauri::command]
pub fn get_protocol_handlers() -> Result<Vec<Registry>, String> {
  const URL_PROTOCOL_REGISTRY_IDENTIFIER: &str = "URL Protocol";
  const URL_PROTOCOL_REGISTRY_STRUCTURE: &str = "\\Shell\\Open\\Command";
  const URL_PROTOCOL_PLUGGABLE_TYPE: &str = "Pluggable";
  const URL_PROTOCOL_EXECUTABLE_TYPE: &str = "Executable";
  const HKEY_CLASSES_ROOT_IDENTIFIER: &str = "HKEY_CLASSES_ROOT";

  // All registries -
  let mut all_registries: Vec<Registry> = vec![];

  // Registries from HKEY_CLASSES_ROOT
  let hkey_classes_root: RegKey = RegKey::predef(HKEY_CLASSES_ROOT);

  // Fetch all keys from HKEY_CLASSES_ROOT path
  let hkey_classes_root_keys = hkey_classes_root.enum_keys().map(|x| x.unwrap());

  // Loop through all keys from this HKEY_CLASSES_ROOT path
  hkey_classes_root_keys.for_each(|registry: String| {
    // Open registry key
    let registry_key: RegKey = hkey_classes_root.open_subkey(&registry).unwrap();

    // Loop through the existing key,values inside
    for (name, _) in registry_key.enum_values().map(|x| x.unwrap()) {
      let registry_key_path: String = format!("{}/{}", HKEY_CLASSES_ROOT_IDENTIFIER, registry);

      // Verify if is indeed an URL Protocol
      if name.eq(URL_PROTOCOL_REGISTRY_IDENTIFIER) {
        // Get the default value from this registry, the default value is a empty string
        let default_value = "";
        let registry_key_default_data: String = registry_key
          .get_value(default_value)
          .unwrap_or("".to_string());

        // All the URL Custom Protocol Scheme needs to follow this structure
        let registry_key_id: String = format!("{}{}", registry, URL_PROTOCOL_REGISTRY_STRUCTURE);

        // Match if follow this structure above
        let registry_key_command_data: String = match
          hkey_classes_root.open_subkey(&registry_key_id)
        {
          Ok(registry_item) => registry_item.get_value(default_value).unwrap_or("".to_string()),
          Err(_) => "".to_string(),
        };

        // When there is no command on the registry\\Shell\\Open\\Command
        // Its a URL_PROTOCOL_PLUGGABLE_TYPE otherwise is a URL_PROTOCOL_EXECUTABLE_TYPE
        let registry_type: &str = if registry_key_command_data.is_empty() {
          URL_PROTOCOL_PLUGGABLE_TYPE
        } else {
          URL_PROTOCOL_EXECUTABLE_TYPE
        };

        // Query the registry for the latest write time
        let registry_latest_change = registry_key
          .query_info()
          .unwrap()
          .get_last_write_time_system();

        // Format the latest write time
        let registry_formatted_latest_change = format!(
          "{}-{:02}-{:02} {:02}:{:02}:{:02}",
          registry_latest_change.wYear,
          registry_latest_change.wMonth,
          registry_latest_change.wDay,
          registry_latest_change.wHour,
          registry_latest_change.wMinute,
          registry_latest_change.wSecond
        );

        // Push to the registry store array
        all_registries.push(Registry {
          path: registry_key_path,
          description: registry_key_default_data,
          command: registry_key_command_data,
          r#type: registry_type.to_string(),
          modified_at: registry_formatted_latest_change,
        });
      }
    }
  });

  // Return all the protocols registries
  Ok(all_registries)
}

#[tauri::command]
pub fn get_app_config(core: State<'_, Core>) -> CmdResult<URLProtocolView> {
  let app_config = core.app.lock();

  Ok(app_config.clone())
}

#[tauri::command]
pub fn patch_app_config(
  payload: URLProtocolView,
  app_handle: tauri::AppHandle,
  core: State<'_, Core>
) -> CmdResult {
  wrap_err!(core.patch_app_config(payload, &app_handle))
}

#[tauri::command]
pub fn open_app_dir() -> Result<(), String> {
  let app_dir = dirs::app_home_dir();

  wrap_err!(open::that(app_dir))
}

#[cfg(windows)]
pub mod service {
  use super::*;

  #[tauri::command]
  pub async fn start_service() -> Result<(), String> {
    wrap_err!(crate::core::Service::start_service().await)
  }

  #[tauri::command]
  pub async fn stop_service() -> Result<(), String> {
    wrap_err!(crate::core::Service::stop_service().await)
  }

  #[tauri::command]
  pub async fn install_service() -> Result<(), String> {
    wrap_err!(crate::core::Service::install_service().await)
  }

  #[tauri::command]
  pub async fn uninstall_service() -> Result<(), String> {
    wrap_err!(crate::core::Service::uninstall_service().await)
  }
}