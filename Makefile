run: build deploy

visualize:
	cargo run --bin tree-visualizer --no-default-features

build:
	cross build \
		--package tree-writer \
		--release \
		--target armv7-unknown-linux-gnueabihf
	# strip target/armv7-unknown-linux-gnueabihf/release/rusty-tree

frontend:
	(cd tree-frontend \
	&& trunk serve \
		--proxy-backend=http://localhost:3030/ \
		--proxy-rewrite=/api/ \
		--public-url=/static/)

backend:
	(cd tree-backend \
	&& RUST_LOG=debug cargo run)

frontend-release:
	(cd tree-frontend \
	&& trunk build --release)

setup-web:
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli

deploy:
	du -sk target/armv7-unknown-linux-gnueabihf/release/rusty-tree
	scp target/armv7-unknown-linux-gnueabihf/release/rusty-tree tree:~/rusty-tree

move:
	scp rusty-tree tree:~/rusty-tree