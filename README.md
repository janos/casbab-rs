# Camel Snake Kebab

[![Rust](https://github.com/janos/casbab-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/janos/casbab-rs/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/casbab.svg)](https://crates.io/crates/casbab)
[![Docs](https://docs.rs/casbab/badge.svg)](https://docs.rs/casbab)

Package casbab is a rust library for converting representation style of compound words or phrases. Different writing styles of compound words are used for different purposes in computer code and variables to easily distinguish type, properties or meaning.

Functions in this package are separating words from input string and constructing an appropriate phrase representation.

## Install

Run the following Cargo command in your project directory:

```sh
cargo add casbab
```

## Features

This library implements the following functions that return appropriate styles of compound words:

- camel: example `camelSnakeKebab`
- pascal: example `CamelSnakeKebab`
- snake: example `camel_snake_kebab`
- camel_snake: example `Camel_Snake_Kebab`
- screaming_snake: example `CAMEL_SNAKE_KEBAB`
- kebab: example `camel-snake-kebab`
- camel_kebab: example `Camel-Snake-Kebab`
- screaming_kebab: example `CAMEL-SNAKE-KEBAB`
- lower: example `camel snake kebab`
- title: example `Camel Snake Kebab`
- screaming: example `CAMEL SNAKE KEBAB`

## Performance

Benchmarks run `cargo bench` on MacBook Pro M1Pro yield these timings:

```
camel                   time:   [1.0097 µs 1.0164 µs 1.0241 µs]
pascal                  time:   [1.0299 µs 1.0371 µs 1.0444 µs]
snake                   time:   [681.45 ns 685.85 ns 689.92 ns]
camel_snake             time:   [1.1460 µs 1.1534 µs 1.1606 µs]
screaming_snake         time:   [1.0004 µs 1.0057 µs 1.0108 µs]
kebab                   time:   [717.66 ns 726.00 ns 734.36 ns]
camel_kebab             time:   [1.1163 µs 1.1238 µs 1.1309 µs]
screaming_kebab         time:   [999.09 ns 1.0103 µs 1.0210 µs]
title                   time:   [1.0305 µs 1.0412 µs 1.0518 µs]
lower                   time:   [640.24 ns 650.07 ns 659.39 ns]
screaming               time:   [929.30 ns 935.17 ns 941.02 ns]
```

## CLI

Build:

```sh
cargo install casbab --features build-binary
```

Get help:

```sh
casbab -h
```

Examples:

```sh
casbab screaming-snake "it is what it is"
casbab kebab my_snake_string some_kinda_Snake
echo "DO NOT SCREAM PLEASE" | casbab lower
cat variables.txt | casbab camel
```

## Versioning

Each version of the client is tagged and the version is updated accordingly.
To see the list of past versions, run `git tag`.

## Contributing

We love pull requests! Please see the [contribution guidelines](CONTRIBUTING.md).

## License

This library is distributed under the BSD-style license found in the [LICENSE](LICENSE) file.
