import { defineConfig } from "@rsbuild/core";
import { pluginPreact } from "@rsbuild/plugin-preact";
export default defineConfig({
	plugins: [pluginPreact()],
	dev: {
		assetPrefix: "/",
	},
	server: {
		proxy: {
			"/api": "http://127.0.0.1:7410",
		}
	},
	performance: {
		chunkSplit: {
			strategy: "all-in-one",
		},
		// bundleAnalyze: {
		//   generateStatsFile: true,
		// },
	},
	output: {
		injectStyles: true,
		filenameHash: false,
		target: "web",
		distPath: {
			js: "",
			css: "",
		},
		copy: [
			{
				from: "./src/index.d.ts",
				to: "./index.d.ts",
			},
		],
	},
	html: {
		template: "./src/template/index.html",
	},
	tools: {
		rspack: {
			output: {
				library: {
					name: "Yoin",
					type: "window",
					export: "default",
				},
			},
		},
	},
});
