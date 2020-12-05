use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    a("input/five-test.txt")?;
    b("input/five-test.txt")?;

    a("input/five.txt")?;
    b("input/five.txt")?;

    Ok(())
}

struct Seat {
    row: u16,
    col: u16,
}

impl Seat {
    fn seat_id(&self) -> u16 {
        (self.row * 8) + self.col
    }

    fn decode(input: String) -> Seat {
        let mut row_min = 0;
        let mut row_max = 128;
        let mut col_min = 0;
        let mut col_max = 8;

        for c in input.chars() {
            match c {
                'F' => {
                    row_max = row_max - ((row_max - row_min) / 2);
                }
                'B' => {
                    row_min = row_min + ((row_max - row_min) / 2);
                }
                'L' => {
                    col_max = col_max - ((col_max - col_min) / 2);
                }
                'R' => {
                    col_min = col_min + ((col_max - col_min) / 2);
                }
                c => panic!("Unexpected character: {}", c),
            }
        }

        assert_eq!(row_max - row_min, 1);
        assert_eq!(col_max - col_min, 1);

        Seat {
            row: row_min,
            col: col_min,
        }
    }
}

fn a(path: &str) -> Result<()> {
    let seats = read(path)?;

    let max = seats
        .into_iter()
        .fold(0, |c, s| if s.seat_id() > c { s.seat_id() } else { c });

    println!("Result: {}", max);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let seats = read(path)?;

    let seats: HashSet<u16> = seats.iter().map(|s| s.seat_id()).collect();

    for seat in &seats {
        if (!seats.contains(&(seat + 1))) && (seats.contains(&(seat + 2))) {
            println!("Result: {}", seat + 1);
        }
    }

    Ok(())
}

fn read(path: &str) -> Result<Vec<Seat>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut seats = vec![];

    for line in reader.lines() {
        seats.push(Seat::decode(line?));
    }

    Ok(seats)
}
