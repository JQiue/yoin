FROM node:24.14.0-slim AS ui-builder
WORKDIR /app/ui
COPY ui/package.json ui/pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile
COPY ui ./
RUN pnpm run build

FROM rust:1.94.0-slim AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY migration ./migration
COPY src ./src
COPY build.rs ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --features postgres --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY migration ./migration
COPY src ./src
COPY build.rs ./
ENV YOIN_SKIP_UI_BUILD=1
COPY --from=ui-builder /app/ui/dist ./ui/dist
RUN cargo build --release --target x86_64-unknown-linux-musl --features postgres

FROM scratch
WORKDIR /app
ENV HOST=0.0.0.0
COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/yoin /app/yoin
EXPOSE 7410
ENTRYPOINT ["./yoin"]
