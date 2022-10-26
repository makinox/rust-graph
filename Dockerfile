FROM rust:1.61.0 as builder
WORKDIR /usr/src/rust-graph
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-graph /usr/local/bin/rust-graph

CMD ["rust-graph"]