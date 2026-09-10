# 本地开发

## 仓库结构

```
.
├─ src/            Rust 服务端（Axum + SeaORM）
├─ migration/      数据库迁移（独立 crate）
├─ tests/          集成测试
├─ ui/             前端（独立 npm 包：Preact + Rsbuild + Tailwind v4）
├─ docs/           本文档（mdBook）
└─ build.rs        构建时把前端产物打进二进制
```

## 前置条件

- Rust（edition 2024；`rustfmt` 的配置用 nightly，运行本身用 stable 即可）
- Node.js + npm：`build.rs` 会构建前端，缺了会编译失败

## 常用命令

| 命令 | 作用 |
| --- | --- |
| `cargo run` | 起后端；**debug 构建下会顺带起前端 playground**（widget `:3000/client`、后台 `:3001/admin`），Ctrl-C 一起收掉 |
| `YOIN_UI_DEV=0 cargo run` | 只起后端；`=client` / `=admin` 只起其中一个 |
| `cargo test --all-features` | 跑测试（集成测试用内存 SQLite） |
| `cargo +nightly fmt --all` | 格式化 |
| `cargo clippy --all-targets --all-features -- -D warnings` | lint（CI 用的就是这条） |
| `mdbook serve docs --port 3002` | 本地预览本文档（3000/3001 留给 playground） |

前端相关的命令都在 `ui/` 下执行：

| 命令 | 作用 |
| --- | --- |
| `npm run client:dev` / `npm run admin:dev` | 分别起 widget / 后台的 playground（会 proxy `/api` 到 `127.0.0.1:7410`） |
| `npm run build` | 产出 `ui/dist/client/client.js`、`ui/dist/admin/admin.js` |
| `npm run biome` | `biome check --write` |
| `npx tsc --noEmit` | 类型检查 |

> 本地构建用 **npm**（`build.rs` 也是），Docker 与 `ui/pnpm-lock.yaml` 用 **pnpm**。

## 前端产物是怎么进二进制的

`build.rs` 在编译前跑 `npm run build`，然后 `src/handler/js.rs` 用 `include_str!` 把产物编进二进制，由 `/static/client.js`、`/static/admin.js` 两个路由对外提供。

由此带来三条实践约定：

- **改完前端要重新 `cargo build`/`cargo run`**，二进制里的是一份拷贝，不重新编译不会更新；
- `YOIN_SKIP_UI_BUILD=1` 可跳过构建，但前提是 `ui/dist/client/client.js` 与 `ui/dist/admin/admin.js` 已存在（否则编译失败）；
- playground（`:3000`/`:3001`）走的是 Rsbuild dev server，实时生效，用来调 UI 比重新编译二进制快得多。

## 测试与 CI

集成测试在 [tests/integration_test.rs](https://github.com/JQiue/yoin/blob/dev/tests/integration_test.rs)，公共脚手架在 `tests/common/mod.rs`：数据库用 `sqlite::memory:`，每个测试用 `test_app()` 起一个独立实例，**不要共享数据库状态**。

CI（[.github/workflows/check.yml](https://github.com/JQiue/yoin/blob/dev/.github/workflows/check.yml)）在 stable 上跑 clippy 与测试、在 nightly 上检查格式。
