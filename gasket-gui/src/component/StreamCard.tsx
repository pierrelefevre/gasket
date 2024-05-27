import {
  Box,
  Card,
  CardActions,
  CardContent,
  CardHeader,
  Divider,
  FormControl,
  FormControlLabel,
  IconButton,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Switch,
  TextField,
  TextareaAutosize,
  Tooltip,
  Typography,
} from "@mui/material";
import { useEffect, useState } from "react";
import Iconify from "./Iconify";
import useLb from "../hooks/useLb";
import StreamFlowchart from "./StreamFlowchart";
import { Link } from "react-router-dom";
import { Codec, Output, Stream } from "../types";

const StreamCard = ({
  stream,
  controls,
  highlighted,
}: {
  stream: Stream;
  controls?: boolean;
  highlighted?: boolean;
}) => {
  const { updateStream, removeStream } = useLb();

  const [showDetails, setShowDetails] = useState(false);
  const [editing, setEditing] = useState<boolean>(false);
  const [expanded, setExpanded] = useState("");

  useEffect(() => {
    if (highlighted) setEditing(true);
  }, [highlighted]);

  const initialOutput = {
    uri: "",
    codec: Codec.H264,
    options: {},
  };

  const [patch, setPatch] = useState<any>({ id: stream.id });
  const [newOutput, setNewOutput] = useState<any>(initialOutput);

  const updatePatch = (component: string, method: string, value: string) => {
    if (component === "input") {
      if (method === "edit") {
        if (!patch.input) setPatch({ ...patch, input: stream.input });
        setExpanded("input");
        return;
      }
    }
    if (component === "output") {
      if (method === "delete") {
        let outputs = [];
        if (!patch.output) outputs = stream.output;
        else outputs = patch.output;

        setPatch({
          ...patch,
          output: outputs.filter((out: Output) => out.id !== value),
        });
        return;
      }
      if (method === "edit") {
        let curr = stream.output.find((out) => out.id === value);
        if (!curr) return;

        setNewOutput({
          id: curr.id,
          uri: curr.uri,
          codec: curr.codec,
          options: curr.options,
        });
        setExpanded("output");

        return;
      }
    }
  };

  return (
    <Card
      id={stream.id}
      sx={{
        width: "49%",
        boxShadow: highlighted ? "0 0 0 2px rgba(255,255,255,0.5)" : "none",
      }}
    >
      {controls ? (
        <CardHeader title={stream.name} />
      ) : (
        <CardHeader
          title={stream.name}
          component={Link}
          to={"/streams#" + stream.id}
          sx={{ textDecoration: "none", color: "inherit" }}
        />
      )}
      <CardContent>
        <Stack direction={"column"} spacing={2}>
          <StreamFlowchart
            stream={stream}
            editing={editing}
            callback={updatePatch}
          />
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
            {editing ? (
              <>
                <Tooltip title="Delete stream">
                  <IconButton
                    size="small"
                    onClick={() => removeStream(stream.id)}
                  >
                    <Iconify icon="mdi:trash" />
                  </IconButton>
                </Tooltip>
                <Divider orientation="vertical" flexItem />
                {expanded !== "output" && (
                  <>
                    {stream.enabled ? (
                      <Tooltip title="Disable stream">
                        <IconButton
                          size="small"
                          onClick={() => {
                            setPatch({ id: stream.id, enabled: false });
                          }}
                        >
                          <Iconify icon="mdi:pause" />
                        </IconButton>
                      </Tooltip>
                    ) : (
                      <Tooltip title="Enable stream">
                        <IconButton
                          size="small"
                          onClick={() => {
                            setPatch({ id: stream.id, enabled: true });
                          }}
                        >
                          <Iconify icon="mdi:play" />
                        </IconButton>
                      </Tooltip>
                    )}

                    <Tooltip title="Add output">
                      <IconButton
                        size="small"
                        onClick={() => {
                          setNewOutput(initialOutput);
                          setExpanded("output");
                        }}
                      >
                        <Iconify icon="mdi:plus" />
                      </IconButton>
                    </Tooltip>
                  </>
                )}
                <Tooltip title="Apply changes">
                  <IconButton
                    size="small"
                    onClick={() => {
                      setEditing(false);
                      setPatch({ id: stream.id });
                      updateStream(stream.id, patch);
                    }}
                  >
                    <Iconify icon="mdi:content-save" />
                  </IconButton>
                </Tooltip>
                <Tooltip title="Undo changes">
                  <IconButton
                    size="small"
                    onClick={() => {
                      setEditing(false);
                      setPatch({ id: stream.id });
                    }}
                  >
                    <Iconify icon="mdi:undo" />
                  </IconButton>
                </Tooltip>
              </>
            ) : (
              <>
                {showDetails ? (
                  <Tooltip title="Hide details">
                    <IconButton
                      size="small"
                      onClick={() => setShowDetails(false)}
                    >
                      <Iconify icon="mdi:eye-off" />
                    </IconButton>
                  </Tooltip>
                ) : (
                  <Tooltip title="Show details">
                    <IconButton
                      size="small"
                      onClick={() => setShowDetails(true)}
                    >
                      <Iconify icon="mdi:eye" />
                    </IconButton>
                  </Tooltip>
                )}
                <Tooltip title="Edit stream">
                  <IconButton
                    size="small"
                    onClick={() => {
                      setEditing(true), setShowDetails(false);
                    }}
                  >
                    <Iconify icon="mdi:pencil" />
                  </IconButton>
                </Tooltip>
              </>
            )}
          </Stack>
        </CardActions>
      )}

      {!editing && showDetails && (
        <CardContent>
          <Stack direction={"column"} spacing={2}>
            <TextareaAutosize
              value={JSON.stringify(stream, null, 2)}
              readOnly
              style={{
                width: "100%",
                color: "#eee",
                background: "rgba(0, 0, 0, 0.2)",
                border: "none",
                padding: "1rem",
              }}
            />
          </Stack>
        </CardContent>
      )}

      {editing && expanded === "output" && (
        <CardContent>
          <Stack
            direction={"column"}
            spacing={2}
            sx={{ p: 3, background: "#181818" }}
          >
            <Stack
              direction="row"
              spacing={2}
              useFlexGap
              alignItems="center"
              flexWrap={"wrap"}
              justifyContent={"flex-start"}
            >
              <TextField
                label="Output URI"
                variant="outlined"
                sx={{ width: "30rem" }}
                value={newOutput.uri}
                onChange={(e) =>
                  setNewOutput({
                    ...newOutput,
                    uri: e.target.value.trim(),
                  })
                }
              />
              <FormControl>
                <InputLabel id="output-codec-select">Codec</InputLabel>
                <Select
                  label="Codec"
                  variant="outlined"
                  sx={{ width: "6rem" }}
                  value={newOutput.codec.toUpperCase()}
                  onChange={(e) =>
                    setNewOutput({
                      ...newOutput,
                      codec: e.target.value,
                    })
                  }
                  placeholder="Codec"
                >
                  <MenuItem value="H264">H.264</MenuItem>
                  <MenuItem value="H265">H.265</MenuItem>
                  <MenuItem value="AV1">AV1</MenuItem>
                </Select>
              </FormControl>

              <TextField
                label="Output Format"
                variant="outlined"
                value={newOutput.options?.output_format || ""}
                onChange={(e) => {
                  setNewOutput({
                    ...newOutput,
                    options: {
                      ...newOutput.options,
                      output_format: e.target.value.trim(),
                    },
                  });
                }}
              />

              <TextField
                label="Pixel Format"
                variant="outlined"
                value={newOutput.options?.pixel_format || ""}
                onChange={(e) => {
                  setNewOutput({
                    ...newOutput,
                    options: {
                      ...newOutput.options,
                      pixel_format: e.target.value.trim(),
                    },
                  });
                }}
              />

              <TextField
                label="Bitrate"
                variant="outlined"
                value={newOutput.options?.bitrate || ""}
                onChange={(e) => {
                  setNewOutput({
                    ...newOutput,
                    options: {
                      ...newOutput.options,
                      bitrate: e.target.value.trim(),
                    },
                  });
                }}
              />

              <TextField
                label="Framerate"
                variant="outlined"
                value={newOutput.options?.framerate || ""}
                onChange={(e) => {
                  setNewOutput({
                    ...newOutput,
                    options: {
                      ...newOutput.options,
                      framerate: e.target.value.trim(),
                    },
                  });
                }}
              />

              <TextField
                label="GOP Size"
                variant="outlined"
                value={newOutput.options?.gop_size || ""}
                onChange={(e) => {
                  setNewOutput({
                    ...newOutput,
                    options: {
                      ...newOutput.options,
                      gop_size: e.target.value.trim(),
                    },
                  });
                }}
              />
              <FormControlLabel
                sx={{
                  mx: 0,
                  height: 56,
                  pl: 2,
                  pr: 3,
                  border: "1px solid rgba(255, 255, 255, 0.23)",
                  borderRadius: "4px",
                }}
                control={
                  <Switch
                    checked={newOutput.options?.debug_text || false}
                    onChange={(e) => {
                      setNewOutput({
                        ...newOutput,
                        options: {
                          ...newOutput.options,
                          debug_text: e.target.checked,
                        },
                      });
                    }}
                  />
                }
                label="Debug Text"
              />
            </Stack>

            <Stack direction={"row"} spacing={2} alignItems={"center"}>
              <Box sx={{ flexGrow: 1 }} />
              <IconButton
                onClick={() => {
                  setExpanded("");

                  let outputs = [];
                  if (!patch.output) outputs = stream.output;
                  else outputs = patch.output;

                  let curr: any = {};
                  if (newOutput.id) {
                    curr = outputs.find(
                      (out: Output) => out.id === newOutput.id,
                    );
                    curr.uri = newOutput.uri;
                    curr.codec = newOutput.codec;
                    curr.options = newOutput.options;
                  } else {
                    outputs.push(newOutput);
                  }

                  setPatch({
                    ...patch,
                    output: outputs,
                  });
                }}
              >
                <Iconify icon="mdi:check" />
              </IconButton>
              <IconButton
                onClick={() => {
                  const { input, ...newPatch } = patch;
                  setPatch(newPatch);
                  setExpanded("");
                }}
              >
                <Iconify icon="mdi:close" />
              </IconButton>
            </Stack>
          </Stack>
        </CardContent>
      )}

      {editing && expanded === "input" && (
        <CardContent>
          <Stack direction={"row"} spacing={2} alignItems={"center"}>
            <TextField
              label="Input URI"
              variant="outlined"
              fullWidth
              sx={{ maxWidth: 400 }}
              value={patch.input}
              onChange={(e) =>
                setPatch({
                  ...patch,
                  input: e.target.value.trim(),
                })
              }
            />
            <IconButton
              onClick={() => {
                setExpanded("");
              }}
            >
              <Iconify icon="mdi:check" />
            </IconButton>
            <IconButton
              onClick={() => {
                const { input, ...newPatch } = patch;
                setPatch(newPatch);
                setExpanded("");
              }}
            >
              <Iconify icon="mdi:close" />
            </IconButton>
          </Stack>
        </CardContent>
      )}

      {editing && Object.keys(patch).length > 1 && (
        <CardContent>
          <Stack direction={"column"} spacing={2}>
            <Typography variant="h6">Changes</Typography>
            <TextareaAutosize
              value={JSON.stringify(patch, null, 2)}
              readOnly
              style={{
                width: "100%",
                color: "#eee",
                background: "rgba(0, 0, 0, 0.2)",
                border: "none",
                padding: "1rem",
              }}
            />
          </Stack>
        </CardContent>
      )}
    </Card>
  );
};

export default StreamCard;
