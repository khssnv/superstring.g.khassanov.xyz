FROM rust:1.61

WORKDIR /usr/src/superstring
COPY . .

RUN cargo install --path .

CMD ["superstring"]
