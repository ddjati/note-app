# 1. This tells docker to use the Rust official image
FROM rust:1.83 AS builder

# create a new empty shell project
WORKDIR /note-app
COPY . .
RUN cargo build --release

# our final base
# FROM debian:buster-slim
# FROM debian:bullseye
# FROM debian:bookworm-slim
FROM debian:stable-slim

# copy the build artifact from the build stage
COPY --from=builder /note-app/target/release/note-app ./note-app
COPY --from=builder /note-app/.env ./.env

USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

# set the startup command to run your binary
CMD ["./note-app"]
