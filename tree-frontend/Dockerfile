FROM rust:1 as builder

WORKDIR /app

COPY . /app

RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.14.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- \
    && chmod +x trunk \
    && mv trunk /usr/bin \
    && cd tree-frontend \
    && rustup target add wasm32-unknown-unknown \
    && trunk build \
        --release


FROM joseluisq/static-web-server:2-alpine

COPY --from=builder /app/tree-frontend/dist/ /public/
