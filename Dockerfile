FROM rust:1.75.0
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release 
CMD ["target/release/frontend"]