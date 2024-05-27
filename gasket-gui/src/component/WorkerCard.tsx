import {
  Backdrop,
  Box,
  Card,
  CardActions,
  CardContent,
  Drawer,
  IconButton,
  LinearProgress,
  Stack,
  Tooltip,
  Typography,
} from "@mui/material";
import Iconify from "./Iconify";
import CircularProgressWithLabel from "./CircularProgressWithLabel";
import useLb from "../hooks/useLb";
import { useState } from "react";
import WorkerForm from "./WorkerForm";
import { Worker } from "../types";

const WorkerCard = ({
  worker,
  controls = false,
}: {
  worker: Worker;
  controls?: boolean;
}) => {
  const { removeWorker, updateWorker } = useLb();
  const [editing, setEditing] = useState(false);

  const handleEdit = () => {
    setEditing(true);
    console.log("Edit worker");
  };

  const editCallback = (worker: Worker) => {
    updateWorker(worker.id, worker);
    setEditing(false);
  };

  return (
    <>
      <Drawer
        anchor={"right"}
        open={editing}
        onClose={() => setEditing(false)}
        slots={{ backdrop: Backdrop }}
        slotProps={{
          backdrop: {
            sx: {
              background: "rgba(0, 0, 0, 0.4)",
              backdropFilter: "blur(3px)",
            },
          },
        }}
      >
        <Box sx={{ p: 2, maxWidth: 700, minWidth: 400 }}>
          <Stack
            direction="row"
            alignItems="center"
            justifyContent="space-between"
          >
            <Typography variant="h5" sx={{ my: 3 }}>
              Edit worker
            </Typography>
            <IconButton onClick={() => setEditing(false)}>
              <Iconify icon="mdi:close" />
            </IconButton>
          </Stack>
          <WorkerForm worker={worker} callback={editCallback} editing />
        </Box>
      </Drawer>
      <Card
        key={`dashboard-worker-overview-${worker.id}`}
        sx={{
          border: 5,
          borderColor:
            (worker.status === "Up" && "#6cf542") ||
            (worker.status === "Crashed" && "red") ||
            (worker.status === "Configuring" && "orange") ||
            null,
        }}
      >
        <CardContent>
          <Stack direction="column" spacing={1} sx={{ px: 1, pb: 1 }}>
            <Typography variant="body1" sx={{ pt: 0.5, pb: 1 }}>
              {worker.host}
            </Typography>

            {worker.public_ip && (
              <Stack direction="row" spacing={2} alignItems={"center"}>
                <Iconify icon="mdi:globe" />

                <Typography variant="caption" pt={0.2}>
                  {worker.public_ip}
                </Typography>
              </Stack>
            )}

            {worker.status === "Configuring" ? (
              <LinearProgress />
            ) : (
              <>
                <Stack direction="row" spacing={2} alignItems={"center"}>
                  <Iconify icon="mdi:video" />

                  <Typography variant="caption" pt={0.2}>
                    {worker.codecs.join(", ") + " via " + worker.encoder}
                  </Typography>
                </Stack>

                {worker.stats && (
                  <Stack direction="column" spacing={2}>
                    {/* <Stack direction="row" spacing={2} alignItems={"center"}>
                      <Stack direction="row" spacing={2} alignItems={"center"}>
                        <Iconify icon="tabler:activity-heartbeat" />
                        <Typography variant="caption" pt={0.2}>
                          System util
                        </Typography>
                      </Stack>

                      <CircularProgressWithLabel
                        value={worker.stats?.utilization}
                      />
                    </Stack> */}

                    {worker.stats.devices?.length > 0 && (
                      <Stack direction="row" spacing={2} alignItems={"center"}>
                        <Iconify icon="mdi:gpu" />
                        <Typography variant="caption" pt={0.2}>
                          Load
                        </Typography>

                        {worker.stats.devices?.map((deviceUtil, index) => (
                          <CircularProgressWithLabel
                            value={deviceUtil}
                            key={worker.id + "stats" + index}
                          />
                        ))}
                      </Stack>
                    )}
                  </Stack>
                )}
              </>
            )}
          </Stack>
        </CardContent>
        {controls && (
          <CardActions>
            <Stack
              direction="row"
              spacing={1}
              alignItems={"center"}
              justifyContent={"flex-end"}
              width={"100%"}
            >
              <Tooltip title="Remove worker">
                <IconButton
                  size="small"
                  onClick={() => removeWorker(worker.id)}
                >
                  <Iconify icon="mdi:trash" />
                </IconButton>
              </Tooltip>
              <Tooltip title="Edit worker">
                <IconButton size="small" onClick={handleEdit}>
                  <Iconify icon="mdi:pencil" />
                </IconButton>
              </Tooltip>
            </Stack>
          </CardActions>
        )}
      </Card>
    </>
  );
};
export default WorkerCard;
