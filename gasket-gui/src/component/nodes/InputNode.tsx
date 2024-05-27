import { IconButton } from "@mui/material";
import { memo } from "react";
import { Handle } from "reactflow";
import Iconify from "../Iconify";
import { InputNodeProps } from "./types";
import { chopString } from "../../utils";
import { Stream } from "../../types";

const renderNodeBg = (stream: Stream) => {
  if (!stream.enabled) return "#777";

  let color = "orange";
  switch (stream.status.toLowerCase()) {
    case "running":
    case "up":
      color = "#6cf542";
      break;
    case "stopped":
    case "crashed":
      color = "red";
      break;
  }
  return color;
};

const renderNodeColor = (stream: Stream) => {
  if (!stream.enabled) return "white";
  return "black";
};

export default memo(({ data }: InputNodeProps) => {
  return (
    <>
      <div
        style={{
          padding: "10px",
          backgroundColor: renderNodeBg(data.resource),
          color: renderNodeColor(data.resource),
          borderRadius: ".5rem",
          display: "flex",
          flexDirection: "column",
          border: "3px solid #121212",
          minWidth: "200px",
        }}
      >
        <strong>{chopString(data.resource.input, 25)}</strong>

        {data.editing && (
          <span
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              marginTop: ".5rem",
              gap: ".5rem",
            }}
          >
            <div style={{ flexGrow: 1 }} />
            <>
              <IconButton
                size={"small"}
                sx={{ color: "black" }}
                onClick={() => data.callback("input", "edit", data.resource.id)}
              >
                <Iconify icon={"mdi:pencil"} />
              </IconButton>
            </>
          </span>
        )}
      </div>
      <Handle type="source" position={"right"} id="a" />
    </>
  );
});
