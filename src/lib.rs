use std::usize;

pub fn camel(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 1);
    w.join("")
}

pub fn pascal(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 0);
    w.join("")
}

pub fn snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    "_".repeat(head) + words(s).join("_").as_str() + "_".repeat(tail).as_str()
}

pub fn camel_snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    let mut w = words(s);
    camel_slice(&mut w, 0);
    "_".repeat(head) + w.join("_").as_str() + "_".repeat(tail).as_str()
}

pub fn screaming_snake(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '_');
    let mut w = words(s);
    scream_slice(&mut w);
    "_".repeat(head) + w.join("_").as_str() + "_".repeat(tail).as_str()
}

pub fn kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    "-".repeat(head) + words(s).join("-").as_str() + "-".repeat(tail).as_str()
}

pub fn camel_kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    let mut w = words(s);
    camel_slice(&mut w, 0);
    "-".repeat(head) + w.join("-").as_str() + "-".repeat(tail).as_str()
}

pub fn screaming_kebab(s: &str) -> String {
    let (head, tail) = head_tail_count(s, '-');
    let mut w = words(s);
    scream_slice(&mut w);
    "-".repeat(head) + w.join("-").as_str() + "-".repeat(tail).as_str()
}

pub fn lower(s: &str) -> String {
    words(s).join(" ")
}

pub fn title(s: &str) -> String {
    let mut w = words(s);
    camel_slice(&mut w, 0);
    w.join(" ")
}

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
