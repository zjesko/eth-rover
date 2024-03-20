FROM rust:1.76-slim-buster as build

RUN USER=root cargo new --bin ethrover
WORKDIR /ethrover

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/ethrover*
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /ethrover
RUN mkdir /ethrover/wallets
COPY --from=build /ethrover/target/release/ethrover .

CMD ["./ethrover"]