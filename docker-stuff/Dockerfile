# Rust as the base image
# FROM rust:1.60.0 AS builder_cache

# WORKDIR /app
# COPY . /app/

# RUN cargo build

# RUN cargo run

FROM avfelipe/app:latest

RUN ls -a

WORKDIR /app

RUN ls -a

RUN head ./src/main.rs

COPY . /app/

RUN head ./src/main.rs

RUN ls -a

RUN cargo build

RUN cargo run