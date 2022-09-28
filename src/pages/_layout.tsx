import i18next from "i18next";
import useSWR, { SWRConfig, useSWRConfig } from "swr";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Route, Routes } from "react-router-dom";
import { alpha, List, Paper, ThemeProvider } from "@mui/material";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { routers } from "./_routers";
import { getAxios } from "../services/api";
import { getAppConfig } from "../services/cmds";
import { ReactComponent as LogoSvg } from "../assets/image/logo.svg";
import LayoutItem from "../components/layout/layout-item";
import LayoutControl from "../components/layout/layout-control";
import UpdateButton from "../components/layout/update-button";
import useCustomTheme from "../components/layout/use-custom-theme";
import getSystem from "../utils/get-system";

const OS = getSystem();

const Layout = () => {
  const { t } = useTranslation();
  const { mutate } = useSWRConfig();
  const { theme } = useCustomTheme();
  const { data: config } = useSWR("getAppConfig", getAppConfig);
  const { theme_blur, language } = config || {};

  useEffect(() => {
    window.addEventListener("keydown", (e) => {
      if (e.key === "Escape") appWindow.close();
    });

    listen("protocolview://refresh-app-config", () => mutate("getAppConfig"));
  }, []);

  useEffect(() => {
    if (language) i18next.changeLanguage(language);
  }, [language]);

  return (
    <SWRConfig value={{}}>
      <ThemeProvider theme={theme}>
        <Paper
          square
          elevation={0}
          className={`${OS} layout`}
          onPointerDown={(e: any) => {
            if (e.target?.dataset?.windrag) appWindow.startDragging();
          }}
          onContextMenu={(e) => {
            // Only prevent it on Windows
            if (OS === "windows") e.preventDefault();
          }}
          sx={[
            ({ palette }) => ({
              bgcolor: alpha(palette.background.paper, theme_blur ? 0.8 : 1),
            }),
          ]}
        >
          <div className="layout__left" data-windrag>
            <div className="the-logo" data-windrag>
              <LogoSvg />

              <UpdateButton className="the-newbtn" />
            </div>

            <List className="the-menu">
              {routers.map((router) => (
                <LayoutItem key={router.label} to={router.link}>
                  {t(router.label)}
                </LayoutItem>
              ))}
            </List>
          </div>

          <div className="layout__right" data-windrag>
            <div className="the-bar">
              <LayoutControl />
            </div>

            <div className="the-content">
              <Routes>
                {routers.map(({ label, link, ele: Ele }) => (
                  <Route key={label} path={link} element={<Ele />} />
                ))}
              </Routes>
            </div>
          </div>
        </Paper>
      </ThemeProvider>
    </SWRConfig>
  );
};

export default Layout;
