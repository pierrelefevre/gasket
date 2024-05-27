import { Navigate, useRoutes } from "react-router-dom";
import PageLayout from "./layout/PageLayout";
import DashboardTab from "./pages/DashboardTab";
import StreamsTab from "./pages/StreamsTab";
import WorkersTab from "./pages/WorkersTab";
import SettingsTab from "./pages/SettingsTab";

// ----------------------------------------------------------------------

export default function Router() {
  return useRoutes([
    {
      path: "/",
      element: <PageLayout />,
      children: [
        { path: "/", element: <DashboardTab /> },
        { path: "/streams", element: <StreamsTab /> },
        { path: "/workers", element: <WorkersTab /> },
        { path: "/settings", element: <SettingsTab /> },
      ],
    },
    {
      path: "*",
      element: <Navigate to="/" replace />,
    },
  ]);
}
