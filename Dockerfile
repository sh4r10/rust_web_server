FROM rust:1.75

WORKDIR /opt/sources
COPY . . 
RUN cargo build --release 

EXPOSE 4000

ENTRYPOINT ["/opt/sources/target/release/web_server"]
