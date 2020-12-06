use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Group = HashSet<char>;

fn main() -> Result<()> {
    a("input/six-test.txt")?;
    b("input/six-test.txt")?;

    a("input/six.txt")?;
    b("input/six.txt")?;

    Ok(())
}

fn a(path: &str) -> Result<()> {
    let groups = read(path, |group, chars| {
        group.extend(chars);
    })?;

    let count = groups.iter().fold(0, |c, g| c + g.len());

    println!("Result: {}", count);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let groups = read(path, |group, chars| {
        let new: Group = chars.collect();
        let int: Group = group.intersection(&new).map(|c| *c).collect();
        std::mem::replace(group, int);
    })?;

    //println!("Groups: {:?}", groups);

    let count = groups.iter().fold(0, |c, g| c + g.len());

    println!("Result: {}", count);

    Ok(())
}

fn read<F>(path: &str, combine: F) -> Result<Vec<Group>>
where
    F: Fn(&mut Group, std::str::Chars),
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut groups = vec![];
    let mut group: Option<Group> = None;

    for line in reader.lines() {
        let line = line?;

        if line == "" {
            if let Some(g) = group.take() {
                groups.push(g);
            }
        } else {
            if let Some(ref mut group) = group {
                combine(group, line.chars());
            } else {
                group = Some(line.chars().collect());
            }
        }
    }

    if let Some(g) = group.take() {
        groups.push(g);
    }

    Ok(groups)
}
