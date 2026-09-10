# Yoin UI

Preact widgets compiled into the Rust binary: the public comment widget (`client`) and the admin console (`admin`).

## Setup

```bash
pnpm install
```

Docker and `pnpm-lock.yaml` use **pnpm**; [build.rs](../build.rs) shells out to **npm** (`npm run build`). Either package manager works locally.

## Development

- `npm run client:dev` — widget playground at http://localhost:3000/client, proxies `/api` to `127.0.0.1:7410`
- `npm run admin:dev` — admin playground

## Build and check

- `npm run build` — bundles both entries into `dist/client` and `dist/admin`
- `npm run client:build` / `npm run admin:build` — one entry only
- `npm run client:preview` / `npm run admin:preview` — serve a production bundle
- `npx tsc --noEmit` — typecheck
- `npm run biome` — `biome check --write`
