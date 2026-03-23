#!/bin/sh
# plog installer
# Usage: curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh

DEST="$HOME/.local/bin/plog"

mkdir -p "$HOME/.local/bin"
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/plog -o "$DEST"
chmod +x "$DEST"
echo "plog installed to $DEST"
