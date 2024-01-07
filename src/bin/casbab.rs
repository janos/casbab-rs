// Copyright (c) 2024, Janoš Guljaš <janos@resenje.org>
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use atty::Stream;
use clap::{arg, error::ErrorKind, Command};
use std::io;

fn main() {
    let mut cmd = Command::new("casbab")
        .about("Camel Snake Kebab (https://github.com/janos/casbab-rs)")
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .author("Janos Guljas <janos@resenje.org>")
        .arg(arg!(<dialect> "Convert into dialect. Possible values:
- camel            `camelSnakeKebab`
- pascal           `CamelSnakeKebab`
- snake            `camel_snake_kebab`
- camel-snake      `Camel_Snake_Kebab`
- screaming-snake  `CAMEL_SNAKE_KEBAB`
- kebab            `camel-snake-kebab`
- camel-kebab      `Camel-Snake-Kebab`
- screaming-kebab  `CAMEL-SNAKE-KEBAB`
- lower            `camel snake kebab`
- title            `Camel Snake Kebab`
- screaming        `CAMEL SNAKE KEBAB`

If no phrases are provided as arguments, arguments will be read from the
Stdin as the new-line separated list.
"))
        .arg(
            arg!(<phrases> ... "phrases to convert")
                .trailing_var_arg(true)
                .required(false),
        );

    let matches = cmd.clone().get_matches();

    let dialect: &str = matches
        .get_one::<String>("dialect")
        .expect("`dialect` is required")
        .as_str();

    let func = match dialect {
        "camel" => casbab::camel,
        "pascal" => casbab::pascal,
        "snake" => casbab::snake,
        "camel-snake" => casbab::camel_snake,
        "screaming-snake" => casbab::screaming_snake,
        "kebab" => casbab::kebab,
        "camel-kebab" => casbab::camel_kebab,
        "screaming-kebab" => casbab::screaming_kebab,
        "lower" => casbab::lower,
        "title " => casbab::title,
        "screaming" => casbab::screaming,
        _ => {
            cmd.error(ErrorKind::InvalidSubcommand, "Invalid dialect")
                .exit();
        }
    };

    let phrases: Vec<String> = match matches.get_many("phrases") {
        None => {
            if atty::is(Stream::Stdin) {
                cmd.error(
                    ErrorKind::InvalidValue,
                    "Missing phases either from arguments or stdin",
                )
                .exit();
            }
            let mut phrases: Vec<String> = Vec::new();
            for line in io::stdin().lines() {
                phrases.push(line.unwrap())
            }
            phrases
        }
        Some(o) => o.cloned().collect(),
    };

    for p in phrases {
        println!("{}", func(p.as_str()));
    }
}
