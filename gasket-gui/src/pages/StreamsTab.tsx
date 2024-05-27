import {
  Button,
  Card,
  CardContent,
  CardHeader,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import useLb from "../hooks/useLb";
import StreamCard from "../component/StreamCard";
import { useEffect, useState } from "react";
import { faker } from "@faker-js/faker";
import { Stream, Uuid } from "../types";

const StreamsTab = () => {
  const { streams, addStream } = useLb();
  const [createStream, setCreateStream] = useState({
    name: `${faker.word.verb()}-${faker.word.adjective()}-${faker.word.noun()}`,
    input: "data/test_loop.mp4",
    output: [],
  });
  const [highlighted, setHighlighted] = useState<Uuid | null>(null);

  useEffect(() => {
    const url = window.location.href;
    const streamId = url.split("#")[1];
    if (streamId) {
      // scroll to #streamId
      const element = document.getElementById(streamId);
      if (element) {
        element.scrollIntoView();
        setHighlighted(streamId);
      }
    }
    // eslint-disable-next-line
  }, []);

  return (
    <>
      <Typography variant="h5" sx={{ pt: 0, pb: 2 }}>
        Streams - {streams.length}
      </Typography>
      <Card sx={{ p: 1, my: 2 }}>
        <CardHeader title="Create stream" />
        <CardContent>
          <Stack direction="column" spacing={2} alignItems={"flex-start"}>
            <TextField
              label="Name"
              variant="outlined"
              fullWidth
              sx={{ maxWidth: 400 }}
              value={createStream.name}
              onChange={(e) =>
                setCreateStream({
                  ...createStream,
                  name: e.target.value.trim(),
                })
              }
            />
            <TextField
              label="Input"
              variant="outlined"
              fullWidth
              sx={{ maxWidth: 400 }}
              value={createStream.input}
              onChange={(e) =>
                setCreateStream({
                  ...createStream,
                  input: e.target.value.trim(),
                })
              }
            />
            <Button variant="contained" onClick={() => addStream(createStream)}>
              Create stream
            </Button>
            <Typography variant="caption" color="grey">
              Add outputs in the next step
            </Typography>
          </Stack>
        </CardContent>
      </Card>
      <Stack
        direction="row"
        spacing={2}
        flexWrap={"wrap"}
        useFlexGap
        justifyContent={"space-between"}
        alignItems={"flex-start"}
      >
        {streams.map((stream: Stream) => (
          <StreamCard
            key={`dashboard-stream-overview-${stream.id}`}
            stream={stream}
            controls
            highlighted={highlighted === stream.id}
          />
        ))}
      </Stack>
    </>
  );
};

export default StreamsTab;
