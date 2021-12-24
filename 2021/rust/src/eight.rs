#![allow(non_upper_case_globals)]

use std::fs::read_to_string;

use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref Zero : HashSet<char> = set(vec!['a', 'b', 'c', 'e', 'f', 'g']);
    static ref One : HashSet<char> = set(vec!['c', 'f']);
    static ref Two : HashSet<char> = set(vec!['a', 'c', 'd', 'e', 'g']);
    static ref Three : HashSet<char> = set(vec!['a', 'c', 'd', 'f', 'g']);
    static ref Four : HashSet<char> = set(vec!['b', 'c', 'd', 'f']);
    static ref Five : HashSet<char> = set(vec!['a', 'b', 'd', 'f', 'g']);
    static ref Six : HashSet<char> = set(vec!['a', 'b', 'd', 'e', 'f', 'g']);
    static ref Seven : HashSet<char> = set(vec!['a', 'c', 'f']);
    static ref Eight : HashSet<char> = set(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    static ref Nine : HashSet<char> = set(vec!['a', 'b', 'c', 'd', 'f', 'g']);


    static ref FiveBars : HashSet<char> = set(vec!['a', 'd', 'g']);
    static ref SixBars : HashSet<char> = set(vec!['a', 'b', 'f', 'g']);
}

fn set(v: Vec<char>) -> HashSet<char> {
    v.into_iter().collect()
}

fn main() {
    test("src/eight-test1.txt");
    test("src/eight-test.txt");
    test("src/eight.txt");
}

fn update_biject(
    valid: &HashSet<char>,
    chars: &HashSet<char>,
    mapping: &mut HashMap<char, HashSet<char>>,
) {
    for (k, v) in mapping {
        if chars.contains(k) {
            *v = v.intersection(valid).cloned().collect();
        } else {
            *v = v.difference(valid).cloned().collect();
        }
    }
}

fn update_mandatory(
    required: &HashSet<char>,
    chars: &HashSet<char>,
    mapping: &mut HashMap<char, HashSet<char>>,
) {
    for (k, v) in mapping {
        if !chars.contains(k) {
            *v = v.difference(required).cloned().collect();
        }
    }
}

fn determine_output(input: &[&str], output: &[&str]) -> u32 {
    // Pattern => Correct
    let mut mapping: HashMap<char, HashSet<char>> = HashMap::new();

    {
        let all: HashSet<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
            .into_iter()
            .collect();
        for a in all.iter() {
            mapping.insert(*a, all.clone());
        }
    }

    update_patterns(input, &mut mapping);
    update_patterns(output, &mut mapping);

    let mut found: HashSet<char> = HashSet::new();
    let mut progress = true;

    while progress {
        progress = false;
        for v in &mut mapping.values_mut() {
            let olen = v.len();
            if olen != 1 {
                *v = v.difference(&found).cloned().collect();
            }

            if v.len() != olen {
                progress = true;
            }

            if v.len() == 1 {
                let r = *v.iter().next().unwrap();
                if !found.contains(&r) {
                    found.insert(r);
                    progress = true;
                }
            }

            if found.len() == 8 {
                progress = false;
            }
        }
    }

    if !found.len() == 8 {
        panic!("Unable to determine mapping: {:?}", mapping);
    }

    let mapping: HashMap<char, char> = mapping
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect();

    let mut out = 0;

    for pattern in output {
        let correct: HashSet<char> = pattern.chars().map(|x| mapping[&x]).collect();

        let v = match &correct {
            a if *a == *Zero => 0,
            a if *a == *One => 1,
            a if *a == *Two => 2,
            a if *a == *Three => 3,
            a if *a == *Four => 4,
            a if *a == *Five => 5,
            a if *a == *Six => 6,
            a if *a == *Seven => 7,
            a if *a == *Eight => 8,
            a if *a == *Nine => 9,
            a => panic!("Unexpected pattern: {:?}", a),
        };

        //println!("{} => {:?}: {}", pattern, correct, v);

        out = (out * 10) + v;
    }

    //println!("{:?}, {:?}", input, mapping);
    //println!("{:?}: {}", output, out);

    out
}

fn update_patterns(input: &[&str], mapping: &mut HashMap<char, HashSet<char>>) {
    for pattern in input {
        let chars = pattern.chars().collect();
        //println!("Mapping: {:?}", mapping);
        //println!("Pattern: {:?}", pattern);
        match pattern.len() {
            2 => {
                // 1
                // Segments are C and F
                update_biject(&*One, &chars, mapping);
            }
            3 => {
                // 7
                update_biject(&*Seven, &chars, mapping);
            }
            4 => {
                // 4
                update_biject(&*Four, &chars, mapping);
            }
            5 => {
                // 2, 3, or 5
                update_mandatory(&*FiveBars, &chars, mapping);
            }
            6 => {
                // 0, 6, or 9
                update_mandatory(&*SixBars, &chars, mapping);
            }
            7 => {
                // 8
                // No help
            }
            _ => panic!("Unexpected segment"),
        };
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<(Vec<&str>, Vec<&str>)> = data
        .lines()
        .map(|x| {
            let data: Vec<&str> = x.trim().split('|').collect();
            let input: Vec<&str> = data[0].trim().split(' ').collect();
            let output: Vec<&str> = data[1].trim().split(' ').collect();
            (input, output)
        })
        .collect();

    let mut result = vec![0; 10];

    for line in &data {
        for pattern in &line.1 {
            let num = match pattern.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => 2, // or 3 or 5,
                6 => 0, // or 6 or 9,
                7 => 8,
                _ => panic!("Unexpected segment"),
            };
            result[num] += 1;
        }
    }

    println!("A: {}", result[1] + result[4] + result[7] + result[8]);

    let mut out = 0;

    for line in data {
        out += determine_output(&line.0, &line.1)
    }

    println!("B: {}", out);
}
