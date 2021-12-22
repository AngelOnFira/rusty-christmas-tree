visualize:
	cargo run --example visualize

build:
	cd tree-writer \
	&& cross build \
		--release \
		--target armv7-unknown-linux-gnueabihf