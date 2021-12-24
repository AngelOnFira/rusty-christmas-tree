FROM rust:1 as builder

WORKDIR /app

COPY . /app

RUN cd tree-backend \
    && cargo build --release


FROM debian:buster-slim

COPY --from=builder /app/target/release/tree-backend /app/tree-backend

ENTRYPOINT ["/app/tree-backend"]
