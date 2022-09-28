import { useEffect, useMemo, useRef, useState } from "react";
import { useRecoilState } from "recoil";
import { Box, Button, CircularProgress, MenuItem, Paper, Select, TextField } from "@mui/material";
import { TableVirtuoso, Virtuoso } from "react-virtuoso";
import { useTranslation } from "react-i18next";
import { atomLogData } from "../services/states";
import BasePage from "../components/base/base-page";
import Registry from "../components/log/log-item";
import { getURLProtocolHandlers } from "../services/cmds";

export const useEffectOnce = (effect: () => void | (() => void)) => {
  const destroyFunc = useRef<void | (() => void)>();
  const effectCalled = useRef(false);
  const renderAfterCalled = useRef(false);
  const [_, setVal] = useState<number>(0);

  if (effectCalled.current) {
    renderAfterCalled.current = true;
  }

  useEffect(() => {
    if (!effectCalled.current) {
      destroyFunc.current = effect();
      effectCalled.current = true;
    }

    setVal((val) => val + 1);

    return () => {
      if (!renderAfterCalled.current) {
        return;
      }
      if (destroyFunc.current) {
        destroyFunc.current();
      }
    };
  }, []);
};

const LogPage = () => {
  const { t } = useTranslation();
  const [logData, setLogData] = useRecoilState(atomLogData);
  const [logState, setLogState] = useState("all");
  const [filterText, setFilterText] = useState("");
  const [isLoading, setLoading] = useState(true);

  const filterLogs = useMemo(() => {
    return logData.filter((data) => {
      return (
        (
          data.path.includes(filterText) ||
          data.command.includes(filterText) ||
          data.description.includes(filterText)
        ) &&
        (logState === "all" ? true : data.type.toLowerCase().includes(logState))
      );
    });
  }, [logData, logState, filterText]);

  const fetchURLProtocolHandlers = () => {
    setLoading(true);

    getURLProtocolHandlers().then((info: any) => {
      setLogData(info);
    });

    setTimeout(() => {
      setLoading(false);
    }, 500);
  }

  useEffectOnce(() => {
    fetchURLProtocolHandlers();
  });

  const header = isLoading === true ?
    (
      <Button
        size="small"
        sx={{ mt: 1 }}
        variant="contained"
      >
        <CircularProgress size={20} color="inherit" />
      </Button>
    ) :
    (
      <Button
        size="small"
        sx={{ mt: 1 }}
        variant="contained"
        onClick={() => { fetchURLProtocolHandlers() }}
      >
        {t("Refresh")}
      </Button>
    )

  return (
    <BasePage
      title={t("Protocols")}
      contentStyle={{ height: "100%" }}
      header={header}
    >
      <Paper sx={{ boxSizing: "border-box", boxShadow: 2, height: "100%" }}>
        <Box
          sx={{
            pt: 1,
            mb: 0.5,
            mx: "12px",
            height: "36px",
            display: "flex",
            alignItems: "center",
          }}
        >
          <Select
            disabled={false}
            size="small"
            autoComplete="off"
            value={logState}
            onChange={(e) => setLogState(e.target.value)}
            sx={{ width: 120, mr: 1, '[role="button"]': { py: 0.65 } }}
          >
            <MenuItem value="all">{t("All").toUpperCase()}</MenuItem>
            <MenuItem value="executable">{t("Executable").toUpperCase()}</MenuItem>
            <MenuItem value="pluggable">{t("Pluggable").toUpperCase()}</MenuItem>
          </Select>

          <TextField
            hiddenLabel
            fullWidth
            size="small"
            autoComplete="off"
            variant="outlined"
            placeholder={t("Filter")}
            value={filterText}
            onChange={(e) => setFilterText(e.target.value)}
            sx={{ input: { py: 0.65, px: 1.25 } }}
          />
        </Box>
        <Box height="calc(100% - 50px)" style={{ margin: `12px` }}>
          <TableVirtuoso
            fixedHeaderContent={() => (
              <tr>
                <th>{t("Name")}</th>
                <th>{t("Description")}</th>
                <th>{t("Command")}</th>
                <th>{t("ModifiedAt")}</th>
              </tr>
            )}
            data={filterLogs}
            itemContent={(_, item) => <Registry value={item} />}
          />
        </Box>
      </Paper>
    </BasePage>
  );
};

export default LogPage;
