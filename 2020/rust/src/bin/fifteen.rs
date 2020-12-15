pub use rust::*;

fn main() -> Result<()> {
    a("input/fifteen-test.txt")?;
    a("input/fifteen.txt")?;

    b("input/fifteen-test.txt")?;
    b("input/fifteen.txt")?;

    Ok(())
}

fn read(path: &str) -> Result<Vec<Vec<i32>>> {
    let mut res = vec![];

    reader(path, |line| {
        res.push(line.split(',').map(|u| u.parse().unwrap()).collect());

        Ok(())
    })?;

    Ok(res)
}

enum Spoken {
    Once(i32),
    Twice(i32, i32),
}

use Spoken::*;

fn update(lookup: &mut HashMap<i32, Spoken>, i: i32, number: i32) {
    lookup.insert(
        number,
        match lookup.get(&number) {
            None => Once(i),
            Some(Once(h)) => Twice(*h, i),
            Some(Twice(_, h)) => Twice(*h, i),
        },
    );
}

fn a(path: &str) -> Result<()> {
    puzzle(path, 2020, "A")
}

fn puzzle(path: &str, end: i32, section: &str) -> Result<()> {
    let numbers = read(path)?;

    for numbers in numbers {
        println!("Evaluating: {:?}", numbers);

        let mut lookup: HashMap<i32, Spoken> = HashMap::new();
        let mut c = 0;
        for (i, number) in numbers.iter().enumerate() {
            update(&mut lookup, i as i32, *number);
            c = *number;
        }

        for i in (numbers.len() as i32)..end {
            c = match lookup.get(&c) {
                None => 0,
                Some(Once(_)) => 0,
                Some(Twice(h, i)) => i - h,
            };

            debug!("{} is {}", i, c);

            update(&mut lookup, i, c);
        }

        println!("{} Result: {}", section, c);
    }

    Ok(())
}

fn b(path: &str) -> Result<()> {
    puzzle(path, 30000000, "B")
}
