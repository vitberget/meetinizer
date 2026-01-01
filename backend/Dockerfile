FROM rust:1.92-alpine3.23 as build

WORKDIR /app/
ADD sql_migrations /app/sql_migrations/
ADD Cargo.toml Cargo.lock /app/
ADD src /app/src/
RUN cargo build --release



FROM alpine:3.23

WORKDIR /app/
COPY --from=build /app/target/release/meetinizer /app/

ENTRYPOINT /app/meetinizer serve
