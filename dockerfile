FROM rust:1.51

WORKDIR /usr/src/wait_for_rust
COPY . .

RUN cargo install --path .