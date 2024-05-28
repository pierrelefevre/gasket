import { memo } from "react";
import { Handle } from "reactflow";
import { WorkerNodeProps } from "./types";
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

export default memo(({ data }: WorkerNodeProps) => {
  return (
    <>
      <Handle type="target" position={"left"} />
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
        <strong>{chopString(data.resource.host, 25)}</strong>
        <span>{data.resource.stats.utilization + "% utilization"}</span>
        <span>
          {data.resource.codecs.join(", ") +
            " via " +
            (data.resource.encoder || "CPU")}
        </span>
      </div>
      <Handle type="source" position={"right"} />
    </>
  );
});
