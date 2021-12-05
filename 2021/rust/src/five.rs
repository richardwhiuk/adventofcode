use std::cmp::{max, min};
use std::fs::read_to_string;

fn main() {
    test("src/five.txt");
    test("src/five-test.txt");
}

struct Map(Vec<Vec<i32>>);

impl Map {
    fn add(&mut self, x: i32, y: i32) {
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();
        while self.0.len() <= y {
            self.0.push(vec![]);
        }
        while self.0[y].len() <= x {
            self.0[y].push(0)
        }
        self.0[y][x] += 1;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.0 {
            print!(":");
            for v in row {
                if *v == 0 {
                    print!(".");
                } else {
                    print!("{}", *v);
                }
            }
            println!("");
        }
    }

    fn score(&self) -> u32 {
        let mut s = 0;
        for row in &self.0 {
            for v in row {
                if *v > 1 {
                    s += 1;
                }
            }
        }
        s
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data = data.lines();

    let data: Vec<((i32, i32), (i32, i32))> = data
        .map(|line| {
            let line: Vec<(i32, i32)> = line
                .split(" -> ")
                .map(|position| {
                    let pos: Vec<i32> = position.split(",").map(|x| x.parse().unwrap()).collect();
                    (pos[0], pos[1])
                })
                .collect();
            (line[0], line[1])
        })
        .collect();

    let mut map = Map(vec![]);

    for line in &data {
        if line.0 .0 == line.1 .0 {
            let s = min(line.0 .1, line.1 .1);
            let e = max(line.0 .1, line.1 .1);
            for y in s..=e {
                map.add(line.0 .0, y);
            }
        } else if line.0 .1 == line.1 .1 {
            let s = min(line.0 .0, line.1 .0);
            let e = max(line.0 .0, line.1 .0);
            for x in s..=e {
                map.add(x, line.0 .1);
            }
        }
    }

    println!("A: {}", map.score());

    let mut map = Map(vec![]);
    for line in data {
        let sx = line.0 .0;
        let sy = line.0 .1;
        let ex = line.1 .0;
        let ey = line.1 .1;

        let len = max((ex - sx).abs(), (ey - sy).abs());
        let dx = if sx == ex {
            0
        } else if sx > ex {
            -1
        } else {
            1
        };
        let dy = if sy == ey {
            0
        } else if sy > ey {
            -1
        } else {
            1
        };

        for i in 0..=len {
            map.add(sx + (dx * i), sy + (dy * i));
        }
    }

    println!("B: {}", map.score());
}
