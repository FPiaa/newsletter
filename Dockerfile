FROM rust:1.71.0


WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/newsletter"]
