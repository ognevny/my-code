// same as tumba-umba.cpp, but for only the first task

use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut alpha, mut k) = (String::new(), String::new());
    io::stdin().read_line(&mut alpha)?;
    let alpha: Vec<char> = alpha.trim().chars().collect();

    io::stdin().read_line(&mut k)?;
    let k = k.trim().parse()?;

    let mut handle = BufWriter::new(io::stdout());
    let mut words: Vec<String> = Vec::new();
    let count = gen(&alpha, &mut vec![' '; k], 0, &mut words);
    words.sort();
    for word in words {
        handle.write_all(word.as_bytes())?;
    }
    writeln!(handle, "{count}")?;

    Ok(())
}

fn gen(alpha: &Vec<char>, current: &mut Vec<char>, i: usize, words: &mut Vec<String>) -> usize {
    if i == current.len() {
        words.push(current.iter().collect::<String>() + "\n");
        return 1;
    }

    let mut count = 0;
    for j in 0..alpha.len() {
        current[i] = alpha[j];
        count += gen(alpha, current, i + 1, words);
    }
    count
}
