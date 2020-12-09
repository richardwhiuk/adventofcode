use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    a("input/nine-test.txt", 5)?;
    b("input/nine-test.txt", 5)?;

    a("input/nine.txt", 25)?;
    b("input/nine.txt", 25)?;

    Ok(())
}

fn a(path: &str, preamble: usize) -> Result<()> {
    let is = read(path)?;

    let s = search(&is, path, preamble);
    println!("Result {}", s);

    Ok(())
}

fn search(is: &Vec<i64>, path: &str, preamble: usize) -> i64 {
    let mut start = 0;

    for i in &is[preamble..] {
        let mut available = HashSet::new();
        let mut found = false;
        for j in &is[start..start + preamble] {
            if available.contains(&(i - j)) {
                debug!("{} = {} + {}", i, j, (i - j));
                found = true;
                break;
            }
            available.insert(j);
        }

        if !found {
            return *i;
        }

        start += 1;
    }

    0
}

fn b(path: &str, preamble: usize) -> Result<()> {
    let is = read(path)?;

    let search = search(&is, path, preamble);

    let mut start = 0;

    for i in 1..is.len() {
        let mut cont = true;
        while cont {
            let sum: i64 = is[start..i].iter().sum();
            if sum == search {
                println!("{} to {} = {}", start, i, sum);
                let min: i64 = *is[start..i].iter().min().unwrap();
                let max: i64 = *is[start..i].iter().max().unwrap();

                println!("Result: {}", min + max);

                return Ok(());
            } else if sum > search {
                start += 1;
            } else {
                cont = false;
            }
        }
    }

    Ok(())
}

fn read(path: &str) -> Result<Vec<i64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = vec![];

    for line in reader.lines() {
        let line = line?;
        res.push(line.parse()?);
    }

    Ok(res)
}
