import { Box, Container, Typography } from "@mui/material";

import { Outlet } from "react-router-dom";
import Navbar from "../component/Navbar";

const PageLayout = () => {
  let release_branch = import.meta.env.VITE_RELEASE_BRANCH || "dev";
  let release_date = import.meta.env.VITE_RELEASE_DATE || "1970-01-01_00:00";
  let release_commit = (
    import.meta.env.VITE_RELEASE_COMMIT || "0000000"
  ).substring(0, 7);

  let release_info = `gasket-gui ${release_branch}-${release_date}-${release_commit}`;

  return (
    <>
      <Box sx={{ flexGrow: 1 }}>
        <Navbar />
      </Box>
      <Container
        sx={{ p: 5, minHeight: "calc(100vh - 200px)" }}
        maxWidth={"xl"}
      >
        <Outlet />
      </Container>
      <Box sx={{ textAlign: "center", my: 5 }}>
        <Typography sx={{ color: "grey", fontFamily: "monospace" }}>
          {release_info}
        </Typography>
      </Box>
    </>
  );
};

export default PageLayout;
