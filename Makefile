PHONY: check
check:
	cargo check --features with-development-runtime
PHONY: build
build:
	cargo build --features with-development-runtime
PHONY: debug
debug:
    cargo build --bin=nativex --package=nativex-node --message-format=json

.PHONY: run
run:
	cargo run --features with-development-runtime -- --alice --dev 

.PHONY: run-release
run-release:
	cargo run --release --features with-development-runtime -- --chain=dev --tmp -lruntime=debug --rpc-external  