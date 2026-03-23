# plog — agent instructions

## Code style

- No defensive code. No fallbacks. No defaults.
- All variables required. No `Option` used to paper over missing values.
- No swallowed errors. Every failure must propagate or panic with a message.
- Avoid conditional code. Require everything to be present and correct.
- Minimal. No abstraction for its own sake.

## Rust specifics

- `.expect("message")` over `.unwrap()` — the message must say what failed.
- Never `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, or silent `let _ =`.
- Unreachable branches must use `unreachable!()` or `panic!()`, never a silent default.
- No `#[cfg(...)]` guards to paper over platform differences — this runs on Unix.

## Structure

- Single binary: `src/main.rs`.
- Integration tests only: `tests/integration_test.rs`.
- Installs to `~/.local/bin` via `make install`.

## What plog does

Wraps a command, logs its output to `./plogs/<slug>/` in the current working directory.

- `stdout.log` — child stdout
- `stderr.log` — child stderr
- `info.json` — `command`, `started`, `pid`, `exit_code`

Repeated runs of the same command overwrite the previous logs.
Exit code, stdin, stdout, stderr, and signals are fully transparent.
If `.gitignore` exists in the current directory, `plogs` is appended to it.
