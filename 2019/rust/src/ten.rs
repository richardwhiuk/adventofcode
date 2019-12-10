use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    SPACE,
    ASTEROID,
    DESTROYED,
    STATION,
}

use State::*;

struct Map(Vec<Vec<State>>);

pub fn run_a() {
    println!("10a: {}", Map::parse("10.txt").find_best_station().1);
}

pub fn run_b() {
    let res = Map::parse("10.txt").destroy_asteroids()[199];
    println!("10b: {}", (res.0 * 100) + res.1);
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector(i32, i32);

// Return degrees from top in fractions of PI. 0->2
impl Vector {
    fn degs(self) -> f64 {
        let x = self.0;
        let y = -self.1;
        let mut res = ((x as f64) / (y as f64)).atan() / std::f64::consts::PI;

        if y < 0 {
            res += 1.0;
        } else if x < 0 {
            res += 2.0;
        }

        res
    }

    fn before(self, compare: Vector) -> bool {
        self.degs() < compare.degs()
    }

    fn between(self, a: Vector, b: Vector) -> bool {
        let us = self.degs();
        let a = a.degs();
        let b = b.degs();

        if b > a {
            (b > us) && (us > a)
        } else {
            !((b < us) && (us < a))
        }
    }
}

impl Position {
    fn get_path(&self, target: Position) -> Vector {
        let mut diff_x = target.0 - self.0;
        let mut diff_y = target.1 - self.1;

        let mut mult = 2;

        while (mult <= diff_x.abs()) || (mult <= diff_y.abs()) {
            if ((diff_x % mult) == 0) && ((diff_y % mult) == 0) {
                diff_x = diff_x / mult;
                diff_y = diff_y / mult;
            } else {
                mult += 1;
            }
        }

        Vector(diff_x, diff_y)
    }

    fn intermediates(&self, target: Position) -> Vec<Position> {
        let path = self.get_path(target);

        let mut next_x = self.0 + path.0;
        let mut next_y = self.1 + path.1;

        let mut ints = vec![];

        // println!("{:?} -> {:?} - {},{} - {},{}", self, target, diff_x, diff_y, next_x, next_y);

        while (next_x != target.0) || (next_y != target.1) {
            // println!("{},{}", next_x, next_y);
            ints.push(Position(next_x, next_y));
            next_x += path.0;
            next_y += path.1;
        }

        ints
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 {
            for cell in line {
                match *cell {
                    SPACE => write!(f, " ")?,
                    ASTEROID => write!(f, "#")?,
                    DESTROYED => write!(f, ":")?,
                    STATION => write!(f, "S")?,
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

impl Map {
    fn parse(file: &str) -> Self {
        let mut map = Map(Vec::new());

        let lines: Vec<_> = BufReader::new(File::open(file).expect("Unable to open file"))
            .lines()
            .map(Result::unwrap)
            .collect();

        for line in lines {
            let mut row = Vec::new();
            for char in line.chars() {
                if char == '#' {
                    row.push(ASTEROID);
                } else {
                    row.push(SPACE);
                }
            }
            map.0.push(row);
        }

        map
    }

    fn destroy_asteroids(mut self) -> Vec<Position> {
        let source = self.find_best_station().0;

        self.0[source.1 as usize][source.0 as usize] = STATION;

        let mut last = None;

        let mut destroyed = vec![];

        loop {
            // println!("{}\n{}", destroyed.len(), &self);
            match self.get_next_asteroid(source, last) {
                Some(pos) => {
                    destroyed.push(pos);
                    self.0[pos.1 as usize][pos.0 as usize] = DESTROYED;
                    last = Some(pos);
                }
                None => {
                    return destroyed;
                }
            }
        }
    }

    fn get_next_asteroid(&self, source: Position, last: Option<Position>) -> Option<Position> {
        let mut choice: Option<(Vector, Position)> = None;

        // Vector to last
        let last = last.map(|p| source.get_path(p));

        let mut y = 0;
        for row in &self.0 {
            let mut x = 0;
            for col in row {
                if *col == ASTEROID {
                    let target = Position(x, y);
                    if source == target {
                        // println!("Ignoring {:?} as source", target);
                    } else {
                        let intermediates: Vec<Position> = source
                            .intermediates(target)
                            .into_iter()
                            .filter(|p| self.is_asteroid(*p))
                            .collect();
                        if intermediates.is_empty() {
                            let path = source.get_path(target);

                            choice = match choice {
                                None => {
                                    // println!("Using {:?} as first", target);
                                    Some((path, target))
                                }
                                Some((cpath, cpos)) => match last {
                                    Some(lpath) =>
                                    // A choice is better if it's between the choice and the last
                                    {
                                        if path.between(lpath, cpath) {
                                            // println!(
                                            //    "Using {:?} as between {:?} and {:?}",
                                            //    target, lpath, cpath
                                            // );
                                            Some((path, target))
                                        } else {
                                            // println!(
                                            //    "Ignoring {:?} as {:?} is better",
                                            //    target, cpos
                                            // );
                                            Some((cpath, cpos))
                                        }
                                    }
                                    None => {
                                        if path.before(cpath) {
                                            // println!("Using {:?} as before: {:?}", target, cpos);
                                            Some((path, target))
                                        } else {
                                            // println!(
                                            //    "Ignoring {:?} as {:?} is better",
                                            //    target, cpos
                                            // );
                                            Some((cpath, cpos))
                                        }
                                    }
                                },
                            };
                        } else {
                            // println!(
                            //    "Ignoring {:?} as intermediates: {:?}",
                            //    target, intermediates
                            // );
                        }
                    }
                }
                x += 1;
            }
            y += 1;
        }

        choice.map(|c| c.1)
    }

    fn find_best_station(&self) -> (Position, u32) {
        let mut best = None;

        let mut y = 0;
        for row in &self.0 {
            let mut x = 0;
            for col in row {
                if *col == ASTEROID {
                    let position = Position(x, y);
                    let count = self.visible_from(position);
                    best = match best {
                        Some((opos, oval)) => {
                            if oval < count {
                                Some((position, count))
                            } else {
                                Some((opos, oval))
                            }
                        }
                        None => Some((position, count)),
                    }
                }
                x += 1;
            }
            y += 1;
        }

        best.expect("No station found")
    }

    fn is_asteroid(&self, query: Position) -> bool {
        self.0[query.1 as usize][query.0 as usize] == ASTEROID
    }

    fn visible_from(&self, source: Position) -> u32 {
        let mut found = 0;
        let mut y = 0;

        // println!("Checking what is visible from {:?}", source);

        for row in &self.0 {
            let mut x = 0;
            for col in row {
                if *col == ASTEROID {
                    let target = Position(x, y);

                    // Visibility check

                    // Not visible from ourselves
                    if source != target {
                        let mut visible = true;
                        let options = source.intermediates(target);

                        // println!("{:?} -> {:?} intermediats: {:?}", source, target, options);

                        for option in options {
                            // println!("Checking for asteroid at intermediate: {:?}", option);
                            if self.is_asteroid(option) {
                                // println!("Found asteroid at intermediate: {:?}", option);
                                visible = false;
                                break;
                            }
                        }

                        if visible {
                            found += 1;
                        }
                    }
                }
                x += 1;
            }
            y += 1;
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            Map::parse("10_test_1.txt").find_best_station(),
            (Position(3, 4), 8)
        );
    }

    #[test]
    fn test_a_2() {
        assert_eq!(
            Map::parse("10_test_2.txt").find_best_station(),
            (Position(11, 13), 210)
        );
    }

    #[test]
    fn test_b() {
        let res = Map::parse("10_test_2.txt").destroy_asteroids();

        println!("{:?}", res);

        assert_eq!(res[0], Position(11, 12));
        assert_eq!(res[1], Position(12, 1));
        assert_eq!(res[2], Position(12, 2));
        assert_eq!(res[9], Position(12, 8));
        assert_eq!(res[19], Position(16, 0));
        assert_eq!(res[49], Position(16, 9));
        assert_eq!(res[99], Position(10, 16));
        assert_eq!(res[198], Position(9, 6));
        assert_eq!(res[199], Position(8, 2));
    }
}
