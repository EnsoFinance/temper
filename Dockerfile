FROM rust:1.69.0-slim-buster

COPY . .
EXPOSE 80
RUN cargo build --release

CMD ["./target/release/transaction-simulator"]