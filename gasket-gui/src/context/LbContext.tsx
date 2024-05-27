import { useState, createContext } from "react";
import useInterval from "../hooks/useInterval";
import {
  createStream,
  createWorker,
  deleteStream,
  deleteWorker,
  getStreams,
  getWorkers,
  patchStream,
  patchWorker,
} from "../api/gasket-lb";
import { useCookies } from "react-cookie";
import { enqueueSnackbar } from "notistack";
import { Stream, Uuid } from "../types";
const initialState: any = {
  streams: [],
  workers: [],
};

export const LbContext = createContext({
  ...initialState,
});
export const LbContextProvider = ({ children }: { children: any }) => {
  const [streams, setStreams] = useState([]);
  const [workers, setWorkers] = useState([]);
  const [cookies] = useCookies(["gasket_api_url"]);

  const addWorker = async (worker: Worker) => {
    try {
      let response = await createWorker(cookies.gasket_api_url, worker);
      enqueueSnackbar("Worker created successfully", { variant: "success" });
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  const removeWorker = async (worker_id: Uuid) => {
    try {
      console.log(worker_id);
      let response = await deleteWorker(cookies.gasket_api_url, worker_id);
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      enqueueSnackbar("Worker deleted successfully", { variant: "success" });
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  const updateWorker = async (worker_id: Uuid, patch: any) => {
    try {
      let response = await patchWorker(
        cookies.gasket_api_url,
        worker_id,
        patch,
      );
      enqueueSnackbar("Worker updated successfully", { variant: "success" });
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  const addStream = async (stream: Stream) => {
    try {
      let response = await createStream(cookies.gasket_api_url, stream);
      enqueueSnackbar("Stream created successfully", { variant: "success" });
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  const removeStream = async (stream_id: Uuid) => {
    try {
      let response = await deleteStream(cookies.gasket_api_url, stream_id);
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      enqueueSnackbar("Stream deleted successfully", { variant: "success" });
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  const updateStream = async (stream_id: Uuid, patch: any) => {
    try {
      let response = await patchStream(
        cookies.gasket_api_url,
        stream_id,
        patch,
      );
      if (response.error) {
        console.log(response.error);
        enqueueSnackbar(response.error, { variant: "error" });
        return;
      }
      enqueueSnackbar("Stream updated successfully", { variant: "success" });
      return response;
    } catch (error) {
      enqueueSnackbar(JSON.stringify(error), { variant: "error" });
    }
  };

  useInterval(() => {
    getStreams(cookies.gasket_api_url).then((data) => {
      setStreams(data);
    });
    getWorkers(cookies.gasket_api_url).then((data) => {
      setWorkers(data);
    });
  }, 1000);

  return (
    <LbContext.Provider
      value={{
        streams,
        setStreams,
        workers,
        setWorkers,
        addWorker,
        removeWorker,
        updateWorker,
        addStream,
        removeStream,
        updateStream,
      }}
    >
      {children}
    </LbContext.Provider>
  );
};

export default LbContext;
