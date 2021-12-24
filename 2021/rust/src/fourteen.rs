#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    test("src/fourteen-test.txt");
    test("src/fourteen.txt");
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let mut lines = data.lines();

    let start: Vec<char> = lines.next().unwrap().chars().collect();
    let _ = lines.next().unwrap();
    let instructions: HashMap<Vec<char>, char> = lines
        .map(|x| {
            let instruction: Vec<&str> = x.split(" -> ").collect();
            (
                instruction[0].chars().collect(),
                instruction[1].chars().next().unwrap(),
            )
        })
        .collect();

    // println!("Start: {:?}", start);
    let data = start;

    let (most, least) = step_fast(&data, &instructions, 10);

    println!("A: {}", most - least);
    //panic!();

    let (most, least) = step_fast(&data, &instructions, 40);

    println!("B: {}", most - least);
}

fn step_fast(data: &[char], instructions: &HashMap<Vec<char>, char>, steps: usize) -> (u64, u64) {
    let mut pairs: HashMap<Vec<char>, u64> = HashMap::new();

    for x in 0..(data.len() - 1) {
        let v = pairs.entry(vec![data[x], data[x + 1]]).or_insert(0);
        *v += 1;
    }

    // println!("{:?}", pairs);

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for (pair, count) in pairs {
            let insert = instructions[&pair];
            *new_pairs.entry(vec![pair[0], insert]).or_insert(0) += count;
            *new_pairs.entry(vec![insert, pair[1]]).or_insert(0) += count;
        }
        pairs = new_pairs;
        // println!("{:?}", pairs);
    }

    let mut ccount: HashMap<char, u64> = HashMap::new();

    for (pair, count) in pairs {
        *ccount.entry(pair[0]).or_insert(0) += count;
        *ccount.entry(pair[1]).or_insert(0) += count;
    }

    *ccount.entry(data[0]).or_insert(0) += 1;
    *ccount.entry(data[data.len() - 1]).or_insert(0) += 1;

    for v in ccount.values_mut() {
        *v /= 2;
    }

    //println!("{:?}", ccount);

    count(&ccount)
}

fn step_slow(
    data: &mut Vec<char>,
    instructions: &HashMap<Vec<char>, char>,
    steps: usize,
) -> (u64, u64) {
    for _ in 0..steps {
        let mut new_data = vec![];
        for x in 0..(data.len() - 1) {
            new_data.push(data[x]);

            let pair = vec![data[x], data[x + 1]];

            if let Some(insert) = instructions.get(&pair) {
                new_data.push(*insert);
            } else {
                panic!("Unknown pair: {:?}", pair);
            }
        }

        new_data.push(data[data.len() - 1]);
        *data = new_data;
    }

    let mut ccount: HashMap<char, u64> = HashMap::new();

    for c in data {
        let v = ccount.entry(*c).or_insert(0);
        *v += 1;
    }

    count(&ccount)
}

fn count(ccount: &HashMap<char, u64>) -> (u64, u64) {
    let mut most = None;
    let mut least = None;

    for val in ccount.values() {
        if let Some(cur) = most {
            if val > cur {
                most = Some(val);
            }
        } else {
            most = Some(val);
        }

        if let Some(cur) = least {
            if val < cur {
                least = Some(val);
            }
        } else {
            least = Some(val);
        }
    }

    // println!("Most: {}, Least: {}", most.unwrap(), least.unwrap());
    (*most.unwrap(), *least.unwrap())
}
