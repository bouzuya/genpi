FROM rust:1.78-alpine as builder

WORKDIR /usr/src/app
RUN apk update && apk add --no-cache musl-dev
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src/
RUN touch src/lib.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl
COPY . .
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM scratch
ENV PORT=8080
COPY --from=builder /usr/local/cargo/bin/genpi /usr/local/bin/genpi
ENTRYPOINT ["genpi"]
