type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    a("input/three-test.txt")?;
    b("input/three-test.txt")?;

    a("input/three.txt")?;
    b("input/three.txt")?;

    Ok(())
}

type Map = Vec<Vec<bool>>;

fn read(path: &str) -> Result<Map> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res: Map = vec![];

    for line in reader.lines() {
        let mut row = vec![];
        for c in line?.chars() {
            if c == '#' {
                row.push(true);
            } else if c == '.' {
                row.push(false);
            }
        }
        if row.len() > 0 {
            res.push(row);
        }
    }

    Ok(res)
}

fn a(path: &str) -> Result<()> {
    let map = read(path)?;

    let found = find(&map, (3, 1))?;

    println!("Result: {}", found);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let map = read(path)?;

    let mut answer: u64 = 1;

    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let count = find(&map, *slope)?;
        println!("Slope: {:?} Answer: {}", slope, count);
        answer = answer * count;
    }

    println!("Result: {}", answer);

    Ok(())
}

fn find(map: &Map, slope: (usize, usize)) -> Result<u64> {
    let mut found = 0;
    let mut x = 0;
    let mut y_target = 0;

    for (y, row) in map.iter().enumerate() {
        if y_target == y {
            if row[x] {
                found += 1;
            }

            x += slope.0;
            y_target += slope.1;
            x = x % row.len();
        }
    }

    Ok(found)
}
