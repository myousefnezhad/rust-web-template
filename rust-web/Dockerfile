FROM rust:1.85.0-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --bin srv_backend --release

FROM debian:bookworm-slim as service 
RUN apt-get update
RUN apt-get install libssl3 ca-certificates libssl-dev -y
WORKDIR /
COPY --from=builder /app/target/release/srv_backend /
ENV APP="/srv_backend"
CMD ["sh", "-c", "${APP}"]
