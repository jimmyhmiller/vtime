# vtime - a verbose `time` command

[![CI](https://github.com/jimmyhmiller/vtime/actions/workflows/ci.yml/badge.svg)](https://github.com/jimmyhmiller/vtime/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/vtime.svg)](https://crates.io/crates/vtime)

I've never ever remembered what `time`'s output meant. It is so terse that I just don't know what each of the numbers mean. So I made a human readable version.

## Usage

```sh
vtime [flags] <command> [args...]
```

## Example Output

```sh
❯ vtime -a sleep 1

  Wall clock time ·····  1.008 s
  User CPU time ·······  622 µs
  System CPU time ·····  1.6 ms
  CPU utilization ·····  0%
  Voluntary ctx sw ····  0
  Involuntary ctx sw ··  6
  Max memory (RSS) ····  1.2 MB
  Page faults (major) ·  1
  Page faults (minor) ·  214
  Disk reads ··········  0
  Disk writes ·········  0
```

## Flags

- `-a`, `--all`: show all available details
- `-c`, `--cpu`: show CPU utilization and context switches
- `-m`, `--memory`: show memory usage and page faults
- `-d`, `--disk`: show disk I/O
- `-h`, `--help`: show usage

## Features

- Zero dependencies
- Works on macOS and Linux

## Install

```sh
cargo install vtime
```

## Build

```sh
cargo build --release
```

The binary will be available at `target/release/vtime`.



## License

MIT. See [LICENSE](LICENSE).
