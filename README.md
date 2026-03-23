# plog

Wraps any command and logs its output to `~/.plogs/`.

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

Every run creates a directory:

```
~/.plogs/<timestamp>-<command-slug>/
```

Example: `~/.plogs/20260323T115042-npm-run-build/`

## Files in each log directory

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

## Override log location

```
PLOG_DIR=/var/log/myapp plog ./server
```

## Install

```
make install
```

Installs to `~/.local/bin/plog`.
