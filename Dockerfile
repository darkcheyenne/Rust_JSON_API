FROM rust

ADD target/release/rust_api2 /
EXPOSE 8000

CMD ["/rust_api2"]
