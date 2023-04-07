use std::io;

fn main() {
    let stdin = io::stdin;
    
    let mut num = String::new();
    stdin().read_line(&mut num).expect("Failed to read line");

    let num: u32 = num.trim().parse().expect("NaN");

    println!("{num}'s Fibonacci number is {}", fibr(num)); }

fn fibr(n: u32) -> u32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    fibr(n - 1) + fibr(n - 2) }