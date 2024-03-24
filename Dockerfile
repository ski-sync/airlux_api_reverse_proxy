FROM rust:1.75.0 as base
ARG DATABASE_URL=app.db
WORKDIR /app
ADD . /app
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
RUN cargo build --release

FROM debian:12.0-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libsqlite3-0
COPY --from=base /app/target/release/api /app
COPY --from=base /app/$DATABASE_URL /app
EXPOSE 8081
ENTRYPOINT ["./api"]
