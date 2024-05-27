import {
  AppBar,
  Box,
  Stack,
  Tab,
  Tabs,
  Toolbar,
  Typography,
} from "@mui/material";
import { useEffect, useState } from "react";
import Iconify from "./Iconify";
import { Link } from "react-router-dom";
import { useLocation } from "react-router-dom";

const Navbar = () => {
  const [currentTab, setCurrentTab] = useState(0);

  let location = useLocation();

  useEffect(() => {
    if (location.pathname === "/") {
      setCurrentTab(0);
    } else if (location.pathname.startsWith("/streams")) {
      setCurrentTab(1);
    } else if (location.pathname.startsWith("/workers")) {
      setCurrentTab(2);
    } else if (location.pathname.startsWith("/settings")) {
      setCurrentTab(3);
    }
  }, [location]);

  return (
    <AppBar position="static">
      <Toolbar>
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="space-between"
          spacing={1}
          width={"100%"}
        >
          <Stack
            direction="column"
            alignItems="stretch"
            justifyContent="stretch"
            sx={{
              textDecoration: "none",
            }}
            component={Link}
            to="/"
            color="inherit"
          >
            <Typography
              variant="h6"
              sx={{
                flexGrow: 1,
                display: { xs: "none", sm: "none", md: "inline" },
                fontFamily: "monospace",
                fontWeight: "bold",
              }}
            >
              gasket
            </Typography>
            <Box
              sx={{
                height: "3px",
                background: "linear-gradient(90deg, #FFA500, #f00c93)",
                width: "100%",
              }}
            ></Box>
          </Stack>
          <Box sx={{ flexGrow: 1 }} />
          <Tabs value={currentTab}>
            <Tab
              label="Dashboard"
              icon={<Iconify icon="material-symbols:dashboard" />}
              iconPosition="start"
              component={Link}
              to="/"
            />
            <Tab
              label="Streams"
              icon={<Iconify icon="mdi:video" />}
              iconPosition="start"
              component={Link}
              to="/streams"
            />
            <Tab
              label="Workers"
              icon={<Iconify icon="mdi:worker" />}
              iconPosition="start"
              component={Link}
              to="/workers"
            />
            <Tab
              label="Settings"
              icon={<Iconify icon="mdi:cog" />}
              iconPosition="start"
              component={Link}
              to="/settings"
            />
          </Tabs>
        </Stack>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;
