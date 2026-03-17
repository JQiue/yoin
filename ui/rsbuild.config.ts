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
    },
  },
  performance: {
    chunkSplit: {
      strategy: "all-in-one",
    },
    // bundleAnalyze: {
    //   generateStatsFile: true,
    // },
  },
  html: {
    template: "./src/template/index.html",
  },
  environments: {
    client: {
      source: {
        entry: {
          client: "./src/client/index.tsx",
        },
      },
      output: {
        injectStyles: true,
        filenameHash: false,
        target: "web",
        distPath: {
          js: "client",
          css: "",
        },
        copy: [
          {
            from: "./src/client/index.d.ts",
            to: "./client/index.d.ts",
          },
        ],
      },
      tools: {
        rspack: {
          output: {
            library: {
              name: "YoinClient",
              type: "window",
              export: "default",
            },
          },
        },
      },
    },
    admin: {
      source: {
        entry: {
          admin: "./src/admin/index.tsx",
        },
      },
      output: {
        injectStyles: true,
        filenameHash: false,
        target: "web",
        distPath: {
          js: "admin",
          css: "",
        },
        copy: [
          {
            from: "./src/admin/index.d.ts",
            to: "./admin/index.d.ts",
          },
        ],
      },
      tools: {
        rspack: {
          output: {
            library: {
              name: "YoinAdmin",
              type: "window",
              export: "default",
            },
          },
        },
      },
    },
  },
});
