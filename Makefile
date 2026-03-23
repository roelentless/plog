INSTALL_DIR := $(HOME)/.local/bin

.PHONY: build install test fmt check

build:
	cargo build

install:
	cargo build --release
	install -d $(INSTALL_DIR)
	install -m 755 target/release/plog $(INSTALL_DIR)/plog

test:
	cargo test

fmt:
	cargo fmt

check:
	cargo fmt --check
	cargo clippy -- -W clippy::all
	cargo test
