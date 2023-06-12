// same as tumba-umba.cpp, but for only the first task bla

use std::io;

fn main() -> io::Result<()> {
    let (mut alpha, mut k, mut words) = (String::new(), String::new(), Vec::new());
    io::stdin().read_line(&mut alpha)?;
    let alpha: Vec<char> = alpha.trim().chars().collect();

    io::stdin().read_line(&mut k)?;
    let k: usize = k.trim().parse().unwrap();

    gen(&mut words, &alpha, &mut vec![' '; k], 0);
    words.sort();

    for word in &words {
        println!("{}", word);
    }
    println!("{}", words.len());
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
