FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y musl-tools \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/stress /app/server
EXPOSE 8080

CMD ["/app/server"]
