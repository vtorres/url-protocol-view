import useSWR, { useSWRConfig } from "swr";
import { useTranslation } from "react-i18next";
import {
  ListItemText,
  Switch,
} from "@mui/material";
import {
  getAppConfig,
  patchAppConfig,
} from "../../services/cmds";
import { SettingList, SettingItem } from "./setting";
import { CmdType } from "../../services/types";
import GuardState from "./guard-state";

interface Props {
  onError?: (err: Error) => void;
}

const SettingSystem = ({ onError }: Props) => {
  const { t } = useTranslation();
  const { mutate } = useSWRConfig();
  const { data: config } = useSWR("getAppConfig", getAppConfig);

  const {
    enable_auto_launch,
    enable_silent_start,
  } = config ?? {};

  const onSwitchFormat = (_e: any, value: boolean) => value;

  const onChangeData = (patch: Partial<CmdType.AppConfig>) => {
    mutate("getAppConfig", { ...config, ...patch }, false);
  };

  return (
    <SettingList title={t("System Setting")}>
      <SettingItem>
        <ListItemText primary={t("Auto Launch")} />
        <GuardState
          value={enable_auto_launch ?? false}
          valueProps="checked"
          onCatch={onError}
          onFormat={onSwitchFormat}
          onChange={(e) => onChangeData({ enable_auto_launch: e })}
          onGuard={(e) => patchAppConfig({ enable_auto_launch: e })}
        >
          <Switch edge="end" />
        </GuardState>
      </SettingItem>

      <SettingItem>
        <ListItemText primary={t("Silent Start")} />
        <GuardState
          value={enable_silent_start ?? false}
          valueProps="checked"
          onCatch={onError}
          onFormat={onSwitchFormat}
          onChange={(e) => onChangeData({ enable_silent_start: e })}
          onGuard={(e) => patchAppConfig({ enable_silent_start: e })}
        >
          <Switch edge="end" />
        </GuardState>
      </SettingItem>
    </SettingList>
  );
};

export default SettingSystem;
