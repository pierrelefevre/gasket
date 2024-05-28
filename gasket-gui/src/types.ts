export type Uuid = string;
export type Uri = string;

export enum Codec {
  H264 = "H264",
  H265 = "H265",
  AV1 = "AV1",
}

export type StreamOptions = {
  pixel_format?: string;
  bitrate?: string;
  framerate?: string;
  gop_size?: string;
  debug_text?: boolean;
  output_format?: string;
};

export type Output = {
  id: Uuid;
  uri: Uri;
  codec: Codec;
  options?: StreamOptions;
  status: string;
  worker?: Uuid;
  logs?: string[];
  last_error?: string;
};

export type Stream = {
  id: Uuid;
  input: Uri;
  name: string;
  output: Output[];
  enabled: boolean;
  status: string;
};

export type Worker = {
  codecs: Codec[];
  encoder: string;
  protocol: string;
  host: Uri;
  public_ip?: string;
  id: Uuid;
  stats: {
    devices: number[];
    utilization: number;
  };
  status: string;
  udp_ports: number[];
  streams?: number;
  server?: string;
};
