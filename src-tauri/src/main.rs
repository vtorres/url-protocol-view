mod cmds;
mod core;
mod utils;

use crate::{ utils::{ resolve } };

use tauri::{ api, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem };

fn main() -> std::io::Result<()> {
  #[cfg(target_os = "windows")]
  unsafe {
    use crate::utils::dirs;

    dirs::init_portable_flag();
  }

  let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("open_window", "Show"))
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"));

  #[allow(unused_mut)]
  let mut builder = tauri::Builder
    ::default()
    .manage(core::Core::new())
    .setup(|app| Ok(resolve::resolve_setup(app)))
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(move |app_handle, event| {
      match event {
        SystemTrayEvent::MenuItemClick { id, .. } =>
          match id.as_str() {
            "open_window" => {
              resolve::create_window(app_handle);
            }
            "quit" => {
              resolve::resolve_reset(app_handle);
              app_handle.exit(0);
            }
            _ => {}
          }
        #[cfg(target_os = "windows")]
        SystemTrayEvent::LeftClick { .. } => {
          resolve::create_window(app_handle);
        }
        _ => {}
      }
    })
    .invoke_handler(
      tauri::generate_handler![
        cmds::open_app_dir,
        cmds::get_app_config,
        cmds::patch_app_config,
        cmds::get_protocol_handlers,
        cmds::service::start_service,
        cmds::service::stop_service,
        cmds::service::install_service,
        cmds::service::uninstall_service
      ]
    );

  builder
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, e| {
      match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
          api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
          resolve::resolve_reset(app_handle);
          api::process::kill_children();
        }
        _ => {}
      }
    });

  Ok(())
}