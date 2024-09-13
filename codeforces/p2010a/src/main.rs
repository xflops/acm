use std::io;

fn main() -> io::Result<()>{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n: i32 = line.trim().parse().expect("invalid number");
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        // Ignore the array len.
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let s: i32 = line
            .trim()
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| {
                if i % 2 == 0 {
                    s.parse().unwrap()
                } else {
                    -(s.parse::<i32>().unwrap())
                }
            })
            .sum();

        println!("{}", s);
    }
    Ok(())
}
