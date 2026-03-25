export interface RuntimeOptions {
  containerId: string;
  api_base?: string;
}

export interface AppConfig extends RuntimeOptions {
  site_id?: number;
}
