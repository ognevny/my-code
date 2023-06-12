// same as lastword.cpp, but with Rust syntax bla

use std::io;

fn main() -> io::Result<()> {
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
