pub fn parse_program(program: &str) -> Vec<i32> {
    program
        .trim()
        .split(',')
        .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
        .collect()
}

#[cfg(test)]
pub fn execute_string(program: &str, input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    execute_program(parse_program(program), input)
}

fn get_param(val: i32, program: &Vec<i32>, mode: i32) -> i32 {
    match mode {
        0 => {
            // Position mode
            program[val as usize]
        }
        1 => {
            // Immediate mode
            val
        }
        mode => panic!("Unknown parameter mode: {}", mode),
    }
}

pub fn execute_program(mut program: Vec<i32>, mut input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut position: usize = 0;

    let mut output = Vec::new();

    loop {
        let code = program[position];

        let opcode = code % 100;
        let param_modes = code / 100;
        let param_mode_a = param_modes % 10;
        let param_mode_b = (param_modes / 10) % 10;
        //let param_mode_c = (param_modes / 100) % 10;

        match opcode {
            1 => {
                // Opcode Add
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                let res_loc = program[position + 3];
                program[res_loc as usize] = a + b;
                position += 4
            }
            2 => {
                // Opcode Multiply
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                let res_loc = program[position + 3];
                program[res_loc as usize] = a * b;
                position += 4
            }
            3 => {
                // Opcode Input
                let res_loc = program[position + 1];
                program[res_loc as usize] = input.pop().expect("Unsufficient input available");
                position += 2
            }
            4 => {
                // Opcode Output
                let o = get_param(program[position + 1], &program, param_mode_a);
                output.push(o);
                position += 2
            }
            5 => {
                // Jump if true
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                if a != 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            6 => {
                // Jump if false
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                if a == 0 {
                    position = b as usize;
                } else {
                    position += 3;
                }
            }
            7 => {
                // Less than
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                let res_loc = program[position + 3] as usize;
                if a < b {
                    program[res_loc] = 1;
                } else {
                    program[res_loc] = 0;
                }
                position += 4
            }
            8 => {
                // Equals
                let a = get_param(program[position + 1], &program, param_mode_a);
                let b = get_param(program[position + 2], &program, param_mode_b);
                let res_loc = program[position + 3] as usize;
                if a == b {
                    program[res_loc] = 1;
                } else {
                    program[res_loc] = 0;
                }
                position += 4
            }
            99 => {
                // Opcode Quit
                return (program, output);
            }
            opcode => {
                panic!("Unexpected opcode: {}", opcode);
            }
        }
    }
}

#[cfg(test)]
pub fn execute_to_string(program: &str) -> String {
    let r: Vec<_> = execute_string(program, Vec::new())
        .0
        .iter()
        .map(|s| s.to_string())
        .collect();
    r.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        assert_eq!(execute_string("3,0,4,0,99", vec![94]).1, vec![94]);
    }

    #[test]
    fn param_mode_test() {
        assert_eq!(
            execute_string("1002,4,3,4,33", vec![]).0,
            vec![1002, 4, 3, 4, 99]
        );
    }

    #[test]
    fn pos_mode_equal() {
        assert_eq!(
            execute_string("3,9,8,9,10,9,4,9,99,-1,8", vec![8]).1,
            vec![1]
        );
        assert_eq!(
            execute_string("3,9,8,9,10,9,4,9,99,-1,8", vec![9]).1,
            vec![0]
        );
    }

    #[test]
    fn pos_mode_less() {
        assert_eq!(
            execute_string("3,9,7,9,10,9,4,9,99,-1,8", vec![8]).1,
            vec![0]
        );
        assert_eq!(
            execute_string("3,9,7,9,10,9,4,9,99,-1,8", vec![7]).1,
            vec![1]
        );
    }

    #[test]
    fn imm_mode_equal() {
        assert_eq!(execute_string("3,3,1108,-1,8,3,4,3,99", vec![8]).1, vec![1]);
        assert_eq!(execute_string("3,3,1108,-1,8,3,4,3,99", vec![7]).1, vec![0]);
    }

    #[test]
    fn imm_mode_less() {
        assert_eq!(execute_string("3,3,1107,-1,8,3,4,3,99", vec![8]).1, vec![0]);
        assert_eq!(execute_string("3,3,1107,-1,8,3,4,3,99", vec![7]).1, vec![1]);
    }
}
