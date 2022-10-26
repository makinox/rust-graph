FROM rust:1.61.0

WORKDIR /Users/jesusbossa/Documents/Development/rust-graph
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["rust-graph"]