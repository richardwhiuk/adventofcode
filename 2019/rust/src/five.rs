use crate::intcode::*;

pub fn run_a() {
    let data = std::fs::read_to_string("5.txt").expect("Unable to read file");
    let output = Intcode::from(&data).input(vec![1]).output;
    println!("5a output = {:?}", output);
}

pub fn run_b() {
    let data = std::fs::read_to_string("5.txt").expect("Unable to read file");
    let output = Intcode::from(&data).input(vec![5]).output;
    println!("5b output = {:?}", output);
}
