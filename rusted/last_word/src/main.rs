// same as lastword.cpp, but with Rust syntax

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    print!(
        "{}",
        match line.trim().rfind(' ') {
            Some(n) => line[n + 1..].trim(),
            None => line.trim(),
        }
    );
    Ok(())
}
