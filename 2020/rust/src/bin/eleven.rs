pub use rust::*;

fn main() -> Result<()> {
    // 1A
    a("input/eleven-test.txt")?;
    a("input/eleven.txt")?;

    // 1B
    b("input/eleven-test.txt")?;
    b("input/eleven.txt")?;

    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spot {
    Floor,
    Empty,
    Taken,
}

use Spot::*;

impl Spot {
    fn from(c: char) -> Spot {
        match c {
            '.' => Floor,
            'L' => Empty,
            '#' => Taken,
            c => {
                panic!("Unknown floor type: {}", c);
            }
        }
    }
    fn to_char(&self) -> char {
        match self {
            Floor => '.',
            Empty => 'L',
            Taken => '#',
        }
    }

    fn basic_update(&self, row: usize, col: usize, map: &Map) -> Spot {
        let mut count = 0;
        for r in &[-1, 0, 1] {
            for c in &[-1, 0, 1] {
                if *r != 0 || *c != 0 {
                    count += occupied(map, (row as isize) + r, (col as isize) + c).as_val();
                }
            }
        }
        match self {
            Empty if count == 0 => Taken,
            Taken if count >= 4 => Empty,
            s => *s,
        }
    }

    fn complex_update(&self, row: usize, col: usize, map: &Map) -> Spot {
        let mut count = 0;
        for r in &[-1, 0, 1] {
            for c in &[-1, 0, 1] {
                if *r != 0 || *c != 0 {
                    let mut candidate = (row as isize, col as isize);
                    let mut x = Exist(Floor);
                    while x == Exist(Floor) {
                        candidate = (candidate.0 + r, candidate.1 + c);
                        x = occupied(map, candidate.0, candidate.1);
                    }
                    count += x.as_val();
                }
            }
        }
        match self {
            Empty if count == 0 => Taken,
            Taken if count >= 5 => Empty,
            s => *s,
        }
    }
}

#[derive(PartialEq)]
enum Occupied {
    NotExist,
    Exist(Spot),
}

use Occupied::*;

impl Occupied {
    fn as_val(&self) -> usize {
        match self {
            Exist(Taken) => 1,
            _ => 0,
        }
    }
}

fn occupied(map: &Map, row: isize, col: isize) -> Occupied {
    if row < 0 {
        NotExist
    } else if col < 0 {
        NotExist
    } else {
        let row = row as usize;
        let col = col as usize;
        match map.get(row).and_then(|r| r.get(col)) {
            Some(s) => Exist(*s),
            None => NotExist,
        }
    }
}

type Map = Vec<Vec<Spot>>;

fn read(path: &str) -> Result<Map> {
    let mut map: Map = vec![];

    reader(path, |line| {
        map.push(line.chars().map(Spot::from).collect());
        Ok(())
    })?;

    Ok(map)
}

fn print(map: &Map) {
    for line in map {
        let s: String = line.iter().map(Spot::to_char).collect();
        debug!("{}", s);
    }
    debug!("");
}

fn a(path: &str) -> Result<()> {
    converge(path, "A", Spot::basic_update)
}

fn b(path: &str) -> Result<()> {
    converge(path, "B", Spot::complex_update)
}

fn converge<F>(path: &str, section: &str, f: F) -> Result<()>
where
    F: Fn(&Spot, usize, usize, &Map) -> Spot,
{
    let mut map = read(path)?;

    print(&map);

    let mut changes = true;
    while changes {
        changes = false;

        let mut new_map = vec![];

        for (row, seats) in map.iter().enumerate() {
            let mut new_row = vec![];
            for (col, seat) in seats.iter().enumerate() {
                let new_seat = f(seat, row, col, &map);
                new_row.push(new_seat);
                if new_seat != *seat {
                    changes = true;
                }
            }
            new_map.push(new_row);
        }

        print(&new_map);

        map = new_map;
    }

    let result = map.iter().fold(0, |count, row| {
        count
            + row
                .iter()
                .fold(0, |count, s| count + if *s == Taken { 1 } else { 0 })
    });

    println!("{} Result: {}", section, result);

    Ok(())
}
