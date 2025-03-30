FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin miniflux-feed-combiner

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/miniflux-feed-combiner miniflux-feed-combiner
ENTRYPOINT ["./miniflux-feed-combiner"]

