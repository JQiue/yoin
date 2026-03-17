import { create } from "zustand";
import type { AppConfig, ConfigState } from "./types";

export const useConfigStore = create<ConfigState>((set) => ({
	config: {} as AppConfig,
	setConfig: (options) => set({ config: options }),
}));
