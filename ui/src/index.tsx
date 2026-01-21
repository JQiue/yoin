import { render } from "preact";
import App from "./App";
import type { Option } from "./index.d";

export default class Yoin {
  private container: HTMLElement | null = null;

  constructor(options: Option) {
    this.container = document.getElementById(options.containerId);
    if (this.container) {
      render(<App config={options} />, this.container);
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
