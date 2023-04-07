// bubble sort

use std::io::stdin;

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] { arr.swap(j + 1, j) } } } }

fn main() {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    let mut nums: Vec<i32> = nums.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();     
    bubble_sort(&mut nums);
    println!("{:?}", nums); }
