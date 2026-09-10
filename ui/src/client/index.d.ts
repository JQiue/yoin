interface RuntimeOptions {
  containerId: string;
  api_base?: string;
}

export type Option = RuntimeOptions & {
  site_id: number;
};
