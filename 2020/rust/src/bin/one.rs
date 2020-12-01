type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // 1A
    a("input/one-test.txt")?;
    a("input/one.txt")?;

    // 1B
    b("input/one-test.txt")?;
    b("input/one.txt")?;

    Ok(())
}

fn read(path: &str) -> Result<Vec<u32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut num: Vec<u32> = vec![];

    for line in reader.lines() {
        num.push(line?.parse()?);
    }

    Ok(num)
}

fn a(path: &str) -> Result<()> {
    let num = read(path)?;

    for x in &num {
        for y in &num {
            if x + y == 2020 {
                println!("Result: {}", x * y);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let num = read(path)?;

    for x in &num {
        for y in &num {
            if x + y < 2020 {
                for z in &num {
                    if x + y + z == 2020 {
                        println!("B Result: {}", x * y * z);
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}
