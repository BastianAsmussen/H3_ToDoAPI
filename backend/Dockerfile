FROM rust:latest as builder

WORKDIR /usr/src/backend
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as runtime

RUN apt-get update  \
    && apt-get install -y libpq-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

CMD ["backend"]
