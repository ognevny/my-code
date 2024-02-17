use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n = String::new();
    let mut handle = io::BufWriter::new(io::stdout());
    io::stdin().read_line(&mut n)?;
    let (mut n, mut printed): (u64, bool) = (n.trim().parse()?, false);
    let (mut count, m) = (0, n);
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    match count {
        1 => {
            write!(handle, "2")?;
            printed = true;
        },
        2.. => {
            write!(handle, "2^{count}")?;
            printed = true;
        },
        _ => (),
    }
    for i in (3..).step_by(2).take_while(|i| i * i <= m) {
        count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        match count {
            1 =>
                if printed {
                    write!(handle, "*{i}")?;
                } else {
                    write!(handle, "{i}")?;
                    printed = true;
                },
            2.. =>
                if printed {
                    write!(handle, "*{i}^{count}")?;
                } else {
                    write!(handle, "{i}^{count}")?;
                    printed = true;
                },
            _ => (),
        }
    }
    if n > 2 {
        if printed {
            write!(handle, "*{n}")?;
        } else {
            write!(handle, "{n}")?;
        }
    }
    handle.flush()?;
    Ok(())
}
