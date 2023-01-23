FROM rust:1-buster

RUN USER=root cargo new --bin mrbgpdv2
WORKDIR /mrbgpdv2

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN rustup default nightly && cargo build && rm src/*.rs

COPY ./tests ./tests
COPY ./src ./src
RUN cargo build

CMD ["./target/debug/mrbgpdv2", "64513 10.200.100.3 64512 10.200.100.2 passive 10.100.220.0/24"]