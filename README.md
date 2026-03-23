# plog

Wraps any command and logs its output to `./plogs/` in the current directory.

## Usage

```
plog <command> [args...]
```

```
plog npm run build
plog make test
plog python train.py --epochs 100
```

## Log location

Each run writes to a directory named after the command:

```
./plogs/<command-slug>/
```

Example: `./plogs/npm-run-build/`

Repeated runs of the same command overwrite the previous logs.

## Files

| File | Contents |
|------|----------|
| `info.json` | Command, start time, PID, exit code |
| `stdout.log` | Standard output |
| `stderr.log` | Standard error |

Example `info.json`:

```json
{
  "command": "npm run build",
  "started": "2026-03-23T11:50:42+01:00",
  "pid": 12345,
  "exit_code": 0
}
```

## Behavior

- Fully transparent: stdin, stdout, and stderr all pass through normally
- Output is written to log files in real time
- Exit code of the wrapped command is preserved
- If `.gitignore` exists in the current directory, `plogs` is appended to it

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh
```

Or from source:

```sh
make install
```
