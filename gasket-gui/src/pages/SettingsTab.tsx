import { useEffect } from "react";
import { useCookies } from "react-cookie";
import {
  Typography,
  Stack,
  CircularProgress,
  OutlinedInput,
  FormControl,
  FormHelperText,
  InputAdornment,
} from "@mui/material";
import Iconify from "../component/Iconify";
import { useState } from "react";
import { useTheme } from "@mui/material";
import { getLb } from "../api/gasket-lb";
import { Uri } from "../types";

const SettingsTab = () => {
  const [cookies, setCookie] = useCookies(["gasket_api_url"]);
  const [apiState, setApiState] = useState("loading");
  const [apiResponse, setApiResponse] = useState<
    { server: string } | undefined | null
  >(null);
  const [ping, setPing] = useState(-1);
  const theme = useTheme();

  const checkApiState = async (url: Uri) => {
    let start = new Date().getTime();

    getLb(url)
      .then((response) => {
        setApiState("success");
        setApiResponse(response);
      })
      .catch(() => {
        setApiState("error"), setApiResponse(null);
      })
      .finally(() => {
        let end = new Date().getTime();
        setPing(end - start);
      });
  };

  useEffect(() => {
    if (!cookies.gasket_api_url) {
      let url = "http://localhost:8888";
      if (import.meta.env.VITE_API_URL) url = import.meta.env.VITE_API_URL;

      setCookie("gasket_api_url", url);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    checkApiState(cookies.gasket_api_url);
  }, [cookies.gasket_api_url]);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setApiState("loading");
    setCookie("gasket_api_url", event.target.value);
    checkApiState(event.target.value);
  };

  const renderColor = () => {
    if (apiState === "loading") return "info";
    if (apiState === "success") return "success";
    if (apiState === "error") return "error";
  };

  const renderIcon = () => {
    if (apiState === "loading") return <CircularProgress />;
    if (apiState === "success")
      return (
        <Iconify
          icon="mdi:check-circle"
          sx={{ fontSize: 24 }}
          color={theme.palette.success.main}
        />
      );
    if (apiState === "error")
      return (
        <Iconify
          icon="mdi:close-circle"
          sx={{ fontSize: 24 }}
          color={theme.palette.error.main}
        />
      );
  };

  return (
    <>
      <Typography variant="h5" sx={{ pt: 0, pb: 2 }}>
        Settings
      </Typography>
      <Stack direction="column" spacing={2}>
        <Stack direction="row" spacing={2} useFlexGap alignItems={"center"}>
          <FormControl fullWidth variant="outlined">
            <FormHelperText id="apiurl-helper-text">API URL</FormHelperText>
            <OutlinedInput
              id="apiurl-adornment-weight"
              value={cookies.gasket_api_url}
              onChange={handleChange}
              error={apiState === "error"}
              endAdornment={
                <InputAdornment position="end">{renderIcon()}</InputAdornment>
              }
              aria-describedby="apiurl-helper-text"
              inputProps={{
                "aria-label": "weight",
              }}
              color={renderColor()}
            />
          </FormControl>
        </Stack>
        {apiResponse && ping >= 0 && (
          <Typography
            variant="caption"
            color="textSecondary"
            sx={{ whiteSpace: "nowrap" }}
          >
            {apiResponse.server} - Ping: {ping}ms
          </Typography>
        )}
      </Stack>
    </>
  );
};

export default SettingsTab;
