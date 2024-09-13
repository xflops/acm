use std::io;

fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    return Ok(String::from(line.trim()))
}

fn read_i32() -> io::Result<i32> {
    let line = read_line()?;
    return Ok(line.parse::<i32>().unwrap());
}

fn main() -> io::Result<()> {
    let n = read_i32()?;
    for _ in 0..n {
        let ln = read_i32()?;
        let mut res = vec![];
        for _ in 0..ln {
            let line = read_line()?;
            res.push(line.chars().position(|c| c == '#').unwrap() + 1);
        }

        res.iter().rev().for_each(|c| print!("{} ", c));
        println!();
    }

    Ok(())
}
