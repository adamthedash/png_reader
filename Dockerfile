FROM rust:1-slim
LABEL authors="Adam"

RUN apt-get update && apt-get install -y linux-perf
RUN cargo install flamegraph

RUN rustup toolchain install nightly && rustup default nightly

WORKDIR /app

