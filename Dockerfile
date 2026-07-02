FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
COPY Cargo.toml Cargo.lock ./
COPY entity/Cargo.toml ./entity/Cargo.toml
COPY migration/Cargo.toml ./migration/Cargo.toml
RUN mkdir -p src entity/src migration/src && \
    echo "fn main() {}" > src/main.rs && \
    touch entity/src/lib.rs && \
    touch migration/src/lib.rs
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN touch entity/src/lib.rs migration/src/lib.rs src/main.rs
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
ENV TZ=Etc/UTC DATABASE_URL=sqlite:/data/database.sqlite?mode=rwc RUST_LOG= DISCORD_TOKEN= OAUTH_URL_BASE= MASTODON_CLIENT_ID= MASTODON_CLIENT_SECRET=
VOLUME /data
WORKDIR /app
COPY --from=builder /build/target/release/oneword oneword
CMD ["./oneword"]