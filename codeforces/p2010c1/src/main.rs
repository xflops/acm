use std::io;

fn main() -> io::Result<()> {
    loop {
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        match get_message(line.trim().chars().collect()) {
            Some(msg) => println!("YES\n{}", msg),
            None => println!("NO"),
        }
    }

    Ok(())
}

fn get_message(msg: Vec<char>) -> Option<String> {
    let c = msg[0];
    let len = msg.len();

    let mut p2 = 0;

    loop {
        let mut res = Vec::<char>::new();

        let idx = msg[p2 + 1..].iter().position(|r| *r == c);
        if idx.is_none() {
            break;
        }

        p2 = idx.unwrap() + p2 + 1;

        let mut c1 = 0;
        let mut c2 = p2;

        while c2 < len {
            if msg[c1] != msg[c2] {
                break;
            }
            res.push(msg[c1]);
            c1 = c1 + 1;
            c2 = c2 + 1;
        }

        if c2 == len {
            if res.len() * 2 > len {
                return Some(res.into_iter().collect());
            }
        }
    }

    None
}
