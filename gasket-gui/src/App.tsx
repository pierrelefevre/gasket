import { BrowserRouter } from "react-router-dom";
import { LbContextProvider } from "./context/LbContext";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import Router from "./Router";
import { SnackbarProvider, closeSnackbar } from "notistack";
import { CookiesProvider } from "react-cookie";
import { IconButton, PaletteMode } from "@mui/material";
import Iconify from "./component/Iconify";
import "./style/custom.css";
const palette = {
  palette: {
    mode: "dark" as PaletteMode,
    primary: {
      main: "#FFA500",
    },
    // make more if needed https://meyerweb.com/eric/tools/color-blend/
    // custom: ["#FFA500", "#F8594A", "#F00C93"],
  },
};
const darkTheme = createTheme(palette);

function App() {
  return (
    <BrowserRouter>
      <CookiesProvider defaultSetOptions={{ path: "/", sameSite: "strict" }}>
        <LbContextProvider>
          <ThemeProvider theme={darkTheme}>
            <CssBaseline />
            <SnackbarProvider
              maxSnack={5}
              anchorOrigin={{ vertical: "bottom", horizontal: "left" }}
              autoHideDuration={3000}
              action={(snack) => (
                <IconButton
                  onClick={() => closeSnackbar(snack)}
                  color="inherit"
                >
                  <Iconify icon="material-symbols:close" />
                </IconButton>
              )}
              dense
              preventDuplicate
            >
              <Router />
            </SnackbarProvider>
          </ThemeProvider>
        </LbContextProvider>
      </CookiesProvider>
    </BrowserRouter>
  );
}

export default App;
