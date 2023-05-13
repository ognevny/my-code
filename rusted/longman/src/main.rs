use std::io::stdin;

fn main() {
    let (mut maxstr, mut input) = ("", String::new());
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .for_each(|x| maxstr = if x.len() > maxstr.len() { x } else { maxstr });
    print!("{maxstr}");
}
