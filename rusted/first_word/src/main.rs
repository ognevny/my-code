use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    print!("{}", input.find(' ').map_or_else(|| input.trim(), |n| input[..n].trim()));
    Ok(())
}
