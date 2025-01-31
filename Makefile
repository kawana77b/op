fmt:
	@cargo fmt -- --emit=files

.PHONY: fmt

test:
	@cargo test

.PHONY: test

build:
	@cargo build
	@ls -lh target/debug

.PHONY: build

build-rel:
	@make clean
	@cargo build --release
	@ls -lh target/release
	@cargo build --release --target x86_64-pc-windows-gnu
	@cargo build --release --target x86_64-unknown-linux-musl

build-publish:
	@make clean
	@cargo build --release --target x86_64-pc-windows-gnu
	@cargo build --release --target x86_64-unknown-linux-musl
	@bash scripts/publish.sh

.PHONY: build-rel

clean:
	@rm -rf target
	@rm -rf dist

.PHONY: clean

credits:
	@cargo install cargo-credits
	@cargo-credits credits > CREDITS

.PHONY: credits

publish:
	@bash scripts/publish.sh