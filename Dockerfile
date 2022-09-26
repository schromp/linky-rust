FROM node:latest as client-builder
WORKDIR /app
COPY ./client/package.json ./
COPY ./client/package-lock.json ./
RUN npm install
COPY ./client ./
RUN npm run build
# CMD ["npm", "run" "build"]

FROM rust:latest as server-builder
WORKDIR /usr/src/linky-rust
COPY ./server .
COPY --from=client-builder ./app/build/ /web/
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=server-builder /usr/local/cargo/bin/linky-rust /usr/local/bin/linky-rust
CMD ["linky-rust"]