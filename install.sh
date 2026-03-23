#!/bin/sh
# plog installer
# Usage: curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh

REPO="roelentless/plog"
REPO_URL="https://github.com/${REPO}"
DEST_DIR="$HOME/.local/bin"
DEST="$DEST_DIR/plog"

if [ -t 1 ]; then
  BOLD='\033[1m'
  GREEN='\033[32m'
  RED='\033[31m'
  YELLOW='\033[33m'
  CYAN='\033[36m'
  RESET='\033[0m'
else
  BOLD='' GREEN='' RED='' YELLOW='' CYAN='' RESET=''
fi

ok()   { printf "  ${GREEN}✓${RESET}  %s\n" "$1"; }
info() { printf "  ${CYAN}→${RESET} %s\n" "$1"; }
warn() { printf "  ${YELLOW}!${RESET} %s\n" "$1"; }
err()  { printf "  ${RED}✗${RESET} %s\n" "$1" >&2; }

if ! command -v curl >/dev/null 2>&1; then
  err "curl is required but not installed."
  exit 1
fi

# ── Platform detection ────────────────────────────────────────────────────────

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Darwin) PLATFORM="macos" ;;
  Linux)  PLATFORM="linux" ;;
  *)
    err "Unsupported OS: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH_LABEL="amd64" ;;
  arm64|aarch64) ARCH_LABEL="arm64" ;;
  *)
    err "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

# ── Latest version ────────────────────────────────────────────────────────────

LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$LATEST" ]; then
  err "Could not determine latest version from GitHub."
  err "Visit: ${REPO_URL}/releases"
  exit 1
fi

# ── Download and install ──────────────────────────────────────────────────────

TARBALL="plog-${PLATFORM}-${ARCH_LABEL}.tar.gz"
URL="${REPO_URL}/releases/download/${LATEST}/${TARBALL}"

printf "\n  ${BOLD}plog installer${RESET}\n\n"
info "Version:  ${LATEST}"
info "Platform: ${PLATFORM}/${ARCH_LABEL}"
info "Dest:     ${DEST}"
printf "\n"

TMP_DIR=$(mktemp -d)

if ! curl -fsSL -o "$TMP_DIR/$TARBALL" "$URL"; then
  rm -rf "$TMP_DIR"
  err "Download failed: ${URL}"
  exit 1
fi

tar xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"
mkdir -p "$DEST_DIR"
mv "$TMP_DIR/plog" "$DEST"
chmod +x "$DEST"
rm -rf "$TMP_DIR"

ok "plog ${LATEST} installed to ${DEST}"

# ── PATH ──────────────────────────────────────────────────────────────────────

case ":$PATH:" in
  *":$DEST_DIR:"*) ;;
  *)
    PATH_LINE="export PATH=\"${DEST_DIR}:\$PATH\""
    CURRENT_SHELL=$(basename "${SHELL:-/bin/sh}")
    if [ "$CURRENT_SHELL" = "zsh" ]; then
      PROFILE="$HOME/.zshenv"
    else
      PROFILE="$HOME/.profile"
    fi
    printf '\n%s\n' "$PATH_LINE" >> "$PROFILE"
    ok "Added PATH entry to ${PROFILE}"
    warn "Open a new terminal or run: source ${PROFILE}"
    ;;
esac

printf "\n"
