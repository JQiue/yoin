# Yoin Agent Guidelines

Embeddable comment system: Axum + SeaORM backend, Preact widget + admin UI compiled into the binary.

## Architecture

Layering (keep it):

- [src/handler/](src/handler/) — HTTP payloads, extractors, `ApiResponse` / `AppError`. Thin.
- [src/service/](src/service/) — business rules, RBAC checks, JWT, moderation orchestration. Methods live on `AppService`.
- [src/repository/](src/repository/) — SeaORM only. Return `Result<_, DbErr>`. Map with `.with_op("...")`.
- [src/entity/](src/entity/) — generated SeaORM entities (`sea-orm-codegen 2.0`). Prefer regenerating over hand-editing schema.
- [src/extractor.rs](src/extractor.rs) — `AppJson`, `RequireAuth`, `OptionnalAuth` (spelling is intentional), `RemoteIp`.
- [src/moderation/](src/moderation/) — pluggable comment moderators.
- [src/rbac/](src/rbac/) — bootstrap + permission/role code re-exports.

Request flow: `handler` → `state.service.*` → `repo.*` → entity. Do not query SeaORM from handlers.

Routes are nested under `/api` in [src/app.rs](src/app.rs). Static widgets: `/static/client.js`, `/static/admin.js`.

## API conventions

- Success: `ApiResponse::success(data)` → HTTP 200, `{ "code": "ok", "msg": "success", "data": ... }` ([src/response.rs](src/response.rs)).
- Client errors: `AppError::Client` helpers (`bad_request`, `forbidden`, `user_not_found`, …). `code` is a snake_case `ErrorCode` string such as `invalid_credentials` ([src/constants/error_codes.rs](src/constants/error_codes.rs)).
- Infra/DB errors: `.with_op("short operation")` → `AppError::Internal`. Never leak internals to clients.
- JSON bodies: `AppJson<T>`, not raw `Json<T>`.
- Auth: Bearer JWT. Public routes use `OptionnalAuth`; private/admin use `RequireAuth`.
- Permission strings live in [src/constants.rs](src/constants.rs) (`site.manage`, `comment.moderate`, …). First registered user gets global `super_admin` and a default site.

Do not “fix” `OptionnalAuth` unless the task is a rename.

## Build and test

Default DB feature is `sqlite`. Env via `envy` + optional `.env` ([src/config.rs](src/config.rs)): `DATABASE_URL`, `HOST`, `PORT`, `JWT_KEY` (≥32 chars). Unset `JWT_KEY` uses an ephemeral key (tokens die on restart). Default listen: `127.0.0.1:7410`.

```bash
cargo run
cargo +nightly fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Integration tests: [tests/integration_test.rs](tests/integration_test.rs) + [tests/common/mod.rs](tests/common/mod.rs) (`sqlite::memory:`, `TEST_JWT_KEY`). Run with `cargo test --test integration_test`. Each test should use `test_app()`; do not share DB state.

`cargo build` / `cargo test` compile-time `include_str!` the UI bundles ([src/handler/js.rs](src/handler/js.rs)). [build.rs](build.rs) runs `npm run build` in `ui/` unless `YOIN_SKIP_UI_BUILD=1`. Skip only if `ui/dist/client/client.js` and `ui/dist/admin/admin.js` already exist.

CI: [.github/workflows/check.yml](.github/workflows/check.yml) (`fmt` nightly, clippy + tests on stable). Rustfmt: [rustfmt.toml](rustfmt.toml) (`tab_spaces = 2`, edition 2024).

Migrations: [migration/README.md](migration/README.md). Enums used by entities live in `migration::enums`.

## Pitfalls

- Local UI build in `build.rs` is **npm**, not pnpm. Docker/UI workspace uses **pnpm**.
- `TESTING.md` layout/commands are stale (`cargo test --test integration`, empty `tests/unit/`, tarpaulin). Prefer CI commands above.
- [README.md](README.md) is a stub. Product status: [CHANGELOG.md](CHANGELOG.md) (`0.0.1` draft; OAuth/moderation/admin tabs not fully E2E).
- RBAC bootstrap: [src/docs/bootstrap_rbac.md](src/docs/bootstrap_rbac.md).
- Rate limiter and site-config cache are in-process mutexes ([src/app.rs](src/app.rs), [src/helper.rs](src/helper.rs)); do not treat them as distributed.
- `main` leaks a `'static` `DatabaseConnection`; repositories take `&'static DatabaseConnection`.
- CORS is permissive. `RemoteIp` trusts `X-Forwarded-For` only from loopback/private peers.

## Frontend

See [ui/AGENTS.md](ui/AGENTS.md). Do not use React APIs or invent a `pnpm dev` script.
