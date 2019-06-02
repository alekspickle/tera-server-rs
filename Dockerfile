#compile the app
FROM rust:1.35
WORKDIR /server
COPY . /server
RUN cargo build --release

#copy binary
FROM alpine:latest  
WORKDIR /root/
COPY --from=0 /home/aleksrow/Documents/Rust/myserver-rs/target/release/my_rust_server .
EXPOSE 80
CMD ["./my_rust_server"] 
