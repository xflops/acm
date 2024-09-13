use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let n: i32 = line.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        let bs = io::stdin().read_line(&mut line)?;
        if bs == 0 {
            break;
        }

        let data: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        println!("{}", data[1] - data[0]);
    }

    Ok(())
}
