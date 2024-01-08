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
camel                   time:   [878.76 ns 884.24 ns 889.59 ns]
pascal                  time:   [858.82 ns 866.89 ns 874.65 ns]
snake                   time:   [537.22 ns 540.21 ns 543.24 ns]
camel_snake             time:   [870.81 ns 876.18 ns 881.71 ns]
screaming_snake         time:   [551.08 ns 552.52 ns 553.97 ns]
kebab                   time:   [537.33 ns 540.59 ns 543.96 ns]
camel_kebab             time:   [863.78 ns 871.67 ns 879.36 ns]
screaming_kebab         time:   [554.04 ns 555.31 ns 556.61 ns]
lower                   time:   [530.32 ns 531.99 ns 533.61 ns]
title                   time:   [877.86 ns 884.77 ns 891.13 ns]
screaming               time:   [560.80 ns 563.78 ns 566.98 ns]
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
