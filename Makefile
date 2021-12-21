visualize:
	cargo run --example visualize

build:
	cross build \
		--release \
		--target armv7-unknown-linux-gnueabihf \
		--features pi \
		--no-default-features