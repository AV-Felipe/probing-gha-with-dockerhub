# Rust as the base image
FROM rust:1.60.0 AS builder_cache

WORKDIR /app
COPY . /app/

RUN cargo build

RUN cargo run