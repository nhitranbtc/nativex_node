PHONY: check
check:
	cargo check 

PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release

PHONY: debug
debug:
    cargo build --bin=nativex --package=nativex-node --message-format=json

.PHONY: run
run:
	./target/release/nativex-collator --dev --alice --tmp -lruntime=info --rpc-external   


.PHONY: run-with-features
run-with-features:
	cargo run  -- --alice --dev 

.PHONY: run-release
run-release:
	cargo run --release -- --chain=dev --tmp -lruntime=debug --rpc-external  