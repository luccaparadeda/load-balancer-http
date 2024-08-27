FROM rust:latest

WORKDIR /usr/src/service2
COPY . .

RUN cargo build --release

CMD ["./target/release/rust-web-server-2"]
