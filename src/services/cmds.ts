import { invoke } from "@tauri-apps/api/tauri";
import { CmdType } from "./types";
import Notice from "../components/base/base-notice";

export async function getURLProtocolHandlers() {
  return invoke<void>("get_protocol_handlers");
}

export async function getAppConfig() {
  return invoke<CmdType.AppConfig>("get_app_config");
}

export async function patchAppConfig(payload: CmdType.AppConfig) {
  return invoke<void>("patch_app_config", { payload });
}

export async function openAppDir() {
  return invoke<void>("open_app_dir").catch((err) =>
    Notice.error(err?.message || err.toString(), 1500)
  );
}

export async function startService() {
  return invoke<void>("start_service");
}

export async function stopService() {
  return invoke<void>("stop_service");
}

export async function installService() {
  return invoke<void>("install_service");
}

export async function uninstallService() {
  return invoke<void>("uninstall_service");
}
