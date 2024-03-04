//! find a maximum HEX number in string

use {
    rayon::prelude::*,
    std::{error::Error, io::stdin},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = String::new();
    let mut nums: Vec<String> = vec![];
    stdin().read_line(&mut data)?;
    let mut word = String::new();
    for char in data.chars() {
        if usize::from_str_radix(&char.to_string(), 16).is_ok() {
            word.push(char);
        } else {
            if !word.is_empty() {
                nums.push(word.clone());
            }
            word = String::new();
        }
    }
    let max = nums
        .par_iter()
        .max_by_key(|&num| usize::from_str_radix(num, 16).unwrap())
        .unwrap();
    println!("{max}");
    Ok(())
}
