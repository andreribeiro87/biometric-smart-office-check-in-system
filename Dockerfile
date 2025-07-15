# Dev stage
FROM rust:latest

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
# COPY schema.rs ./src/schema.rs

EXPOSE 3000

ENTRYPOINT ["cargo", "watch", "-x", "run"]