use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut maxstr, mut input) = ("", String::new());
    io::stdin().read_line(&mut input)?;
    for word in input.split_whitespace() {
        maxstr = if word.len() > maxstr.len() { word } else { maxstr };
    }
    print!("{maxstr}");
    Ok(())
}
