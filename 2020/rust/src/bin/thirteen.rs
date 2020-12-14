pub use rust::*;

fn main() -> Result<()> {
    // 1A
    a("input/thirteen-test.txt")?;
    a("input/thirteen.txt")?;

    // 1B
    b("input/thirteen-test.txt")?;
    b("input/thirteen.txt")?;

    Ok(())
}

struct BusInfo {
    timestamp: i64,
    ids: Vec<i64>,
}

fn next(timestamp: i64, bus: i64) -> i64 {
    let next = (((timestamp - 1) / bus) + 1) * bus;
    next - timestamp
}

fn a(path: &str) -> Result<()> {
    let mut lines = read_lines(path)?;

    let timestamp = lines.next().unwrap()?.parse()?;

    let ids: Vec<i64> = lines
        .next()
        .unwrap()?
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

    let bi = BusInfo { timestamp, ids };

    let mut earliest: Option<(i64, i64)> = None;

    for bus in bi.ids {
        let next = next(bi.timestamp, bus);
        let next_arrival = (next, next * bus);

        if let Some(current) = earliest {
            if next_arrival.0 < current.0 {
                earliest = Some(next_arrival);
            } else {
                earliest = Some(current);
            }
        } else {
            earliest = Some(next_arrival);
        }
    }

    println!("A Result: {:?}", earliest);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let mut lines = read_lines(path)?;

    let _ = lines.next();

    let mut ids: Vec<(i64, i64)> = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|s| s.parse().ok())
        .enumerate()
        .filter_map(|(a, b)| b.map(|b| (a as i64, b)))
        .collect();

    ids.sort_by_key(|(_, b)| -b);

    let mut guess = ids[0].1 - ids[0].0;
    let mut add = ids[0].1;

    for (a, b) in &ids[1..] {
        println!(" Bus {} leaves at {} minutes past", b, a);
        println!("  Current guess: {}, (add: {})", guess, add);

        // Work out remainder
        let mut t = *a;
        while t > 0 {
            t -= b;
        }
        let rem = -t;

        println!("  Fixing {} mod {}", rem, b);
        while (guess % b) != rem {
            guess += add;
        }
        add *= b;
    }

    for id in &ids {
        let check = next(guess, id.1);
        println!("{}: {} => {} - {:?}", id.0, id.1, check, check == id.0);
    }

    println!("B Result: {}", guess);

    Ok(())
}
