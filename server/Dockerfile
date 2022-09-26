FROM rust:1.40 as builder
WORKDIR /usr/src/linky-rust
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/linky-rust /usr/local/bin/linky-rust
CMD ["linky-rust"]