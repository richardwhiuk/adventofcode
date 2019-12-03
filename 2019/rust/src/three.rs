use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn run_a() {
    let lines: Vec<_> = BufReader::new(File::open("3.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .collect();

    println!("3a: Distance: {}", distance(&lines[0], &lines[1]));
}

pub fn run_b() {
    let lines: Vec<_> = BufReader::new(File::open("3.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .collect();

    println!("3b: Steps: {}", steps(&lines[0], &lines[1]));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    dir: Direction,
    distance: i32,
}

impl FromStr for Movement {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let dir = match &input[0..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => {
                panic!("Unexpected direction: {}", input);
            }
        };

        let distance = input[1..].parse()?;

        Ok(Self { dir, distance })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    CentralPort,
    WireA(i32),
    WireB(i32),
    Intersection(i32),
}

struct Board(HashMap<i32, HashMap<i32, State>>);

impl Board {
    fn new() -> Board {
        let mut s = Self(HashMap::new());
        s.set(0, 0, State::CentralPort);
        s
    }

    fn set(&mut self, x: i32, y: i32, state: State) -> State {
        let col = self.0.entry(y).or_insert(HashMap::new()).entry(x);

        match col {
            Entry::Vacant(v) => {
                v.insert(state);
                return state;
            }
            Entry::Occupied(mut o) => match o.get().clone() {
                State::WireA(a) => {
                    if let State::WireB(b) = state {
                        o.insert(State::Intersection(a + b));
                        return State::Intersection(a + b);
                    } else {
                        //panic!("Wire backtracking");
                        return State::WireA(a);
                    }
                }
                State::WireB(b) => {
                    if let State::WireA(a) = state {
                        o.insert(State::Intersection(a + b));
                        return State::Intersection(a + b);
                    } else {
                        //panic!("Wire backtracking");
                        return State::WireB(b);
                    }
                }
                old => {
                    panic!("Trying to insert {:?} where {:?}", state, old);
                }
            },
        }
    }
}

fn distance(a: &str, b: &str) -> i32 {
    measure(a, b, |x, y, _int| x.abs() + y.abs())
}

fn steps(a: &str, b: &str) -> i32 {
    measure(a, b, |_x, _y, int| int)
}

fn measure<F>(a: &str, b: &str, cmp: F) -> i32
where
    F: Fn(i32, i32, i32) -> i32,
{
    let mut board = Board::new();

    let mut nearest = None;

    let wires: Vec<(Box<dyn Fn(i32) -> State>, _)> = vec![
        (Box::new(|x| State::WireA(x)), a),
        (Box::new(|x| State::WireB(x)), b),
    ];

    for (state, wire) in wires {
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;

        for movement in wire.split(',').map(Movement::from_str).map(Result::unwrap) {
            for _ in 0..movement.distance {
                match movement.dir {
                    Direction::Left => {
                        x -= 1;
                    }
                    Direction::Right => {
                        x += 1;
                    }
                    Direction::Up => {
                        y -= 1;
                    }
                    Direction::Down => {
                        y += 1;
                    }
                }
                steps += 1;

                match board.set(x, y, state(steps)) {
                    State::Intersection(int) => {
                        let dist = cmp(x, y, int);
                        nearest = match nearest {
                            None => Some(dist),
                            Some(old) => Some(std::cmp::min(dist, old)),
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    return nearest.expect("No intersection");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_example() {
        assert_eq!(distance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
    }
    #[test]
    fn a_a() {
        assert_eq!(
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }
    #[test]
    fn a_b() {
        assert_eq!(
            distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
    #[test]
    fn b_example() {
        assert_eq!(steps("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
    }
    #[test]
    fn b_a() {
        assert_eq!(
            steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
    }
    #[test]
    fn b_b() {
        assert_eq!(
            steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
