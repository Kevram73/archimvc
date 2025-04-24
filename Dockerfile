FROM rust:1.70 as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libsqlite3-dev
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libsqlite3-0
COPY --from=builder /usr/src/app/target/release/archimvc /usr/local/bin/

EXPOSE 8080
CMD ["archimvc"]
