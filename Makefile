INSTALL_DIR := $(HOME)/.local/bin

.PHONY: install test build

build:
	cargo build --release

install: build
	install -d $(INSTALL_DIR)
	install -m 755 target/release/plog $(INSTALL_DIR)/plog

test:
	cargo test
