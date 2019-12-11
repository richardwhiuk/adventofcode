use crate::intcode::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn x(&self) -> i32 {
        match self {
            Up => 0,
            Down => 0,
            Left => -1,
            Right => 1,
        }
    }

    fn y(&self) -> i32 {
        match self {
            Up => -1,
            Down => 1,
            Left => 0,
            Right => 0,
        }
    }
}

use Direction::*;

struct Robot {
    direction: Direction,
    x: i32,
    y: i32,
    panels: HashMap<(i32, i32), u8>,
}

impl Robot {
    fn new() -> Self {
        Self {
            direction: Up,
            x: 0,
            y: 0,
            panels: HashMap::new(),
        }
    }
    fn run(&mut self, paint: i64, dir: i64) {
        self.panels.insert((self.x, self.y), paint as u8);
        self.direction = match (self.direction, dir) {
            (Up, 0) => Left,
            (Up, 1) => Right,
            (Left, 0) => Down,
            (Left, 1) => Up,
            (Right, 0) => Up,
            (Right, 1) => Down,
            (Down, 0) => Right,
            (Down, 1) => Left,
            _ => panic!("Unexpected turn: {}", dir),
        };

        self.x += self.direction.x();
        self.y += self.direction.y();
    }

    fn color(&self) -> u8 {
        self.panels
            .get(&(self.x, self.y))
            .map(Clone::clone)
            .unwrap_or(0)
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut minx = 0;
        let mut miny = 0;
        let mut maxx = 0;
        let mut maxy = 0;

        for coord in self.panels.keys() {
            minx = std::cmp::min(coord.0, minx);
            miny = std::cmp::min(coord.1, miny);
            maxx = std::cmp::max(coord.0, maxx);
            maxy = std::cmp::max(coord.1, maxy);
        }

        for y in miny..(maxy + 1) {
            for x in minx..(maxx + 1) {
                write!(
                    f,
                    "{}",
                    match self.panels.get(&(x, y)) {
                        None => " ",
                        Some(0) => "_",
                        Some(_) => "#",
                    }
                )?;
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn run(mut robot: Robot, out: i64) -> Robot {
    let data = std::fs::read_to_string("11.txt").expect("Unable to read file");
    let mut program = Intcode::from(&data);
    let mut intcode = program.inputc(vec![out]);

    let mut output = 0;

    loop {
        let data = intcode.output();
        while output < data.len() {
            let paint = data[output];
            let dir = data[output + 1];
            output += 2;

            robot.run(paint, dir);
        }

        match intcode {
            NeedMoreInput(mut vm) => {
                vm.add_input(robot.color() as i64);
                intcode = vm.execute();
            }
            Finished(res) => {
                let data = res.output;
                while output < data.len() {
                    let paint = data[output];
                    let dir = data[output + 1];
                    output += 2;

                    robot.run(paint, dir);
                }
                break;
            }
        }
    }

    robot
}

pub fn run_a() {
    let robot = run(Robot::new(), 0);

    let panels: Vec<_> = robot.panels.keys().collect();

    println!("11a: {}", panels.len());
}

pub fn run_b() {
    let mut robot = Robot::new();
    let robot = run(robot, 1);

    println!("{}", robot);
}
