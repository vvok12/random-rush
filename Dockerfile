FROM rust:1.74 as builder
WORKDIR /usr/src/random-rush
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/random-rush-server /usr/local/bin/random-rush
EXPOSE 3000/tcp
CMD ["/usr/local/bin/random-rush"]
