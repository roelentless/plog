# plog

Wraps any command and logs its output to `./plogs/` in the current directory.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh
```

## Usage

```sh
plog <command> [args...]
```

```sh
plog npm run build
plog make test
plog python train.py --epochs 100
```

## Log location

Each run writes to a file named after the command:

```
./plogs/<command-slug>.log
```

Example: `./plogs/npm-run-build.log`

Repeated runs overwrite the previous log.

## Behavior

- Output shows in the terminal exactly as normal
- stdout and stderr are both captured to the log file
- Exit code of the wrapped command is preserved
- If `.gitignore` exists in the current directory, `plogs` is appended to it
