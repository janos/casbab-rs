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

use std::usize;

/// *Camel* case is the practice of writing compound words
/// or phrases such that each word or abbreviation in the
/// middle of the phrase begins with a capital letter,
/// with no spaces or hyphens.
///
/// Example: `camelSnakeKebab`.
pub fn camel(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 1);
    w.join("")
}

/// *Pascal* case is a variant of Camel case writing where
/// the first letter of the first word is always capitalized.
///
/// Example: `CamelSnakeKebab`.
pub fn pascal(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 0);
    w.join("")
}

/// *Snake* case is the practice of writing compound words
/// or phrases in which the elements are separated with
/// one underscore character (_) and no spaces, with all
/// element letters lowercased within the compound.
///
/// Example: `camel_snake_kebab`.
pub fn snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    "_".repeat(head) + words(s).join("_").as_str() + "_".repeat(tail).as_str()
}

/// *Camel snake* case is a variant of Camel case with
/// each element's first letter uppercased.
///
/// Example: `Camel_Snake_Kebab`.
pub fn camel_snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    let mut w = words(s);
    camel_slice(&mut w, 0);
    "_".repeat(head) + w.join("_").as_str() + "_".repeat(tail).as_str()
}

/// *Screaming snake* case is a variant of Camel case with
/// all letters uppercased.
///
/// Example: `CAMEL_SNAKE_KEBAB`.
pub fn screaming_snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    let mut w = words(s);
    scream_slice(&mut w);
    "_".repeat(head) + w.join("_").as_str() + "_".repeat(tail).as_str()
}

/// *Kebab* case is the practice of writing compound words
/// or phrases in which the elements are separated with
/// one hyphen character (-) and no spaces, with all
/// element letters lowercased within the compound.
///
/// Example: `camel-snake-kebab`.
pub fn kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    "-".repeat(head) + words(s).join("-").as_str() + "-".repeat(tail).as_str()
}

/// *Camel kebab* case is a variant of Kebab case with
/// each element's first letter uppercased.
///
/// Example: `Camel-Snake-Kebab`.
pub fn camel_kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    let mut w = words(s);
    camel_slice(&mut w, 0);
    "-".repeat(head) + w.join("-").as_str() + "-".repeat(tail).as_str()
}

/// *Screaming kebab* case is a variant of Kebab case with
/// all letters uppercased.
///
/// Example: `CAMEL-SNAKE-KEBAB`.
pub fn screaming_kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    let mut w = words(s);
    scream_slice(&mut w);
    "-".repeat(head) + w.join("-").as_str() + "-".repeat(tail).as_str()
}

/// *Lower* is returning detected words, not in a compound
/// form, but separated by one space character with all
/// letters in lower case.
///
/// Example: `camel snake kebab`.
pub fn lower(s: &str) -> String {
    words(s).join(" ")
}

/// *Title* is returning detected words, not in a compound
/// form, but separated by one space character with first
/// character in all letters in upper case and all other
/// letters in lower case.
///
/// Example: `Camel Snake Kebab`.
pub fn title(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 0);
    w.join(" ")
}

/// *Screaming* is returning detected words, not in a compound
/// form, but separated by one space character with all
/// letters in upper case.
///
/// Example: `CAMEL SNAKE KEBAB`.
pub fn screaming(s: &str) -> String {
    let mut w = words(s);
    scream_slice(&mut w);
    w.join(" ")
}

fn words(s: &str) -> Vec<String> {
    let mut start: usize = 0;
    let l = s.len();
    let mut prev_lower = false;
    let mut prev_upper = false;
    let mut prev_upper_location: usize = 0;

    let mut words: Vec<String> = Vec::new();
    for (i, c) in s.char_indices() {
        match c {
            '-' | '_' | ' ' => {
                if start != i {
                    words.push(s[start..i].to_lowercase());
                };
                start = i + 1;
                prev_lower = false;
                prev_upper = false;
                prev_upper_location = 0;
                continue;
            }
            _ => (),
        };

        if c.is_uppercase() {
            prev_upper = true;
            prev_upper_location = i;
            if prev_lower {
                if start != i {
                    words.push(s[start..i].to_lowercase())
                }
                start = i;
                prev_lower = false;
            };
        } else {
            prev_lower = true;
            if prev_upper && prev_upper_location > 0 {
                if start != prev_upper_location {
                    words.push(s[start..prev_upper_location].to_lowercase())
                }
                start = prev_upper_location;
                prev_upper = false;
                prev_upper_location = 0;
            };
        }
    }
    if start != l {
        words.push(s[start..].to_lowercase());
    }
    words
}

fn scream_slice(s: &mut [String]) {
    for e in s {
        *e = e.to_uppercase()
    }
}

fn camel_slice(s: &mut [String], start: usize) {
    for e in s.iter_mut().skip(start) {
        to_titlecase(e);
    }
}

fn to_titlecase(e: &mut String) {
    *e = match e.chars().next() {
        None => e.to_owned(),
        Some(f) => {
            e.replace_range(..f.len_utf8(), &f.to_uppercase().to_string()[..]);
            e.to_owned()
        }
    };
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
