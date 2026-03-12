FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
COPY services ./services
COPY apps ./apps

RUN cargo build --release -p device-broker

FROM debian:bookworm-slim
WORKDIR /srv
COPY --from=builder /app/target/release/device-broker /usr/local/bin/device-broker

ENV BROKER_BIND=0.0.0.0:8080
EXPOSE 8080

CMD ["device-broker"]
