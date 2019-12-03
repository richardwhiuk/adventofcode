use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn run_a() {
    let lines : Vec<_> = BufReader::new(File::open("3.txt").expect("Unable to open file"))
            .lines().map(Result::unwrap).collect();

    println!("3a: Distance: {}", distance(&lines[0], &lines[1]));
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
    WireA,
    WireB,
    Intersection,
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
            Entry::Occupied(mut o) => match o.get() {
                State::WireA => {
                    if state == State::WireB {
                        o.insert(State::Intersection);
                        return State::Intersection;
                    } else {
                        //panic!("Wire backtracking");
                        return state;
                    }
                }
                State::WireB => {
                    if state == State::WireA {
                        o.insert(State::Intersection);
                        return State::Intersection;
                    } else {
                        //panic!("Wire backtracking");
                        return state;
                    }
                }
                old => {
                    panic!("Trying to insert {:?} where {:?}", state, old);
                }
            },
        }
    }
}

pub fn distance(a: &str, b: &str) -> i32 {
    let mut board = Board::new();

    let mut nearest = None;

    for (state, wire) in vec![(State::WireA, a), (State::WireB, b)] {
        let mut x = 0;
        let mut y = 0;

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

            match board.set(x, y, state) {
                State::Intersection => {
                    let m = x.abs() + y.abs();
                    nearest = match nearest {
                        None => Some(m),
                        Some(old) => Some(std::cmp::min(m, old)),
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
    pub fn example() {
        assert_eq!(distance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
    }
    #[test]
    pub fn a() {
        assert_eq!(
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }
    #[test]
    pub fn b() {
        assert_eq!(
            distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
