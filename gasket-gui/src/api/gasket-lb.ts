import { Uri, Uuid } from "../types";

export const getWorkers = async (apiUrl: Uri): Promise<any> => {
  const response = await fetch(`${apiUrl}/worker`);
  return response.json();
};

export const createWorker = async (
  apiUrl: Uri,
  worker: Worker,
): Promise<any> => {
  const response = await fetch(`${apiUrl}/worker`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(worker),
  });
  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return await response.json();
};

export const deleteWorker = async (
  apiUrl: Uri,
  worker_id: Uuid,
): Promise<any> => {
  const response = await fetch(`${apiUrl}/worker/${worker_id}`, {
    method: "DELETE",
  });

  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return response.text();
};

export const patchWorker = async (
  apiUrl: Uri,
  worker_id: Uuid,
  patch: any,
): Promise<any> => {
  const response = await fetch(`${apiUrl}/worker/${worker_id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(patch),
  });

  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return await response.json();
};

export const getStreams = async (apiUrl: Uri): Promise<any> => {
  const response = await fetch(`${apiUrl}/stream`);
  return await response.json();
};

export const createStream = async (apiUrl: Uri, stream: any): Promise<any> => {
  const response = await fetch(`${apiUrl}/stream`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(stream),
  });
  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return await response.json();
};

export const deleteStream = async (
  apiUrl: Uri,
  stream_id: Uuid,
): Promise<any> => {
  const response = await fetch(`${apiUrl}/stream/${stream_id}`, {
    method: "DELETE",
  });

  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return await response.text();
};

export const patchStream = async (
  apiUrl: Uri,
  stream_id: Uuid,
  patch: any,
): Promise<any> => {
  const response = await fetch(`${apiUrl}/stream/${stream_id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(patch),
  });

  if (!response.ok) {
    if (response.body) throw { error: response.body };
    else throw { error: response.statusText };
  }
  return await response.json();
};

export const getLb = async (apiUrl: Uri): Promise<any> => {
  const response = await fetch(`${apiUrl}/`);
  return await response.json();
};
