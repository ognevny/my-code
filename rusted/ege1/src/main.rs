//! the first code with tasks from EGE examW

use std::fs;

#[allow(dead_code)]
fn ex1() -> Result<usize, Box<dyn std::error::Error>> {
    let text = fs::read_to_string("11.txt")?;
    let newtext = text.replace("INFINITY", "@");
    let mut pos = Vec::new();
    for (i, char) in newtext.chars().enumerate() {
        if char == '@' {
            pos.push(i);
        }
    }
    let mut mx = 0;
    for i in 0..(pos.len() - 1001) {
        mx = mx.max(pos[i + 1001] - pos[i] - 1 + 7000 + 14);
    }
    Ok(mx)
}

fn main() {
    println!("ege1: run `cargo test` instead");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_all() {
        let ex1 = ex1().unwrap();
        assert_eq!(ex1, 36747);
    }
}
