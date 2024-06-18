//! find a maximum HEX number in string

use {
    rayon::prelude::*,
    std::{error::Error, io::stdin},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = String::new();
    stdin().read_line(&mut data)?;
    let nums: Vec<String> = data
        .chars()
        .fold((String::new(), vec![]), |(mut word, mut nums), char| {
            if char.is_ascii_digit()
                || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&char.to_ascii_lowercase())
            {
                word.push(char);
            } else {
                if !word.is_empty() {
                    nums.push(word.clone());
                }
                word.clear();
            }
            (word, nums)
        })
        .1;
    let max =
        nums.into_par_iter().max_by_key(|num| usize::from_str_radix(num, 16).unwrap()).unwrap();
    println!("{max}");
    Ok(())
}
