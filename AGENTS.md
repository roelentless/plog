# plog — agent instructions

## What plog does

Wraps any command and logs its output to `./plogs/<command-slug>.log` in the
current working directory. Output shows in the terminal as normal. Repeated
runs overwrite the previous log.

## Implementation

Single bash script: `plog`. No build step. Installed to `~/.local/bin/plog`.

## Code style

- No defensive code. No fallbacks. No defaults.
- No swallowed errors.
- Minimal. Every line earns its place.
