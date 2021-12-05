use std::fs::read_to_string;

fn main() {
    test("src/two.txt");
    test("src/two-test.txt");
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Movement {
    dir: Direction,
    units: i64,
}

impl Movement {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(' ');
        use Direction::*;
        Movement {
            dir: match parts.next().unwrap() {
                "forward" => Forward,
                "down" => Down,
                "up" => Up,
                dir => panic!("Unexpected direction: {}", dir),
            },
            units: parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn test(file: &str) {
    let file = read_to_string(file).expect("failed to load");
    let a: (i64, i64) = file.lines().map(Movement::parse).fold((0, 0), |init, m| {
        use Direction::*;
        match m.dir {
            Forward => (init.0, init.1 + m.units),
            Down => (init.0 - m.units, init.1),
            Up => (init.0 + m.units, init.1),
        }
    });
    println!("A: {}", a.0 * a.1);

    let mut position = (0, 0);
    let mut aim = 0;
    for movement in file.lines().map(Movement::parse) {
        use Direction::*;
        match movement.dir {
            Down => aim += movement.units,
            Up => aim -= movement.units,
            Forward => {
                position = (
                    position.0 + movement.units,
                    position.1 + (aim * movement.units),
                )
            }
        }
    }

    println!("B: {}", position.0 * position.1);
}
