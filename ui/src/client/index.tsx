import { render } from "preact";
import App from "@/client/App";
import type { Option } from "@/client/types";
import { setRuntimeConfig } from "@/config/runtime";
import "@/styles/base.css";
import "@/styles/client.css";

export default class YoinClient {
  private container: HTMLElement | null = null;

  constructor(options: Option) {
    this.container = document.getElementById(options.containerId);
    setRuntimeConfig(options);
    if (this.container) {
      this.container.setAttribute("data-yoin", "");
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
