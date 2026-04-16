FROM rust:1.94.1-alpine3.23 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:3.23 AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/magical_square_api .

EXPOSE 8000

CMD ["./magical_square_api"]
