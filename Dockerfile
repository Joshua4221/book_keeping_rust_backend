
#
# Stage 1 (Build)
#

FROM rust:1.68-slim-buster As build

WORKDIR /book_keeping

COPY . .

RUN app-get update && app-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:bullseye-slim

WORKDIR /book_keeping

RUN app-get update && app-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /book_keeping/target/release/book_keeping ./book_keeping

EXPOSE 80

# And away we go...
CMD [ "./book_keeping" ]