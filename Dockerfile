FROM rust:1.75.0 as build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release 
# CMD ["target/release/frontend"]


FROM gcr.io/distroless/cc-debian12

USER nonroot

WORKDIR /usr/local/bin

COPY --from=build /usr/src/app/target/release .

CMD ["frontend"]