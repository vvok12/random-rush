FROM rust:1.74 as builder
WORKDIR /usr/src/random-rush
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/random-rush /usr/local/bin/random-rush
CMD ["random-rush"]
