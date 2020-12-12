use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    a("input/ten-test.txt")?;
    b("input/ten-test.txt")?;

    a("input/ten-test2.txt")?;
    b("input/ten-test2.txt")?;

    a("input/ten.txt")?;
    b("input/ten.txt")?;

    Ok(())
}

fn a(path: &str) -> Result<()> {
    let mut is = read(path)?;

    is.sort();

    let mut diffs = HashMap::new();
    diffs.insert(3, 1);

    let mut prev = 0;
    for i in is {
        let a = diffs.entry(i - prev).or_insert(0);
        *a += 1;
        prev = i;
    }

    debug!("Diffs: {:?}", diffs);

    println!(
        "A Result: {} * {} => {}",
        diffs[&1],
        diffs[&3],
        diffs[&1] * diffs[&3]
    );

    Ok(())
}

fn combine(start: i64, data: &[i64], end: i64) -> u64 {
    debug!("Checking {} - {:?} - {}", start, data, end);

    if data.len() >= 1 {
        if data[0] - start > 3 {
            0
        } else {
            // Take the number
            combine(data[0], &data[1..], end) + combine(start, &data[1..], end)
        }
    } else {
        if end - start > 3 {
            0
        } else {
            1
        }
    }
}

fn combinations(input: Vec<i64>) -> u64 {
    if input.len() < 3 {
        return 1;
    }

    let start = input[0];

    combine(start, &input[1..input.len() - 1], input[input.len() - 1])
}

fn b(path: &str) -> Result<()> {
    let mut is = read(path)?;

    is.sort();

    let mut groups = vec![];
    let mut group = vec![0];
    let mut prev = 0;
    for i in is {
        if i - prev == 3 {
            groups.push(group);
            group = vec![i];
        } else {
            group.push(i);
        }
        prev = i;
    }

    if group.len() > 0 {
        groups.push(group);
    }

    debug!("Groups: {:?}", groups);

    let mut total = 1;
    for group in groups {
        total *= combinations(group);
    }

    println!("B Result: {}", total);

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
