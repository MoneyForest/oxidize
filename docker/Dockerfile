FROM rust:1.85-bookworm AS builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY schema/proto ./schema/proto
COPY db/migrations ./db/migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/oxidize /app/oxidize
COPY --from=builder /app/db/migrations /app/db/migrations

ENV RUST_LOG=info

EXPOSE 8080 50051

ENTRYPOINT ["/app/oxidize"]
