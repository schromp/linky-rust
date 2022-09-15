FROM rust:1.40 as builder
WORKDIR /usr/src/linky-rust
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/linky-rust /usr/local/bin/linky-rust
CMD ["linky-rust"]