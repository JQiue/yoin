import { render } from "preact";
import App from "./App";
import type { Option } from "./types";
import { useConfigStore } from "@/store/useConfigStore";
import '@/styles/base.css';
import '@/styles/client.css';

export default class Admin {
	private container: HTMLElement | null = null;

	constructor(options: Option) {
		this.container = document.getElementById(options.containerId);
		useConfigStore.getState().setConfig(options);
		if (this.container) {
			render(<App />, this.container);
		} else {
			console.error(`Container #${options.containerId} not found.`);
		}
	}

	destroy() {
		if (this.container) {
			render(null, this.container);
			this.container = null;
		}
	}
}
