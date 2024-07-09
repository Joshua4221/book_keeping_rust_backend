FROM rust:1.79-slim-buster As build

WORKDIR /book_keeping

COPY . .

RUN app-get update && app-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /book_keeping

RUN app-get update && app-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /book_keeping/target/release/book_keeping ./book_keeping

EXPOSE 80

CMD [ "./book_keeping" ]
