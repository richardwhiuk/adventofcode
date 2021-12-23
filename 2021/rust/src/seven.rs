use std::fs::read_to_string;

fn main() {
    test("src/seven-test.txt");
    test("src/seven.txt");
}

fn determine_least<F>(data: &Vec<i64>, min: i64, max: i64, f: F) -> (i64, i64)
where
    F: Fn(i64, i64) -> i64,
{
    let mut least = None;

    for pos in min..=max {
        let fuel = determine_fuel(&data, pos, &f);
        least = match least {
            None => Some((pos, fuel)),
            Some((p, f)) if f < fuel => Some((p, f)),
            Some(_) => Some((pos, fuel)),
        };
    }

    least.unwrap()
}

fn determine_fuel<F>(data: &Vec<i64>, pos: i64, f: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    let mut fuel = 0;
    for d in data {
        fuel += f(pos, *d);
    }
    fuel
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<i64> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    let least = determine_least(&data, min, max, |pos, d| (pos - d).abs());
    println!("A: Pos: {}, Fuel: {}", least.0, least.1);

    let least = determine_least(&data, min, max, |pos, d| {
        let steps = (pos - d).abs();
        (steps * (steps + 1)) / 2
    });
    println!("B: Pos: {}, Fuel: {}", least.0, least.1);
}
