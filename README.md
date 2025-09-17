# fromdir

Change directory, run a command. Like `env -C` or `make -C`.

Essentially the same as `(cd "$1" && shift && exec "$@")`.

## Usage

```
fromdir <directory> <command> [args...]
```
