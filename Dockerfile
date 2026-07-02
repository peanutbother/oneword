FROM docker.io/rust:1-alpine AS chef
WORKDIR /build

FROM chef AS planner
WORKDIR /build
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /build
COPY --from=planner /build/recipe.json recipe.json
COPY . .
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
ENV TZ=Etc/UTC DATABASE_URL=sqlite:/data/database.sqlite?mode=rwc RUST_LOG= DISCORD_TOKEN= OAUTH_URL_BASE= MASTODON_CLIENT_ID= MASTODON_CLIENT_SECRET=
VOLUME /data
WORKDIR /app
COPY --from=builder /build/target/release/oneword oneword
CMD ["./oneword"]