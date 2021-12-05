use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    test("src/four.txt");
    test("src/four-test.txt");
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<(u32, bool)>>, bool);

impl Board {
    fn parse<'a>(x: impl Iterator<Item = &'a str>) -> Self {
        Board(
            x.skip(1)
                .map(|x| {
                    x.split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| (x.parse().unwrap(), false))
                        .collect()
                })
                .collect(),
            false,
        )
    }

    fn add(&mut self, i: u32) {
        for row in &mut self.0 {
            for cell in row.iter_mut() {
                if cell.0 == i {
                    cell.1 = true;
                    return;
                }
            }
        }
    }

    fn finished(&mut self) -> bool {
        if self.1 {
            return false;
        }
        // Check rows
        for row in &self.0 {
            let mut score = true;
            for cell in row.iter() {
                if !cell.1 {
                    score = false;
                    break;
                }
            }

            if score {
                self.1 = true;
                return true;
            }
        }

        // Check columns
        for i in 0..5 {
            let mut score = true;
            for row in &self.0 {
                if !row[i].1 {
                    score = false;
                    break;
                }
            }
            if score {
                self.1 = true;
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let mut u = 0;
        for row in &self.0 {
            for cell in row {
                if !cell.1 {
                    u += cell.0;
                }
            }
        }
        u
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let mut data = data.lines();

    let input: Vec<u32> = data
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = data.chunks(6).into_iter().map(Board::parse).collect();

    let mut first = None;
    let mut last = None;

    for i in input {
        for b in &mut boards {
            b.add(i);
            if b.finished() {
                if first.is_none() {
                    first = Some((b.clone(), i));
                }
                last = Some((b.clone(), i));
            }
        }
    }
    let first = first.unwrap();
    let score = first.0.score();
    let called = first.1;
    println!("Score: {}", score);
    println!("Called: {}", called);
    println!("A: {}", score * called);
    let last = last.unwrap();
    println!("B: {}", last.0.score() * last.1);
}
