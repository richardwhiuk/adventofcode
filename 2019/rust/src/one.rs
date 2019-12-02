use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run_a() {
   run("a", calculate_module_fuel);
}

pub fn run_b(){
   run("b", calculate_full_fuel);
}

pub fn run<F>(part: &'static str, f: F) where F: Fn(i32) -> i32 {
    println!(
        "1{}: Total: {}",
        part,
        BufReader::new(File::open("1.txt").expect("Unable to open file"))
            .lines()
            .map(|s| s
                .expect("Failed to get line")
                .parse()
                .expect("Unable to parse line"))
            .map(f)
            .fold(0, |a, b| a + b)
    );
}

fn calculate_module_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calculate_full_fuel(mut mass: i32) -> i32 {
    let mut fuel = 0;

    loop {
        let additional = (mass / 3) - 2;

        if additional <= 0 {
            return fuel;
        } else {
            fuel += additional;
            mass = additional;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(2, calculate_module_fuel(12));
        assert_eq!(2, calculate_module_fuel(14));
        assert_eq!(654, calculate_module_fuel(1969));
        assert_eq!(33583, calculate_module_fuel(100756));
    }
}
