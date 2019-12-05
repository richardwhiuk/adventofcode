use crate::intcode::*;

pub fn run_a() {
    let data = std::fs::read_to_string("5.txt").expect("Unable to read file");
    let program = parse_program(&data);
    let (_, output) = execute_program(program, vec![1]);
    println!("5a output = {:?}", output);
}

pub fn run_b() {
    let data = std::fs::read_to_string("5.txt").expect("Unable to read file");
    let program = parse_program(&data);
    let (_, output) = execute_program(program, vec![5]);
    println!("5b output = {:?}", output);
}
