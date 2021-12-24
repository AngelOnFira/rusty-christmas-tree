FROM rust:1 as backend-builder

WORKDIR /app

COPY . /app

RUN cd tree-backend \
    && cargo build --release

FROM rust:1 as frontend-builder

WORKDIR /app

COPY . /app

RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.14.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- \
    && chmod +x trunk \
    && mv trunk /usr/bin \
    && cd tree-frontend \
    && rustup target add wasm32-unknown-unknown \
    && trunk build \
        --public-url=/static/ \
        --release

FROM debian:buster-slim

COPY --from=backend-builder /app/target/release/tree-backend /app/tree-backend
COPY --from=frontend-builder /app/tree-frontend/dist/ /public/

ENTRYPOINT ["/app/tree-backend"]
