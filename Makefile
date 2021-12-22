visualize:
	cargo run --bin tree-visualizer

build:
	cd tree-writer \
	&& cross build \
		--release \
		--target armv7-unknown-linux-gnueabihf

frontend:
	(cd tree-frontend \
	&& trunk serve)

frontend-release:
	(cd tree-frontend \
	&& trunk build --release)

setup:
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli