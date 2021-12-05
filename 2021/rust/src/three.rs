use std::fs::read_to_string;

fn main() {
    test("src/three.txt");
    test("src/three-test.txt");
}

fn analyze(file: &str, less: bool) -> u64 {
    let mut i = 0;

    let mut numbers: Vec<&str> = file.lines().collect();
    loop {
        let mut zero = 0;
        let mut one = 0;
        for line in &numbers {
            if line.chars().nth(i).unwrap() == '0' {
                zero += 1;
            } else {
                one += 1;
            }
        }

        let req = if (zero > one) ^ less { '0' } else { '1' };

        numbers = numbers
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == req)
            .collect();

        if numbers.len() == 1 {
            break;
        }
        i += 1;
    }

    numbers[0]
        .chars()
        .fold(0, |init, v| (2 * init) + (if v == '0' { 0 } else { 1 }))
}

fn test(file: &str) {
    let file = read_to_string(file).expect("failed to load");

    let mut vs: Vec<Vec<bool>> = vec![];

    for line in file.lines() {
        for (i, c) in line.chars().enumerate() {
            if vs.len() <= i {
                vs.push(vec![]);
            }
            if c == '0' {
                vs[i].push(false);
            } else {
                vs[i].push(true);
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for v in vs {
        gamma *= 2;
        epsilon *= 2;
        let mut zero = 0;
        let mut one = 0;
        for c in v {
            if c {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one > zero {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("A: {}", gamma * epsilon);

    let oxygen = analyze(&file, false);
    println!("Oxygen Generator Rating: {}", oxygen);
    let co2 = analyze(&file, true);
    println!("CO2 Rating: {}", co2);
    println!("B: {}", co2 * oxygen);
}
