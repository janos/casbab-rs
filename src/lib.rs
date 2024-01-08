// Copyright (c) 2024, Janoš Guljaš <janos@resenje.org>
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! # Camel Snake Kebab
//!
//! Package *casbab* is a rust library for converting
//! representation style of compound words or phrases.
//! Different writing styles of compound words are used
//! for different purposes in computer code and variables
//! to easily distinguish type, properties or meaning.
//!
//! Functions in this package are separating words from
//! input string and constructing an appropriate phrase
//! representation.
//!
//! Examples:
//!
//! - `kebab("camel_snake_kebab")` returns `camel-snake-kebab`
//! - `screaming_snake("camel_snake_kebab")` returns `CAMEL_SNAKE_KEBAB`
//! - `camel("camel_snake_kebab")` returns `camelSnakeKebab`
//! - `pascal("camel_snake_kebab")` returns `CamelSnakeKebab`
//! - `snake("camelSNAKEKebab")` returns `camel_snake_kebab`
//!
//! Word separation works by detecting delimiters hyphen (-),
//! underscore (_), space ( ) and letter case change.
//!
//! Note: Leading and trailing separators will be preserved
//! only within the Snake family or within the Kebab family
//! and not across them. This restriction is based on different
//! semantics between different writings.
//!
//! Examples:
//!
//! - `camel_snake("__camel_snake_kebab__")` returns `__Camel_Snake_Kebab__`
//! - `kebab("__camel_snake_kebab")` returns `camel-snake-kebab`
//! - `screaming("__camel_snake_kebab")` returns `CAMEL SNAKE KEBAB`
//! - `camel_kebab("--camel-snake-kebab")` returns `--Camel-Snake-Kebab`
//! - `snake("--camel-snake-kebab")` returns `camel_snake_kebab`
//! - `screaming("--camel-snake-kebab")` returns `CAMEL SNAKE KEBAB`

use std::{fmt::Write, usize};

/// *Camel* case is the practice of writing compound words
/// or phrases such that each word or abbreviation in the
/// middle of the phrase begins with a capital letter,
/// with no spaces or hyphens.
///
/// Example: `camelSnakeKebab`.
pub fn camel(s: &str) -> String {
    casbab(s, to_titlecase, to_lowercase)
}

/// *Pascal* case is a variant of Camel case writing where
/// the first letter of the first word is always capitalized.
///
/// Example: `CamelSnakeKebab`.
pub fn pascal(s: &str) -> String {
    casbab(s, to_titlecase, to_titlecase)
}

/// *Snake* case is the practice of writing compound words
/// or phrases in which the elements are separated with
/// one underscore character (_) and no spaces, with all
/// element letters lowercased within the compound.
///
/// Example: `camel_snake_kebab`.
pub fn snake(s: &str) -> String {
    casbab_wrap(s, '_', to_lowercase)
}

/// *Camel snake* case is a variant of Camel case with
/// each element's first letter uppercased.
///
/// Example: `Camel_Snake_Kebab`.
pub fn camel_snake(s: &str) -> String {
    casbab_wrap(s, '_', to_titlecase)
}

/// *Screaming snake* case is a variant of Camel case with
/// all letters uppercased.
///
/// Example: `CAMEL_SNAKE_KEBAB`.
pub fn screaming_snake(s: &str) -> String {
    casbab_wrap(s, '_', to_uppercase)
}

/// *Kebab* case is the practice of writing compound words
/// or phrases in which the elements are separated with
/// one hyphen character (-) and no spaces, with all
/// element letters lowercased within the compound.
///
/// Example: `camel-snake-kebab`.
pub fn kebab(s: &str) -> String {
    casbab_wrap(s, '-', to_lowercase)
}

/// *Camel kebab* case is a variant of Kebab case with
/// each element's first letter uppercased.
///
/// Example: `Camel-Snake-Kebab`.
pub fn camel_kebab(s: &str) -> String {
    casbab_wrap(s, '-', to_titlecase)
}

/// *Screaming kebab* case is a variant of Kebab case with
/// all letters uppercased.
///
/// Example: `CAMEL-SNAKE-KEBAB`.
pub fn screaming_kebab(s: &str) -> String {
    casbab_wrap(s, '-', to_uppercase)
}

/// *Lower* is returning detected words, not in a compound
/// form, but separated by one space character with all
/// letters in lower case.
///
/// Example: `camel snake kebab`.
pub fn lower(s: &str) -> String {
    casbab_separate(s, ' ', to_lowercase)
}

/// *Title* is returning detected words, not in a compound
/// form, but separated by one space character with first
/// character in all letters in upper case and all other
/// letters in lower case.
///
/// Example: `Camel Snake Kebab`.
pub fn title(s: &str) -> String {
    casbab_separate(s, ' ', to_titlecase)
}

/// *Screaming* is returning detected words, not in a compound
/// form, but separated by one space character with all
/// letters in upper case.
///
/// Example: `CAMEL SNAKE KEBAB`.
pub fn screaming(s: &str) -> String {
    casbab_separate(s, ' ', to_uppercase)
}

fn casbab(
    s: &str,
    transform: fn(&str) -> String,
    transform_first_word: fn(&str) -> String,
) -> String {
    let mut r = String::new();
    let mut s = s;
    let (w, rest) = first_word(s);
    r += &transform_first_word(w);
    s = rest;
    loop {
        let (w, rest) = first_word(s);
        if w.is_empty() {
            break r;
        }
        r += &transform(w);
        if rest.is_empty() {
            break r;
        }
        s = rest;
    }
}

fn casbab_separate(s: &str, separator: char, transform: fn(&str) -> String) -> String {
    let mut r = String::new();
    let mut s = s;
    let (w, rest) = first_word(s);
    r += &transform(w);
    s = rest;
    loop {
        let (w, rest) = first_word(s);
        if w.is_empty() {
            break r;
        }
        _ = r.write_char(separator);
        r += &transform(w);
        if rest.is_empty() {
            break r;
        }
        s = rest;
    }
}

fn casbab_wrap(s: &str, separator: char, transform: fn(&str) -> String) -> String {
    let mut r = String::new();

    let (head, tail) = head_tail_count(s, separator);

    for _ in 0..head {
        _ = r.write_char(separator);
    }

    let mut s = s;
    let (w, rest) = first_word(s);
    r += &transform(w);
    s = rest;
    loop {
        let (w, rest) = first_word(s);
        if w.is_empty() {
            break;
        }
        _ = r.write_char(separator);
        r += &transform(w);
        if rest.is_empty() {
            break;
        }
        s = rest;
    }

    for _ in 0..tail {
        _ = r.write_char(separator);
    }

    r
}

fn first_word(s: &str) -> (&str, &str) {
    let mut start: usize = 0;
    let l = s.len();
    let mut prev_lower = false;
    let mut prev_upper = false;
    let mut prev_upper_location: usize = 0;

    for (i, c) in s.char_indices() {
        if c == '-' || c == '_' || c == ' ' {
            if start != i {
                return (&s[start..i], &s[i..]);
            };
            start = i + 1;
            prev_lower = false;
            prev_upper = false;
            prev_upper_location = 0;
            continue;
        }

        if c.is_uppercase() {
            prev_upper = true;
            prev_upper_location = i;
            if prev_lower {
                if start != i {
                    return (&s[start..i], &s[i..]);
                }
                start = i;
                prev_lower = false;
            };
        } else {
            prev_lower = true;
            if prev_upper && prev_upper_location > 0 {
                if start != prev_upper_location {
                    return (&s[start..prev_upper_location], &s[prev_upper_location..]);
                }
                start = prev_upper_location;
                prev_upper = false;
                prev_upper_location = 0;
            };
        }
    }
    if start != l {
        return (&s[start..], "");
    }
    ("", "")
}

fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}
fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn to_titlecase(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => s.to_string(),
        Some(f) => f.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
    }
}

fn head_tail_count(s: &str, sub: char) -> (usize, usize) {
    let mut head: usize = 0;
    for (i, char) in s.char_indices() {
        if char != sub {
            head = i;
            break;
        }
    }
    let mut tail: usize = 0;
    for (i, char) in s.chars().rev().enumerate() {
        if char != sub {
            tail = i;
            break;
        }
    }
    (head, tail)
}
