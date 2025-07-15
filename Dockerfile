# Dev stage
FROM rust:1.77-slim

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libclang-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN cargo install cargo-watch

COPY src ./src
COPY migrations ./migrations
COPY schema.rs ./src/schema.rs

EXPOSE 3002

ENTRYPOINT ["cargo", "watch", "-x", "run"]