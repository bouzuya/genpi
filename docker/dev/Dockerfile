FROM rust:1.71.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y \
  jq \
  mold \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
RUN rustup component add \
  clippy \
  rustfmt
RUN cargo install \
  cargo-watch
