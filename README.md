# plog

Records stdout and stderr of a command to a file, exactly as it appeared in the terminal. A thin wrapper around `script`.

```
$ plog -h
plog — record a command's stdout and stderr to a file.

Usage: plog <command> [args...]

Example:
  plog npm run build
  → plogs/npm-run-build/output.log  (stdout and stderr merged)

Options:
  --list   show captured logs in this directory

Tip: tail -50 plogs/npm-run-build/output.log  (limit output lines)

Note: background processes started by the command will be killed when
it exits. plog only works correctly with foreground commands.
```

Writes to `./plogs/npm-run-build/`:

**`output.log`** — merged stdout and stderr, exactly as it appeared in the terminal.

**`info.json`**:
```json
{
  "command": "npm run build",
  "started": "2026-03-23T11:50:42+01:00"
}
```

Your terminal is unchanged. Repeated runs overwrite the previous log. If `.gitignore` exists, `plogs` is added automatically.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh
```
