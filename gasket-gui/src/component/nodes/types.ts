import { Output, Stream, Uuid, Worker } from "../../types";

export type InputNodeProps = {
  data: {
    resource: Stream;
    editing: boolean;
    callback: (type: string, action: string, id: Uuid) => void;
  };
};

export type OutputNodeProps = {
  data: {
    resource: Output;
    editing: boolean;
    callback: (type: string, action: string, id: Uuid) => void;
  };
};

export type WorkerNodeProps = {
  data: {
    resource: Worker;
    editing: boolean;
    callback: (type: string, action: string, id: Uuid) => void;
  };
};
