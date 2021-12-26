#![allow(dead_code)]
#![allow(clippy::unusual_byte_groupings)]
use std::fs::read_to_string;

fn main() {
    test("src/twenty-test.txt");
    test("src/twenty.txt");
}

fn print(image: &[Vec<bool>]) {
    for row in image {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn step(input: &mut Vec<Vec<bool>>, background: &mut bool, mapper: &[bool]) {
    let mut output = vec![];

    // println!("Background: {:?}", background);

    for y in -1..(input.len() as i32 + 1) {
        let mut nrow: Vec<bool> = vec![];
        for x in -1..(input[0].len() as i32 + 1) {
            let mut values = vec![];
            for yd in &[-1, 0, 1] {
                let ny = y + yd;
                if ny < 0 {
                    values.push(*background);
                    values.push(*background);
                    values.push(*background);
                } else {
                    let ny = ny as usize;
                    for xd in &[-1, 0, 1] {
                        let nx = x + xd;
                        if nx < 0 {
                            values.push(*background);
                        } else {
                            let nx = nx as usize;
                            values.push(
                                input
                                    .get(ny)
                                    .and_then(|row| row.get(nx))
                                    .cloned()
                                    .unwrap_or(*background),
                            );
                        }
                    }
                }
            }
            let value: String = values
                .into_iter()
                .map(|x| if x { '1' } else { '0' })
                .collect();
            let value: usize = usize::from_str_radix(&value, 2).unwrap();
            nrow.push(mapper[value]);
        }
        output.push(nrow);
    }
    *input = output;
    if *background {
        *background = mapper[0b111_111_111];
    } else {
        *background = mapper[0b000_000_000];
    }

    // println!("After step: {}", step);
    // print(&input);
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let mut data = data.lines();
    let mapper: Vec<bool> = data.next().unwrap().chars().map(|x| x == '#').collect();
    let _ = data.next();
    let mut input: Vec<Vec<bool>> = data
        .map(|x| x.chars().map(|x| x == '#').collect())
        .collect();

    // println!("Initial");
    // print(&input);

    let mut background = false;

    for sn in 1..=2 {
        println!("Step: {}", sn);
        step(&mut input, &mut background, &mapper);
    }

    let result: usize = input.iter().map(|x| x.iter().filter(|x| **x).count()).sum();

    println!("A: {}", result);

    for sn in 3..=50 {
        println!("Step: {}", sn);
        step(&mut input, &mut background, &mapper);
    }

    let result: usize = input.iter().map(|x| x.iter().filter(|x| **x).count()).sum();

    println!("B: {}", result);
}
