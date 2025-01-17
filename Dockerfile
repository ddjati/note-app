# Step 1: Compute a recipe file
FROM rust:1.83 AS planner
WORKDIR /note-app
RUN cargo install cargo-chef
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Cache project dependencies
FROM rust:1.83 AS cacher
WORKDIR /note-app
RUN cargo install cargo-chef
COPY --from=planner /note-app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 3: Build the binary
FROM rust:1.83 AS builder
WORKDIR /note-app
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# Copy over the cached dependencies from above
COPY --from=cacher /note-app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin note-app

# our final base
FROM debian:stable-slim

# Install cURL
RUN apt-get update && apt-get install -y curl && apt-get clean

# copy the build artifact from the build stage
COPY --from=builder /note-app/target/release/note-app ./note-app
# COPY --from=builder /note-app/.env ./.env

USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

# set the startup command to run your binary
CMD ["./note-app"]
