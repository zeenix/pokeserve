FROM rust:latest

WORKDIR /usr/src/pokeserve

COPY . .

EXPOSE 3000

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/pokeserve"]
