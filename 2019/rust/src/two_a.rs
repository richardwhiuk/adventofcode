pub fn run() {
    let data = std::fs::read_to_string("2a.txt").expect("Unable to read file");
    let data = data.trim();
    let mut program = parse_program(data);
    program[1] = 12;
    program[2] = 2;
    let program = execute_program(program);
    println!("2a: program[0] = {}", program[0]);
}

fn parse_program(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
        .collect()
}

#[cfg(test)]
fn execute_string(input: &str) -> Vec<usize> {
    execute_program(parse_program(input))
}

fn execute_program(mut input: Vec<usize>) -> Vec<usize> {
    let mut position = 0;

    loop {
        match input[position] {
            1 => {
                // Opcode Add
                let res_loc = input[position + 3];
                input[res_loc] = input[input[position + 1]] + input[input[position + 2]];
                position += 4
            }
            2 => {
                // Opcode Multiply
                let res_loc = input[position + 3];
                input[res_loc] = input[input[position + 1]] * input[input[position + 2]];
                position += 4
            }
            99 => {
                // Opcode Quit
                return input;
            }
            opcode => {
                panic!("Unexpected opcode: {}", opcode);
            }
        }
    }
}

#[cfg(test)]
fn execute_to_string(input: &str) -> String {
    let r: Vec<_> = execute_string(input)
        .iter()
        .map(|s| s.to_string())
        .collect();
    r.join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus() {
        assert_eq!("2,0,0,0,99", execute_to_string("1,0,0,0,99"));
    }

    #[test]
    fn test_mult() {
        assert_eq!(execute_to_string("2,3,0,3,99"), "2,3,0,6,99");
    }

    #[test]
    fn test_mult_b() {
        assert_eq!(execute_to_string("2,4,4,5,99,0"), "2,4,4,5,99,9801");
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            execute_to_string("1,1,1,4,99,5,6,0,99"),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
