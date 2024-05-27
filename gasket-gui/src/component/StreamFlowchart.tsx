import { useMemo } from "react";
import ReactFlow, {
  Background,
  ReactFlowProvider,
  useReactFlow,
} from "reactflow";
import "reactflow/dist/style.css";
import useLb from "../hooks/useLb";

import InputNode from "./nodes/InputNode";
import OutputNode from "./nodes/OutputNode";
import WorkerNode from "./nodes/WorkerNode";
import { Stream, Uuid, Worker } from "../types";
import type Node from "reactflow";
import Edge from "reactflow";

const nodeTypes = {
  inputNode: InputNode,
  outputNode: OutputNode,
  workerNode: WorkerNode,
};

const StreamFlowchart = ({
  stream,
  editing = false,
  callback,
}: {
  stream: Stream;
  editing?: boolean;
  callback: (type: string, action: string, id: Uuid) => void;
}) => {
  const { fitView } = useReactFlow();
  const { workers } = useLb();

  const [nodes, edges]: [Node[], Edge[]] = useMemo(() => {
    if (!(stream?.input && stream?.output)) {
      return;
    }

    let n: Node[] = [];
    let e: Edge[] = [];

    let unique_workers = new Set();
    let num_outputs = 0;

    stream.output.forEach((output) => {
      if (output.worker) {
        unique_workers.add(output.worker);
      }
      num_outputs++;
    });

    let num_workers = unique_workers.size;

    let node_height = 100;
    let node_width = 300;
    let node_height_half = node_height / 2;

    let longest_list = Math.max(num_workers, num_outputs);

    let stream_pos = {
      x: 0,
      y: (node_height * longest_list) / 2 - node_height_half,
    };

    n.push({
      id: stream.id,
      position: stream_pos,
      data: { resource: stream, editing: editing, callback: callback },
      type: "inputNode",
    });

    stream.output.forEach((output, index) => {
      if (!output.worker) {
        n.push({
          id: output.id,
          position: { x: 2 * node_width, y: index * node_height },
          data: { resource: output, editing: editing, callback: callback },
          type: "outputNode",
        });
      }

      let w = workers.find((worker: Worker) => worker.id === output.worker);
      if (!w) {
        return;
      }

      // ensure the worker is already a node
      if (!n.find((node) => node.id === output.worker)) {
        n.push({
          id: output.worker,
          position: {
            x: node_width,
            y:
              index * node_height +
              (node_height * longest_list) / 2 -
              node_height_half,
          },
          data: { resource: w, editing: editing, callback: callback },
          type: "workerNode",
        });
      }

      n.push({
        id: output.id,
        position: { x: 2 * node_width, y: index * node_height },
        data: { resource: output, editing: editing, callback: callback },
        type: "outputNode",
      });

      // check if the edge stream.id -> output.worker exists
      if (
        !e.find(
          (edge) => edge.source === stream.id && edge.target === output.worker,
        )
      ) {
        e.push({
          id: `e${stream.id}-${output.worker}`,
          source: stream.id,
          target: output.worker,
          animated: true,
        });
      }

      e.push({
        id: `e${output.worker}-${output.id}`,
        source: output.worker,
        target: output.id,
        animated: true,
      });
    });
    window.requestAnimationFrame(() => {
      fitView();
    });
    return [n, e];
  }, [stream])!;

  return (
    <div style={{ width: "100%", height: "300px" }}>
      <ReactFlow
        proOptions={{ hideAttribution: true }}
        nodes={nodes}
        edges={edges}
        fitView
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        zoomOnDoubleClick={false}
        edgesUpdatable={false}
        nodesDraggable={false}
        nodesConnectable={false}
        elementsSelectable={true}
        draggable={false}
        nodeTypes={nodeTypes}
      >
        <Background variant="dots" gap={12} size={1} />
      </ReactFlow>
    </div>
  );
};

const FlowContainer = ({
  stream,
  editing = false,
  callback,
}: {
  stream: Stream;
  editing?: boolean;
  callback: (type: string, action: string, id: Uuid) => void;
}) => {
  return (
    <div style={{ width: "100%", height: "100%" }}>
      <ReactFlowProvider>
        <StreamFlowchart
          stream={stream}
          editing={editing}
          callback={callback}
        />
      </ReactFlowProvider>
    </div>
  );
};

export default FlowContainer;
