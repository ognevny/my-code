// same as tumba-umba.cpp, but for only the first task

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut alpha, mut k, mut words) = (String::new(), String::new(), Vec::new());
    io::stdin().read_line(&mut alpha)?;
    let alpha: Vec<char> = alpha.trim().chars().collect();

    io::stdin().read_line(&mut k)?;
    let k = k.trim().parse()?;

    gen(&mut words, &alpha, &mut vec![' '; k], 0);
    words.sort();

    let mut handle = io::BufWriter::new(io::stdout());
    for word in &words {
        writeln!(handle, "{word}")?;
    }
    writeln!(handle, "{}", words.len())?;
    Ok(())
}

fn gen(words: &mut Vec<String>, alpha: &Vec<char>, current: &mut Vec<char>, i: usize) {
    if i == current.len() {
        words.push(current.iter().collect());
        return;
    }

    for j in 0..alpha.len() {
        current[i] = alpha[j];
        gen(words, alpha, current, i + 1);
    }
}
