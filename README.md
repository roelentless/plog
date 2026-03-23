# plog

Give your LLM eyes on a running process.

```sh
plog npm run build
```

Creates:

```
./plogs/npm-run-build/
  output.log
  info.json
```

```json
{
  "command": "npm run build",
  "started": "2026-03-23T11:50:42+01:00",
  "exit_code": 0
}
```

Your terminal output is unchanged. Repeated runs overwrite the previous log. If `.gitignore` exists, `plogs` is added automatically.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/roelentless/plog/main/install.sh | sh
```
