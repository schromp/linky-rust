FROM node:latest as client-builder
WORKDIR /app
COPY ./client/package.json ./
COPY ./client/package-lock.json ./
RUN npm install
COPY ./client ./
CMD ["npm", "build"]

FROM rust:latest as server-builder
WORKDIR /app
COPY ./server .
COPY --from=client-builder ./app/build/ /web/
RUN cargo build --release

FROM debian:buster-slim
COPY --from=server-builder /app/target/release /usr/local/bin/linky-rust
CMD ["chmod", "+x", "/usr/local/bin/linky-rust"]