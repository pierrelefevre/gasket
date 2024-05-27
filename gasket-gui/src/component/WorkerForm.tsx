import { Button, Stack, TextField } from "@mui/material";
import { useState } from "react";

const WorkerForm = ({
  worker = {
    protocol: "http",
    host: "localhost",
    public_ip: "127.0.0.1",
  },
  callback,
  editing = false,
}: {
  worker?: any;
  callback?: any;
  editing?: boolean;
}) => {
  const [createWorker, setCreateWorker] = useState(worker);

  return (
    <Stack direction="column" spacing={2} alignItems={"flex-start"}>
      <TextField
        label="Protocol"
        variant="outlined"
        placeholder="http"
        value={createWorker.protocol}
        onChange={(e) =>
          setCreateWorker({
            ...createWorker,
            protocol: e.target.value.trim(),
          })
        }
        fullWidth
        sx={{ maxWidth: 400 }}
      />
      <TextField
        label="URL"
        variant="outlined"
        value={createWorker.host}
        onChange={(e) =>
          setCreateWorker({
            ...createWorker,
            host: e.target.value.trim(),
          })
        }
        fullWidth
        sx={{ maxWidth: 400 }}
      />
      <TextField
        label="Public IP"
        variant="outlined"
        value={createWorker.public_ip}
        onChange={(e) =>
          setCreateWorker({
            ...createWorker,
            public_ip: e.target.value.trim(),
          })
        }
        fullWidth
        sx={{ maxWidth: 400 }}
      />

      <Button
        variant="contained"
        color="primary"
        onClick={() => callback(createWorker)}
      >
        {editing ? "Update" : "Register"} worker
      </Button>
    </Stack>
  );
};

export default WorkerForm;
