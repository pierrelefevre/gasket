import useLb from "../hooks/useLb";
import { Box, Stack, Typography } from "@mui/material";
import { Link } from "react-router-dom";
import WorkerCard from "../component/WorkerCard";
import StreamCard from "../component/StreamCard";
import { Stream, Worker } from "../types";

const DashboardTab = () => {
  const { streams, workers } = useLb();

  const renderStreamColor = (stream: Stream) => {
    let color = "#6cf542";

    if (stream.status === "error") return "red";

    stream.output.forEach((output) => {
      if (output.status === "error") return "red";
    });

    if (!stream.enabled) return "grey";

    if (stream.status === "Creating") return "orange";

    return color;
  };

  return (
    <>
      <Typography variant="h5" sx={{ pt: 0, pb: 2 }}>
        <Link
          to="/workers"
          style={{ color: "inherit", textDecoration: "none" }}
        >
          Workers - {workers.length}
        </Link>
      </Typography>
      <Stack direction="column" spacing={2}>
        <Stack
          direction="row"
          spacing={2}
          flexWrap={"wrap"}
          useFlexGap
          alignItems={"flex-start"}
        >
          {workers.map((worker: Worker) => (
            <WorkerCard
              worker={worker}
              key={`dashboard-worker-overview-${worker.id}`}
            />
          ))}
          {workers.length === 0 && (
            <Typography variant="caption" color="grey">
              No workers registered
            </Typography>
          )}
        </Stack>

        <Typography variant="h5" sx={{ pt: 5, pb: 0 }}>
          <Link
            to="/streams"
            style={{ color: "inherit", textDecoration: "none" }}
          >
            Streams - {streams.length}
          </Link>
        </Typography>
        <Stack
          direction={"row"}
          flexWrap={"wrap"}
          useFlexGap
          alignItems={"flex-start"}
          justifyContent={"flex-start"}
          spacing={1}
          sx={{ pb: 2 }}
        >
          {streams.map((stream: Stream) => (
            <Link
              to={`/streams#${stream.id}`}
              key={`dashboard-stream-link-${stream.id}`}
            >
              <Box
                component="div"
                sx={{
                  height: 16,
                  width: 16,
                  background: renderStreamColor(stream),
                  transition:
                    "background-color 1s ease-in-out, opacity 1s ease-in-out",
                  animation:
                    renderStreamColor(stream) === "orange"
                      ? "pulse 2s cubic-bezier(.4,0,.6,1) infinite"
                      : "none",
                }}
              />
            </Link>
          ))}
        </Stack>
        <Stack
          direction="row"
          spacing={2}
          flexWrap={"wrap"}
          useFlexGap
          justifyContent={"space-between"}
        >
          {streams.slice(0, 6).map((stream: Stream) => (
            <StreamCard
              key={`dashboard-stream-card-${stream.id}`}
              stream={stream}
            />
          ))}

          {streams.length > 6 && (
            <Link
              to="/streams"
              style={{ color: "inherit", textDecoration: "none" }}
            >
              <Typography variant="h5" color="grey">
                +{streams.length - 6} more...
              </Typography>
            </Link>
          )}

          {streams.length === 0 && (
            <Typography variant="caption" color="grey">
              No streams created
            </Typography>
          )}
        </Stack>
      </Stack>
    </>
  );
};

export default DashboardTab;
