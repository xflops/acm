use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let s: i32 = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .sum();

    println!("{}", 6 - s);
    Ok(())
}
