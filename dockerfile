FROM rust:1.31

WORKDIR /myserver
COPY . .

RUN cargo install --path .

CMD ["myapp"]
