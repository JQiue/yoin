# AGENTS.md

You are an expert in JavaScript, Rsbuild, and web application development. You write maintainable, performant, and accessible code.

## Commands

Local UI builds in `build.rs` use **npm**. Docker / this workspace lockfile use **pnpm**. Do not invent a `pnpm dev` script.

- `npm run client:dev` — widget playground (`http://localhost:3000/client`, proxies `/api` to `127.0.0.1:7410`)
- `npm run admin:dev` — admin playground
- `npm run build` — production bundles for `client` and `admin`
- `npm run biome` — `biome check --write`
- `npx tsc --noEmit` — typecheck

## Docs

- Rsbuild: https://rsbuild.rs/llms.txt
- Rspack: https://rspack.rs/llms.txt
