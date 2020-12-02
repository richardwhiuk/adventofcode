type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // 2A
    a("input/two-test.txt")?;
    a("input/two.txt")?;

    // 2B
    b("input/two-test.txt")?;
    b("input/two.txt")?;

    Ok(())
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn read(path: &str) -> Result<Vec<Password>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res: Vec<Password> = vec![];

    let re = regex::Regex::new(r"(\d+)\-(\d+) ([a-z]): ([a-z]+)")?;

    for line in reader.lines() {
        match re.captures(&line?) {
            Some(re) => {
                res.push(Password {
                    min: re.get(1).expect("Missing regex group").as_str().parse()?,
                    max: re.get(2).expect("Missing regex group").as_str().parse()?,
                    letter: re.get(3).expect("Missing regex group").as_str().parse()?,
                    password: re.get(4).expect("Missing regex group").as_str().parse()?,
                });
            }
            None => {}
        }
    }

    Ok(res)
}

fn a(path: &str) -> Result<()> {
    let passwords = read(path)?;

    let mut count = 0;

    for password in passwords {
        let mut copies = 0;
        for letter in password.password.chars() {
            if letter == password.letter {
                copies += 1;
            }
        }
        if copies >= password.min && copies <= password.max {
            count += 1;
        }
    }

    println!("Result: {}", count);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let passwords = read(path)?;

    let mut count = 0;

    for password in passwords {
        let chars: Vec<char> = password.password.chars().collect();
        if (chars[password.min - 1] == password.letter)
            ^ (chars[password.max - 1] == password.letter)
        {
            count += 1;
        }
    }

    println!("Result: {}", count);

    Ok(())
}
