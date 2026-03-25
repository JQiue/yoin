import type { AppConfig } from "@/config/types";

let runtimeConfig: AppConfig = {} as AppConfig;

export const setRuntimeConfig = (config: AppConfig) => {
  runtimeConfig = config;
};

export const getRuntimeConfig = () => runtimeConfig;
