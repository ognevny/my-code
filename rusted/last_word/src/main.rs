// same as lastword.cpp, but with Rust syntax

use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    print!(
        "{}",
        match line.trim().rfind(' ') {
            Some(n) => line[n + 1..].trim(),
            None => line.trim(),
        }
    );
}
