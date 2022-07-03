FROM rust:1.62-slim as builder

WORKDIR /usr/src/myapp
COPY Cargo.toml /usr/src/myapp/
RUN apt-get update && apt-get install -y libpq-dev && mkdir src &&echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY . .

RUN cargo install --path .

# - Remove these line if there backend exit with code 0 -
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/sgcu65-backend-assignment /usr/local/bin/sgcu65-backend-assignment

RUN apt-get update && apt-get install -y libpq-dev
# -------------------------------------------------------

CMD ["sgcu65-backend-assignment"]