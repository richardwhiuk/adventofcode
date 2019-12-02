use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    println!(
        "1a: Total: {}",
        BufReader::new(File::open("1a.txt").expect("Unable to open file"))
            .lines()
            .map(|s| s
                .expect("Failed to get line")
                .parse()
                .expect("Unable to parse line"))
            .map(calculate_module_fuel)
            .fold(0, |a, b| a + b)
    );
}

fn calculate_module_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
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
