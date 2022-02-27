# base64-cli [![crates.io version](https://img.shields.io/crates/v/base64-cli.svg)](https://crates.io/crates/base64-cli) [![crates.io downloads](https://img.shields.io/crates/d/base64-cli.svg)](https://crates.io/crates/base64-cli)

A CLI tool for base64 which supports both native and [WebAssembly](#WebAssembly)

*Note: This project is a fork of the original Rust implementation: [rust-base64](https://github.com/marshallpierce/rust-base64).*

## Installation

You can install this using the `cargo install` command:

```bash
$ cargo install base64-cli
```

### WebAssembly

This application also provides a wasm package.
You can install it using [`wapm`](https://wapm.io/help/install) by the following command:

```bash
$ wapm install ken-matsui/base64
```

## Usage

```bash
$ base64-cli --help
base64-cli 0.1.0
Ken Matsui <26405363+ken-matsui@users.noreply.github.com>
A CLI tool for base64

USAGE:
    base64 <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decode base64 string to string
    encode    Encode string to base64 string
    help      Print this message or the help of the given subcommand(s)
```

### WebAssembly

```bash
$ wapm run base64 --help
base64-cli 0.1.0
Ken Matsui <26405363+ken-matsui@users.noreply.github.com>
A CLI tool for base64

USAGE:
    base64 <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decode base64 string to string
    encode    Encode string to base64 string
    help      Print this message or the help of the given subcommand(s)
```

## Examples

### encode

```bash
$ base64-cli encode hello
aGVsbG8=
```

### decode

```bash
$ base64-cli decode aGVsbG8=
hello
```

### WebAssembly

#### encode

```bash
$ wapm run base64 encode hello
aGVsbG8=
```

#### decode

```bash
$ wapm run base64 decode aGVsbG8=
hello
```

## Contribution

### Build

```bash
$ cargo build
```

Or you can directly execute the binary:

```bash
$ cargo run
```

#### WebAssembly

```bash
$ rustup target add wasm32-wasi
$ cargo build --target wasm32-wasi
$ wasmer run target/wasm32-wasi/debug/base64-cli.wasm encode hello
```

### Test

This command can also test C API.

```bash
$ cargo build
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/base64-cli/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```

#### [wapm.io](https://wapm.io/)

```bash
$ cargo build --release --target wasm32-wasi
$ wapm publish
```
