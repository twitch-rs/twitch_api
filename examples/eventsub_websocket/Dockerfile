# syntax = docker/dockerfile:1.2
# expects the build context to be the twitch_api workspace root, i.e
# docker build -f examples/eventsub/Dockerfile .
# No dockerignore
FROM rust:1-alpine3.16 as builder
WORKDIR /app
ARG BUILD_DEPS="musl-dev pkgconfig perl build-base openssl openssl-dev git"
RUN apk add --no-cache ${BUILD_DEPS}
COPY ../../ .
ARG RUSTFLAGS=-Ctarget-feature=-crt-static
ARG CARGO_UNSTABLE_SPARSE_REGISTRY=true
RUN --mount=type=cache,target=$CARGO_HOME/git \
    --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,sharing=private,target=/app/target \
    cargo -V; cargo build --release -p eventsub && mv /app/target/release/eventsub /app/is_live
FROM alpine:3.17 as runtime
WORKDIR /app
ARG RUN_DEPS="ca-certificates openssl libgcc"
RUN apk add --no-cache \
        ${RUN_DEPS}
COPY --from=builder /app/is_live /app/is_live
COPY ./examples/eventsub/static ./static
ENTRYPOINT "/app/is_live"