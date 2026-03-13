# vtime - a verbose `time` command

I've never ever remembered what `time`'s output meant. It is so terse that I just don't know what each of the numbers mean. So I made a human readable version.

## Features

- Zero dependencies
- Works on macOS and Linux

## Build

```sh
cargo build --release
```

The binary will be available at `target/release/vtime`.

## Usage

```sh
vtime [flags] <command> [args...]
```


## Flags

- `-a`, `--all`: show all available details
- `-c`, `--cpu`: show CPU utilization and context switches
- `-m`, `--memory`: show memory usage and page faults
- `-d`, `--disk`: show disk I/O
- `-h`, `--help`: show usage

## License

MIT. See [LICENSE](LICENSE).
