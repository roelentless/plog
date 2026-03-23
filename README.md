# plog

Give your LLM eyes on a running process.

A thin wrapper around the `script` command. Merges stdout and stderr into a single log file per command.

```sh
plog npm run build
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
