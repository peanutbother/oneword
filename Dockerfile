FROM rust:latest AS planner
WORKDIR /build
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest AS cacher
WORKDIR /build
RUN cargo install cargo-chef
COPY --from=planner /build/recipe.json recipe.json
COPY . .
RUN cargo chef cook --release --recipe-path recipe.json


FROM rust:latest AS builder
WORKDIR /build
COPY --from=cacher /build/target target
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
ENV TZ=Etc/UTC DATABASE_URL=sqlite:/data/database.sqlite?mode=rwc RUST_LOG= DISCORD_TOKEN= OAUTH_URL_BASE= MASTODON_CLIENT_ID= MASTODON_CLIENT_SECRET=
VOLUME /data
WORKDIR /app
COPY --from=builder /build/target/release/oneword oneword
CMD ["./oneword"]