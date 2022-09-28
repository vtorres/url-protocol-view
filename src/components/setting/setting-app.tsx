import useSWR, { useSWRConfig } from "swr";
import { useTranslation } from "react-i18next";
import {
  IconButton,
  ListItemText,
  MenuItem,
  Select,
  Switch,
  Typography,
} from "@mui/material";
import {
  getAppConfig,
  openAppDir,
  patchAppConfig,
} from "../../services/cmds";
import { ArrowForward } from "@mui/icons-material";
import { SettingList, SettingItem } from "./setting";
import { CmdType } from "../../services/types";
import { version } from "../../../package.json";
import PaletteSwitch from "./palette-switch";
import GuardState from "./guard-state";

interface Props {
  onError?: (err: Error) => void;
}

const SettingApp = ({ onError }: Props) => {
  const { t } = useTranslation();
  const { mutate } = useSWRConfig();
  const { data: config } = useSWR("getAppConfig", getAppConfig);
  const { theme_mode, theme_blur, language } = config ?? {};
  const onSwitchFormat = (_e: any, value: boolean) => value;
  const onChangeData = (patch: Partial<CmdType.AppConfig>) => {
    mutate("getAppConfig", { ...config, ...patch }, false);
  };

  return (
    <SettingList title={t("App")}>
      <SettingItem>
        <ListItemText primary={t("Theme Mode")} />
        <GuardState
          value={theme_mode === "dark"}
          valueProps="checked"
          onCatch={onError}
          onFormat={onSwitchFormat}
          onChange={(e) => onChangeData({ theme_mode: e ? "dark" : "light" })}
          onGuard={(e) =>
            patchAppConfig({ theme_mode: e ? "dark" : "light" })
          }
        >
          <PaletteSwitch edge="end" />
        </GuardState>
      </SettingItem>

      <SettingItem>
        <ListItemText primary={t("Theme Blur")} />
        <GuardState
          value={theme_blur ?? false}
          valueProps="checked"
          onCatch={onError}
          onFormat={onSwitchFormat}
          onChange={(e) => onChangeData({ theme_blur: e })}
          onGuard={(e) => patchAppConfig({ theme_blur: e })}
        >
          <Switch edge="end" />
        </GuardState>
      </SettingItem>

      <SettingItem>
        <ListItemText primary={t("Language")} />
        <GuardState
          value={language ?? "en"}
          onCatch={onError}
          onFormat={(e: any) => e.target.value}
          onChange={(e) => onChangeData({ language: e })}
          onGuard={(e) => patchAppConfig({ language: e })}
        >
          <Select size="small" sx={{ width: 135 }}>
            <MenuItem value="en">English</MenuItem>
            <MenuItem value="pt">Portuguese</MenuItem>
          </Select>
        </GuardState>
      </SettingItem>

      <SettingItem>
        <ListItemText primary={t("Open App Dir")} />
        <IconButton color="inherit" size="small" onClick={openAppDir}>
          <ArrowForward />
        </IconButton>
      </SettingItem>

      <SettingItem>
        <ListItemText primary={t("Version")} />
        <Typography sx={{ py: "6px" }}>v{version}</Typography>
      </SettingItem>
    </SettingList>
  );
};

export default SettingApp;
