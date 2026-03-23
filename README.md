# plog

Wrap any command with `plog` and its output is logged to your project folder — where your LLM can read it.

```sh
plog npm run build
plog python train.py --epochs 100
plog uvicorn app:app --reload
```

## Why

LLMs working in your project can't see what's running in your terminals. `plog` bridges that gap: every wrapped command leaves a structured log file in `./plogs/` that any agent, Claude, or tool can read directly from the filesystem.

No daemons. No configuration. No sidecars. Just prefix your command.

## What it logs

Each run creates:

```
./plogs/
  npm-run-build/
    output.log    ← full terminal output, stdout and stderr combined
    info.json     ← command, start time, exit code
```

```json
{
  "command": "npm run build",
  "started": "2026-03-23T11:50:42+01:00",
  "exit_code": 0
}
```

Repeated runs of the same command overwrite the previous log.

If `.gitignore` exists in the current directory, `plogs` is appended automatically.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh
```

Or from source:

```sh
make install
```

## Behavior

- Output shows in your terminal exactly as normal
- Exit code of the wrapped command is preserved
- Works on macOS and Linux
