use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    print!("{}", input.find(' ').map_or_else(|| input.trim(), |n| input[..n].trim()));
    Ok(())
}
