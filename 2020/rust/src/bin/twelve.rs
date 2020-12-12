pub use rust::*;

fn main() -> Result<()> {
    // 1A
    a("input/twelve-test.txt")?;
    a("input/twelve.txt")?;

    // 1B
    b("input/twelve-test.txt")?;
    b("input/twelve.txt")?;

    Ok(())
}

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Instruction {
    fn from(line: String) -> Instruction {
        let i = line.chars().next().unwrap();
        let j = line[1..].parse().expect("Not number");
        use Instruction::*;
        match i {
            'N' => North(j),
            'S' => South(j),
            'E' => East(j),
            'W' => West(j),
            'L' => Left(j),
            'R' => Right(j),
            'F' => Forward(j),
            c => panic!("Unexpected instruction: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn read(path: &str) -> Result<Vec<Instruction>> {
    let mut res = vec![];

    reader(path, |line| {
        res.push(Instruction::from(line));
        Ok(())
    })?;

    Ok(res)
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Direction {
    fn update(self, deg: i32) -> Direction {
        use Direction::*;
        if deg % 90 != 0 {
            panic!("Urgh, not 90 degree angle");
        }
        let mut deg: i32 = deg / 90;
        let mut zelf: Direction = self;

        debug!("Turning {} turns", deg);

        while deg != 0 {
            if deg > 0 {
                zelf = match zelf {
                    East => North,
                    North => West,
                    West => South,
                    South => East,
                };
                deg -= 1;
            } else {
                zelf = match zelf {
                    East => South,
                    North => East,
                    West => North,
                    South => West,
                };
                deg += 1;
            }
        }

        zelf
    }
}

impl Ship {
    fn mv(&mut self, i: Instruction) {
        use Instruction::*;
        match i {
            North(x) => {
                self.y += x;
            }
            South(x) => {
                self.y -= x;
            }
            East(x) => {
                self.x += x;
            }
            West(x) => {
                self.x -= x;
            }
            Forward(x) => match self.dir {
                Direction::North => {
                    self.y += x;
                }
                Direction::South => {
                    self.y -= x;
                }
                Direction::East => {
                    self.x += x;
                }
                Direction::West => {
                    self.x -= x;
                }
            },
            Left(x) => {
                self.dir = self.dir.update(x);
            }
            Right(x) => {
                self.dir = self.dir.update(-x);
            }
        }
    }

    fn waypoint(&mut self, i: Instruction, wp: &mut Waypoint) {
        use Instruction::*;
        match i {
            North(x) => {
                wp.y += x;
            }
            South(x) => {
                wp.y -= x;
            }
            East(x) => {
                wp.x += x;
            }
            West(x) => {
                wp.x -= x;
            }
            Forward(x) => {
                self.x += wp.x * x;
                self.y += wp.y * x;
            }
            Left(x) => {
                wp.rotate(x);
            }
            Right(x) => {
                wp.rotate(-x);
            }
        }
    }
}

fn a(path: &str) -> Result<()> {
    let is = read(path)?;

    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: Direction::East,
    };

    for i in is {
        ship.mv(i);
        debug!("Ship: {:?}", ship);
    }

    println!("A Result: {}", ship.x.abs() + ship.y.abs());

    Ok(())
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn rotate(&mut self, deg: i32){
        if deg % 90 != 0 {
            panic!("Urgh, not 90 degree angle");
        }
        let mut deg: i32 = deg / 90;
        while deg != 0 {
            if deg > 0 {
                deg -= 1;
                let nx = -self.y;
                self.y = self.x;

                self.x = nx;
            } else {
                deg += 1;
                let nx = self.y;
                self.y = -self.x;
                self.x = nx;
            }
        }
    }
}

fn b(path: &str) -> Result<()> {
    let is = read(path)?;

    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: Direction::East,
    };

    let mut waypoint = Waypoint { x: 10, y: 1 };

    for i in is {
        ship.waypoint(i, &mut waypoint);
        debug!("Ship: {:?} Waypoint: {:?}", ship, waypoint);
    }

    println!("B Result: {}", ship.x.abs() + ship.y.abs());

    Ok(())
}
