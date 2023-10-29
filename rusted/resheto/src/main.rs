#![allow(clippy::mut_range_bound)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n = String::new();
    let mut handle = io::BufWriter::new(io::stdout());
    io::stdin().read_line(&mut n)?;
    let (mut n, mut printed) = (n.trim().parse()?, false);
    for i in 2..=n {
        let mut count = 0;
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
                count += 1;
            }
            if printed {
                if count == 1 {
                    write!(handle, "*{i}")?;
                } else {
                    write!(handle, "*{i}^{count}")?;
                }
            } else {
                if count == 1 {
                    write!(handle, "{i}")?;
                } else {
                    write!(handle, "{i}^{count}")?;
                }
                printed = true;
            }
        }
    }
    handle.flush()?;
    Ok(())
}
