import { Paper } from "@mui/material";
import { useTranslation } from "react-i18next";
import Notice from "../components/base/base-notice";
import BasePage from "../components/base/base-page";
import SettingApp from "../components/setting/setting-app";
import SettingSystem from "../components/setting/setting-system";

const SettingPage = () => {
  const { t } = useTranslation();

  const onError = (err: any) => {
    Notice.error(err?.message || err.toString());
  };

  return (
    <BasePage title={t("Settings")}>
      <Paper sx={{ borderRadius: 1, boxShadow: 2, mb: 3 }}>
        <SettingSystem onError={onError} />
      </Paper>

      <Paper sx={{ borderRadius: 1, boxShadow: 2 }}>
        <SettingApp onError={onError} />
      </Paper>
    </BasePage>
  );
};

export default SettingPage;
