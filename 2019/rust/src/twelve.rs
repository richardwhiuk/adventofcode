/*
<x=1, y=-4, z=3>
<x=-14, y=9, z=-4>
<x=-4, y=-6, z=7>
<x=6, y=-9, z=-11>
*/

#[derive(Debug, Clone, Copy)]
struct Axis {
    position: i32,
    velocity: i32
}

#[derive(Debug, Clone)]
struct Moon {
    name: &'static str,
    x: Axis,
    y: Axis,
    z: Axis
}

impl Moon {
    fn new(name: &'static str, x: i32, y: i32, z: i32) -> Self {
        Self {
            name,
            velocity: Vector { x: 0, y: 0, z: 0 },
            position: Vector { x, y, z },
        }
    }

    fn energy(&self) -> i32 {
       let pot = self.x.position.abs() + self.y.position.abs() + self.z.position.abs();
       let pot = self.x.velocity.abs() + self.y.velocity.abs() + self.z.velocity.abs();
       println!("{} pot: {} vel: {}", self.name, pot, vel);
       pot * vel
    }

    fn apply_gravity(&mut self, source: &Moon) {
        if self.position.x < source.position.x {
            self.velocity.x += 1;
        } else if self.position.x > source.position.x {
            self.velocity.x -= 1;
        }
        if self.position.y < source.position.y {
            self.velocity.y += 1;
        } else if self.position.y > source.position.y {
            self.velocity.y -= 1;
        }
        if self.position.z < source.position.z {
            self.velocity.z += 1;
        } else if self.position.z > source.position.z {
            self.velocity.z -= 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn print_x(&self) {
        println!("{}: {} / {}", self.name, self.position.x, self.velocity.x);
    }
}

pub fn run_a() {
    let moons = vec![
        Moon::new("a", 1, -4, 3),
        Moon::new("b", -14, 9, -4),
        Moon::new("c", -4, -6, 7),
        Moon::new("d", 6, -9, -11),
    ];
        let moons = n_steps(moons, 1000);
        let energy = moons.iter().fold(0, |res, m| res + m.energy());
println!("12a: {}", energy);
}

pub fn run_b() {
    let moons = vec![
        Moon::new("a", 1, -4, 3),
        Moon::new("b", -14, 9, -4),
        Moon::new("c", -4, -6, 7),
        Moon::new("d", 6, -9, -11),
    ];
}

fn determine_cycle(moons: Vec<Moon>) -> i32 {
    let start_x = moons.iter().map(|m| m.x()).collect();
    let mut last_x = vec![]

    let steps = 0;

    while start_x != last_x {
        moons = step_moons(moons);
        steps += 1;
        last_x = 
    }

    n_steps(moons, 100);
}

fn step_moons(moons: Vec<Moon>) -> Vec<Moon> {
    let mut moons: Vec<Moon> = {
        let mut new = vec![];

        for moon in &moons {
            let mut updated = moon.clone();

            for source in &moons {
                if updated.name != source.name {
                    updated.apply_gravity(source);
                }
            }

            new.push(updated);
        }

        new
    };

    moons.iter_mut().for_each(|m| m.apply_velocity());

    moons
}

fn n_steps(mut moons: Vec<Moon>, n :u32) -> Vec<Moon> {
    for step in 0..n {
        //println!("{}: {:?}", step, moons);
        println!("{}", step);
        moons.iter().for_each(|m| m.print_x());
        moons = step_moons(moons);
    }

    //println!("{}: {:?}", n, moons);
        println!("{}", n);
        moons.iter().for_each(|m| m.print_x());

    moons
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let moons = vec![
            Moon::new("a", -1, 0, 2),
            Moon::new("b", 2, -10, -7),
            Moon::new("c", 4, -8, 8),
            Moon::new("d", 3, 5, -1),
        ];
        println!("0\n{:?}", moons);
        let moons = n_steps(moons, 10);
        let energy = moons.iter().fold(0, |res, m| res + m.energy());
        assert_eq!(energy, 179);
    }
}
