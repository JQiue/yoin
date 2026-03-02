import { create } from "zustand";
import type { Option } from "../index.d";
import type { ConfigState } from "./type";

export const useConfigStore = create<ConfigState>((set) => ({
	config: {} as Option,
	setConfig: (options) => set({ config: options }),
}));
