use crate::intcode::*;

pub fn run_a() {
    let data = std::fs::read_to_string("9.txt").expect("Unable to read file");
    let program = Intcode::from(&data);
    println!("9a: {}", program.input(vec![1]).output[0]);
}

pub fn run_b() {
    let data = std::fs::read_to_string("9.txt").expect("Unable to read file");
    let program = Intcode::from(&data);
    println!("9b: {}", program.input(vec![2]).output[0]);
}
