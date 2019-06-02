#compile the app
FROM rust:1.35
WORKDIR /home/server
COPY . /home/server
RUN cargo build --release

#copy binary
FROM alpine:latest  
WORKDIR /home/server
COPY --from=0 /target/release/my_rust_server /home/server
EXPOSE 80
CMD ["./server/my_rust_server"] 
