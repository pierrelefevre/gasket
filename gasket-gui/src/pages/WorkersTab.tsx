import {
  Card,
  CardContent,
  CardHeader,
  Stack,
  Typography,
} from "@mui/material";
import useLb from "../hooks/useLb";
import WorkerCard from "../component/WorkerCard";
import WorkerForm from "../component/WorkerForm";
import { Worker } from "../types";

const WorkersTab = () => {
  const { workers, addWorker } = useLb();

  return (
    <>
      <Typography variant="h5" sx={{ pt: 0, pb: 2 }}>
        Workers - {workers.length}
      </Typography>
      <Card sx={{ p: 1, my: 2 }}>
        <CardHeader title="Register worker"></CardHeader>
        <CardContent>
          <WorkerForm callback={addWorker} />
        </CardContent>
      </Card>
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
              controls
            />
          ))}
        </Stack>
      </Stack>
    </>
  );
};

export default WorkersTab;
