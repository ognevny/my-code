// same as lastword.cpp, but with Rust syntax

use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let last_space = line.trim().rfind(' ').unwrap_or(0);
    let last_word = line[last_space + 1..].trim();
    print!("{last_word}");
}
