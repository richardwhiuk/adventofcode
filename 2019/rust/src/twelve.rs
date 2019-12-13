/*
<x=1, y=-4, z=3>
<x=-14, y=9, z=-4>
<x=-4, y=-6, z=7>
<x=6, y=-9, z=-11>
*/

#[derive(Debug, Clone, Copy, PartialEq)]
struct Axis {
    position: i32,
    velocity: i32,
}

impl Axis {
    fn apply_gravity(&mut self, source: &Axis) {
        if self.position < source.position {
            self.velocity += 1;
        } else if self.position > source.position {
            self.velocity -= 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }
}

#[derive(Debug, Clone)]
struct Moon {
    name: &'static str,
    x: Axis,
    y: Axis,
    z: Axis,
}

impl Moon {
    fn new(name: &'static str, x: i32, y: i32, z: i32) -> Self {
        Self {
            name,
            x: Axis {
                position: x,
                velocity: 0,
            },
            y: Axis {
                position: y,
                velocity: 0,
            },
            z: Axis {
                position: z,
                velocity: 0,
            },
        }
    }

    fn energy(&self) -> i32 {
        let pot = self.x.position.abs() + self.y.position.abs() + self.z.position.abs();
        let vel = self.x.velocity.abs() + self.y.velocity.abs() + self.z.velocity.abs();
        println!("{} pot: {} vel: {}", self.name, pot, vel);
        pot * vel
    }

    fn apply_gravity(&mut self, source: &Moon) {
        self.x.apply_gravity(&source.x);
        self.y.apply_gravity(&source.y);
        self.z.apply_gravity(&source.z);
    }

    fn apply_velocity(&mut self) {
        self.x.apply_velocity();
        self.y.apply_velocity();
        self.z.apply_velocity();
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

    println!("12b: {}", determine_cycle(moons));
}

fn determine_cycle(moons: Vec<Moon>) -> u64 {
    let x = determine_cycle_axis(moons.iter().map(|m| m.x.clone()).collect());
    let y = determine_cycle_axis(moons.iter().map(|m| m.y.clone()).collect());
    let z = determine_cycle_axis(moons.iter().map(|m| m.z.clone()).collect());

    let total = num::integer::lcm(x, y);
    num::integer::lcm(total, z)
}

fn determine_cycle_axis(start_x: Vec<Axis>) -> u64 {
    let mut last_x = step_axis(start_x.clone());

    let mut steps = 1;

    while start_x != last_x {
        last_x = step_axis(last_x);
        steps += 1;
    }

    println!("{} steps in x", steps);

    steps
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

fn step_axis(moons: Vec<Axis>) -> Vec<Axis> {
    let mut moons: Vec<Axis> = {
        let mut new = vec![];
        let mut n = 0;

        for moon in &moons {
            let mut updated = moon.clone();

            for source in &moons {
                updated.apply_gravity(source);
            }

            new.push(updated);
        }

        new
    };

    moons.iter_mut().for_each(|m| m.apply_velocity());

    moons
}

fn n_steps(mut moons: Vec<Moon>, n: u32) -> Vec<Moon> {
    for step in 0..n {
        //println!("{}: {:?}", step, moons);
        moons = step_moons(moons);
    }

    //println!("{}: {:?}", n, moons);

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

    #[test]
    fn test_b() {
        let moons = vec![
            Moon::new("a", -1, 0, 2),
            Moon::new("b", 2, -10, -7),
            Moon::new("c", 4, -8, 8),
            Moon::new("d", 3, 5, -1),
        ];

        assert_eq!(determine_cycle(moons), 2772);
    }
}
