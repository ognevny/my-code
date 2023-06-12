//! bla

use std::io;

fn main() -> io::Result<()> {
    let (mut maxstr, mut input) = ("", String::new());
    io::stdin().read_line(&mut input)?;
    input
        .split_whitespace()
        .for_each(|x| maxstr = if x.len() > maxstr.len() { x } else { maxstr });
    print!("{maxstr}");
    Ok(())
}
