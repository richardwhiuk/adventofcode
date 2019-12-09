#[derive(Debug, Clone)]
pub struct Intcode(pub Vec<i64>);

#[derive(Debug, Clone)]
pub struct IntcodeResult {
    pub program: Vec<i64>,
    pub output: Vec<i64>,
}

pub enum IntcodeState {
    NeedMoreInput(IntcodeVm),
    Finished(IntcodeResult),
}

pub use IntcodeState::*;

impl IntcodeState {
    fn unwrap(self) -> IntcodeResult {
        match self {
            NeedMoreInput(_) => panic!("Need more input"),
            Finished(f) => f,
        }
    }

    pub fn get_output(&self) -> Vec<i64> {
        match self {
            NeedMoreInput(vm) => vm.output.clone(),
            Finished(res) => res.output.clone(),
        }
    }
}

impl Intcode {
    pub fn from(program: &str) -> Self {
        Self(
            program
                .trim()
                .split(',')
                .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
                .collect(),
        )
    }

    pub fn execute(self) -> IntcodeResult {
        self.input(vec![])
    }

    pub fn input(self, input: Vec<i64>) -> IntcodeResult {
        self.inputc(input).unwrap()
    }

    pub fn inputc(self, input: Vec<i64>) -> IntcodeState {
        IntcodeVm {
            program: self.0,
            input: input,
            position: 0,
            output: vec![],
            relative_base: 0,
        }
        .execute()
    }
}

pub struct IntcodeVm {
    position: usize,
    program: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    relative_base: i64,
}

impl IntcodeVm {
    pub fn add_input(&mut self, input: i64) {
        self.input.push(input);
    }

    fn param(&mut self, mode: i64) -> i64 {
        let val = self.program[self.position];
        self.position += 1;
        match mode {
            0 => {
                // Position mode
                self.program
                    .get(val as usize)
                    .map(Clone::clone)
                    .unwrap_or(0)
            }
            1 => {
                // Immediate mode
                val
            }
            2 => {
                // Relative base
                self.program[(val + self.relative_base) as usize]
            }
            mode => panic!("Unknown parameter mode: {}", mode),
        }
    }

    fn result(&mut self, value: i64, mode: i64) {
        let val = self.program[self.position];
        let res_loc = match mode {
            0 => {
                val
            }
            2 => {
                val + self.relative_base
            }
            mode => panic!("Unexpected parameter mode: {}", mode),
        };
        let res_loc = res_loc as usize;
        if self.program.len() <= res_loc {
            self.program.resize(res_loc + 1, 0);
        }
        self.program[res_loc] = value;
        self.position += 1;
    }

    pub fn execute(mut self) -> IntcodeState {
        loop {
            let code = self.program[self.position];

            let opcode = code % 100;
            let param_modes = code / 100;
            let param_mode_a = param_modes % 10;
            let param_mode_b = (param_modes / 10) % 10;
            let param_mode_c = (param_modes / 100) % 10;
            self.position += 1;

            match opcode {
                1 => {
                    // Opcode Add
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    self.result(a + b, param_mode_c);
                }
                2 => {
                    // Opcode Multiply
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    self.result(a * b, param_mode_c);
                }
                3 => {
                    // Opcode Input
                    match self.input.pop() {
                        Some(input) => {
                            self.result(input, param_mode_a);
                        }
                        None => {
                            // Move the IP back to when we needed the input
                            self.position -= 1;
                            return NeedMoreInput(self);
                        }
                    }
                }
                4 => {
                    // Opcode Output
                    let val = self.param(param_mode_a);
                    self.output.push(val);
                }
                5 => {
                    // Jump if true
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    if a != 0 {
                        self.position = b as usize;
                    }
                }
                6 => {
                    // Jump if false
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    if a == 0 {
                        self.position = b as usize;
                    }
                }
                7 => {
                    // Less than
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    self.result(if a < b { 1 } else { 0 }, param_mode_c);
                }
                8 => {
                    // Equals
                    let a = self.param(param_mode_a);
                    let b = self.param(param_mode_b);
                    self.result(if a == b { 1 } else { 0 }, param_mode_c);
                }
                9 => {
                    // Relative base adjust
                    let a = self.param(param_mode_a);
                    self.relative_base += a;
                }
                99 => {
                    // Opcode Quit
                    return Finished(IntcodeResult {
                        program: self.program,
                        output: self.output,
                    });
                }
                opcode => {
                    panic!("Unexpected opcode: {}", opcode);
                }
            }
        }
    }
}

#[cfg(test)]
impl IntcodeResult {
    pub fn string(self) -> String {
        let r: Vec<_> = self.program.iter().map(|s| s.to_string()).collect();
        r.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        assert_eq!(Intcode::from("3,0,4,0,99").input(vec![94]).output, vec![94]);
    }

    #[test]
    fn param_mode_test() {
        assert_eq!(
            Intcode::from("1002,4,3,4,33").execute().program,
            vec![1002, 4, 3, 4, 99]
        );
    }

    #[test]
    fn pos_mode_equal() {
        assert_eq!(
            Intcode::from("3,9,8,9,10,9,4,9,99,-1,8")
                .input(vec![8])
                .output,
            vec![1]
        );
        assert_eq!(
            Intcode::from("3,9,8,9,10,9,4,9,99,-1,8")
                .input(vec![9])
                .output,
            vec![0]
        );
    }

    #[test]
    fn pos_mode_less() {
        assert_eq!(
            Intcode::from("3,9,7,9,10,9,4,9,99,-1,8")
                .input(vec![8])
                .output,
            vec![0]
        );
        assert_eq!(
            Intcode::from("3,9,7,9,10,9,4,9,99,-1,8")
                .input(vec![7])
                .output,
            vec![1]
        );
    }

    #[test]
    fn imm_mode_equal() {
        assert_eq!(
            Intcode::from("3,3,1108,-1,8,3,4,3,99")
                .input(vec![8])
                .output,
            vec![1]
        );
        assert_eq!(
            Intcode::from("3,3,1108,-1,8,3,4,3,99")
                .input(vec![7])
                .output,
            vec![0]
        );
    }

    #[test]
    fn imm_mode_less() {
        assert_eq!(
            Intcode::from("3,3,1107,-1,8,3,4,3,99")
                .input(vec![8])
                .output,
            vec![0]
        );
        assert_eq!(
            Intcode::from("3,3,1107,-1,8,3,4,3,99")
                .input(vec![7])
                .output,
            vec![1]
        );
    }

    #[test]
    fn relative_base_test() {
        assert_eq!(
            Intcode::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")
                .input(vec![])
                .output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn sixteen_digit_number() {
        assert_eq!(
            Intcode::from("1102,34915192,34915192,7,4,7,99,0")
                .input(vec![])
                .output,
            vec![1219070632396864]
        );
    }

    #[test]
    fn large_number() {
        assert_eq!(
            Intcode::from("104,1125899906842624,99")
                .input(vec![])
                .output,
            vec![1125899906842624]
        );
    }
}
