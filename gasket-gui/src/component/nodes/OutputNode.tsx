import { memo, useState } from "react";
import { Handle } from "reactflow";
import Iconify from "../Iconify";
import { IconButton, Tooltip } from "@mui/material";
import { OutputNodeProps } from "./types";
import { chopString } from "../../utils";

const renderNodeStyle = (status: string) => {
  let color = "orange";
  switch (status.toLowerCase()) {
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

export default memo(({ data }: OutputNodeProps) => {
  const [errorTooltipOpen, setErrorTooltipOpen] = useState(false);

  return (
    <>
      <Handle type="target" position={"left"} style={{ background: "#555" }} />
      <div
        style={{
          padding: "10px",
          backgroundColor: renderNodeStyle(data.resource.status),
          color: "black",
          borderRadius: ".5rem",
          display: "flex",
          flexDirection: "column",
          minWidth: "200px",
        }}
      >
        <strong>{chopString(data.resource.uri, 25)}</strong>
        <span
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginTop: ".5rem",
            gap: ".5rem",
          }}
        >
          {data.resource.codec}
          <div style={{ flexGrow: 1 }} />
          {data.editing && (
            <>
              <IconButton
                size={"small"}
                sx={{ color: "black" }}
                onClick={() =>
                  data.callback("output", "delete", data.resource.id)
                }
              >
                <Iconify icon={"mdi:trash"} />
              </IconButton>
              <IconButton
                size={"small"}
                sx={{ color: "black" }}
                onClick={() =>
                  data.callback("output", "edit", data.resource.id)
                }
              >
                <Iconify icon={"mdi:pencil"} />
              </IconButton>
            </>
          )}

          {data.resource.last_error && (
            <Tooltip title={data.resource.last_error} open={errorTooltipOpen}>
              <IconButton
                size={"small"}
                sx={{ color: "black" }}
                onClick={() => setErrorTooltipOpen(!errorTooltipOpen)}
              >
                <Iconify
                  icon={
                    "material-symbols:warning" +
                    (errorTooltipOpen ? "-off" : "")
                  }
                />
              </IconButton>
            </Tooltip>
          )}
        </span>
      </div>
    </>
  );
});
