import LogsPage from "./logs";
import SettingsPage from "./settings";

export const routers = [
  {
    label: "Protocols",
    link: "/",
    ele: LogsPage,
  },
  {
    label: "Settings",
    link: "/settings",
    ele: SettingsPage,
  },
];
