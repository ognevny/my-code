use std::io::stdin;
use std::f64::consts::PI;

fn f(x: f64) -> f64 {
    x.sin() / x }


fn lrect(a: f64, b: f64, n: u64) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += f(a + i as f64 * (b - a) / (n as f64)) }
    s * ((b - a) / n as f64) }


fn rrect(a: f64, b: f64, n: u64) -> f64 {
    let mut s: f64 = 0.0;
    for i in 1..=n {
        s += f(a + i as f64 * (b - a) / (n as f64)) }
    s * ((b - a) / n as f64) }


fn mrect(a: f64, b: f64, n: u64) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += f(a + (b - a) * (2.0 * i as f64 + 1.0) / (2.0 * n as f64)) }
    s * ((b - a) / n as f64) }


fn trapezoid(a: f64, b: f64, n: u64) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=n - 1 {
        s += (f(a + i as f64 * (b - a) / n as f64) + f(a + (i + 1) as f64 * (b - a) / n as f64)) / 2.0 }
    s * ((b - a) / n as f64) }


fn main() {
    let mut eps = String::new();

    stdin().read_line(&mut eps).unwrap();
    let eps: f64 = eps.trim().parse().unwrap();
    let (a, b) = (0.0001, PI);

    let (mut n, mut s1, mut s2): (u64, f64, f64) = (1, 0.0, f(a) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, lrect(a, b, n)); }

    println!("\nl = {s2}");

    (s1, s2) = (0.0, f(a) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, rrect(a, b, n)); }

    println!("r = {s2}");

    (s1, s2) = (0.0, f((a + b) / 2.0) * (b - a));

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, mrect(a, b, n)); }

    println!("m = {s2}");

    (s1, s2) = (0.0, (b - a) * (f(a) + f(b)) / 2.0);

    while (s2 - s1).abs() > eps {
        n *= 2;
        (s1, s2) = (s2, trapezoid(a, b, n)); }

    print!("t = {s2}"); }
