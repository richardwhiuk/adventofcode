pub use rust::*;

fn main() -> Result<()> {
    a("input/seventeen-test.txt")?;
    a("input/seventeen.txt")?;

    b("input/seventeen-test.txt")?;
    b("input/seventeen.txt")?;

    Ok(())
}

#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn active(&self, points: &HashSet<Point>) -> bool {
        let mut neigh = 0;

        debug!("Considering: {:?}", self);

        for w in (self.w - 1)..=(self.w + 1) {
            for x in (self.x - 1)..=(self.x + 1) {
                for y in (self.y - 1)..=(self.y + 1) {
                    for z in (self.z - 1)..=(self.z + 1) {
                        let p = Point { w, x, y, z };

                        if p == *self {
                            debug!("Ignoring self: {:?} for {:?}", p, self);
                        // Skip self
                        } else if points.contains(&p) {
                            debug!("Found active: {:?} for {:?}", p, self);
                            neigh += 1;
                            if neigh > 3 {
                                return false;
                            }
                        } else {
                            debug!("Inactive: {:?} for {:?}", p, self);
                        }
                    }
                }
            }
        }

        if points.contains(self) {
            debug!("Now active");
            debug!("");
            neigh == 2 || neigh == 3
        } else {
            debug!("Now inactive");
            debug!("");
            neigh == 3
        }
    }

    fn max(&mut self, p: &Point) {
        if p.w > self.w {
            self.w = p.w;
        }
        if p.x > self.x {
            self.x = p.x;
        }
        if p.y > self.y {
            self.y = p.y;
        }
        if p.z > self.z {
            self.z = p.z;
        }
    }

    fn min(&mut self, p: &Point) {
        if p.w < self.w {
            self.w = p.w;
        }
        if p.x < self.x {
            self.x = p.x;
        }
        if p.y < self.y {
            self.y = p.y;
        }
        if p.z < self.z {
            self.z = p.z;
        }
    }
}

#[derive(Default, Debug)]
struct World {
    points: HashSet<Point>,
    min: Point,
    max: Point,
}

impl World {
    fn active(&self) -> usize {
        self.points.len()
    }

    fn iterate(&mut self) {
        let points = std::mem::replace(&mut self.points, HashSet::new());
        let min = self.min;
        let max = self.max;
        for x in (min.x - 1)..=(max.x + 1) {
            for y in (min.y - 1)..=(max.y + 1) {
                for z in (min.z - 1)..=(max.z + 1) {
                    let p = Point { w: 0, x, y, z };
                    debug!("I: {:?}", p);

                    if p.active(&points) {
                        self.points.insert(p);
                        self.min.min(&p);
                        self.max.max(&p);
                    }
                }
            }
        }
    }

    fn hiterate(&mut self) {
        let points = std::mem::replace(&mut self.points, HashSet::new());
        let min = self.min;
        let max = self.max;
        for w in (min.w - 1)..=(max.w + 1) {
            for x in (min.x - 1)..=(max.x + 1) {
                for y in (min.y - 1)..=(max.y + 1) {
                    for z in (min.z - 1)..=(max.z + 1) {
                        let p = Point { w, x, y, z };
                        debug!("I: {:?}", p);

                        if p.active(&points) {
                            self.points.insert(p);
                            self.min.min(&p);
                            self.max.max(&p);
                        }
                    }
                }
            }
        }
    }

    fn print(&self) {
        for w in self.min.w..=self.max.w {
            for z in self.min.z..=self.max.z {
                println!("Z={}", z);
                for y in self.min.y..=self.max.y {
                    print!("{}", y);
                    for x in self.min.x..=self.max.x {
                        let p = Point { w, x, y, z };
                        if self.points.contains(&p) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!("");
                }
            }
        }
    }
}

fn read(path: &str) -> Result<World> {
    let mut world = World::default();

    let mut y = 0;

    reader(path, |line| {
        let mut x = 0;

        for c in line.chars() {
            match c {
                '#' => {
                    world.points.insert(Point { w: 0, x, y, z: 0 });
                }
                '.' => {}
                c => panic!("unexpected char: {}", c),
            }

            x += 1;
        }

        y += 1;

        if x - 1 > world.max.x {
            world.max.x = x - 1;
        }

        Ok(())
    })?;

    world.max.y = y - 1;

    Ok(world)
}

fn a(path: &str) -> Result<()> {
    let mut world = read(path)?;

    world.print();

    for iter in 1..=6 {
        world.iterate();
        println!("Iteration: {}", iter);
        //world.print();
    }

    println!("A Result: {}", world.active());

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let mut world = read(path)?;

    world.print();

    for iter in 1..=6 {
        world.hiterate();
        println!("Iteration: {}", iter);
        //world.print();
    }

    println!("B Result: {}", world.active());
    Ok(())
}
