use log::debug;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    a("input/seven-test.txt")?;
    b("input/seven-test.txt")?;

    a("input/seven.txt")?;
    b("input/seven.txt")?;

    Ok(())
}

type Bags = HashMap<String, Vec<(u16, String)>>;

fn can_contain<'a>(needle: &str, key: &'a str, bags: &Bags, mut used: HashSet<&'a str>) -> bool {
    debug!(
        "Checking for {} in bags with key {} having used {:?}",
        needle, key, used
    );

    if needle != key {
        used.insert(key);

        if let Some(available_bags) = bags.get(key) {
            for (_, bag) in available_bags {
                if bag == needle {
                    return true;
                } else if !used.contains(bag.as_str()) {
                    if can_contain(needle, bag, &bags, used.clone()) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn a(path: &str) -> Result<()> {
    let bags = read(path)?;

    let mut count = 0;

    for bag in bags.keys() {
        let used = HashSet::new();
        if can_contain("shiny gold", bag, &bags, used) {
            count += 1;
        }
    }

    println!("Result: {}", count);

    Ok(())
}

fn total_bags(needle: &str, bags: &Bags) -> u64 {
    let mut acc = 1;

    if let Some(available_bags) = bags.get(needle) {
        for (count, new_bag) in available_bags {
            acc += (*count as u64) * total_bags(new_bag, bags);
        }
    }

    acc
}

fn b(path: &str) -> Result<()> {
    let bags = read(path)?;

    let count = total_bags("shiny gold", &bags) - 1;

    println!("Result: {}", count);

    Ok(())
}

fn read(path: &str) -> Result<HashMap<String, Vec<(u16, String)>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = HashMap::new();

    let re = regex::Regex::new(r"(\d+) ([a-z ]+) bags?")?;

    for line in reader.lines() {
        let line = line?;

        let mut line = line.split(" bags contain ");
        let key = line.next().expect("No colour");
        let value = line.next().expect("No bags").trim_end_matches('.');
        let mut resb = vec![];
        if value != "no other bags" {
            let value = value.split(", ");
            for value in value {
                let capture = re
                    .captures(value)
                    .unwrap_or_else(|| panic!("Invalid bags: {}", value));
                resb.push((
                    capture.get(1).expect("No count").as_str().parse()?,
                    capture.get(2).expect("Not bag colour").as_str().to_owned(),
                ));
            }
        }
        res.insert(key.to_owned(), resb);
    }

    Ok(res)
}
