FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
ENV TZ=Etc/UTC DATABASE_URL=sqlite:/data/database.sqlite?mode=rwc RUST_LOG= DISCORD_TOKEN= OAUTH_URL_BASE= MASTODON_CLIENT_ID= MASTODON_CLIENT_SECRET=
VOLUME /data
WORKDIR /app
COPY --from=builder /build/target/release/oneword oneword
CMD ["./oneword"]