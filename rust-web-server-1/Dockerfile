FROM rust:latest

WORKDIR /usr/src/service1
COPY . .

RUN cargo build --release

CMD ["./target/release/rust-web-server-1"]
