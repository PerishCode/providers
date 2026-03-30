FROM rust:1.91-alpine AS builder

WORKDIR /app

RUN apk add --no-cache curl musl-dev

COPY Cargo.toml ./Cargo.toml
COPY crates/provider-codex-auth/Cargo.toml ./crates/provider-codex-auth/Cargo.toml
COPY crates/provider-server/Cargo.toml ./crates/provider-server/Cargo.toml
COPY crates/provider-codex-auth/src ./crates/provider-codex-auth/src
COPY crates/provider-server/src ./crates/provider-server/src

RUN cargo build --release -p provider-codex-server

FROM alpine:3.22

WORKDIR /app

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/provider-codex-server /usr/local/bin/provider-codex-server

ENV PORT=8080

EXPOSE 8080

CMD ["provider-codex-server"]
