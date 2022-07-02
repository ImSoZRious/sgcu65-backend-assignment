FROM rust:1.62-slim as builder

WORKDIR /usr/src/myapp
COPY . .
RUN apt-get update && apt-get install -y libpq-dev

RUN cargo install --path .

CMD ["sgcu65-backend-assignment"]

# FROM debian:buster-slim

# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

# CMD ["sgcu65-backend-assignment"]