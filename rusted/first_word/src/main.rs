//! bla
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    print!(
        "{}",
        match input.find(' ') {
            Some(n) => input[..n].trim(),
            None => input.trim(),
        }
    );
    Ok(())
}
