import { render } from "preact";
import App from "@/admin/App";
import type { Option } from "@/admin/types";
import { setRuntimeConfig } from "@/config/runtime";
import "@/styles/base.css";
import "@/styles/admin.css";

export default class YoinAdmin {
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
