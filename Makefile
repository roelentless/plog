INSTALL_DIR := $(HOME)/.local/bin

.PHONY: install test

install:
	install -d $(INSTALL_DIR)
	install -m 755 plog $(INSTALL_DIR)/plog

test:
	python3 test.py
