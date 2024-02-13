// same as lastword.cpp, but with Rust syntax

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    print!("{}", line.trim().rfind(' ').map_or(line.trim(), |n| line[n + 1..].trim()));
    Ok(())
}
