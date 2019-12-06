use crate::intcode::*;

pub fn run_a() {
    let data = std::fs::read_to_string("2.txt").expect("Unable to read file");
    let mut program = Intcode::from(&data);
    program.0[1] = 12;
    program.0[2] = 2;
    let program = program.execute().program;
    println!("2a: program[0] = {}", program[0]);
}

pub fn run_b() {
    let data = std::fs::read_to_string("2.txt").expect("Unable to read file");
    let program = Intcode::from(&data);

    let mut x = 0;
    let mut y = 0;

    loop {
        let mut program = program.clone();

        program.0[1] = x;
        program.0[2] = y;

        let program = program.execute().program;

        if program[0] == 19_690_720 {
            println!(
                "2b: 100 * program[1] + program[2] = {}",
                (100 * program[1]) + program[2]
            );
            return;
        }

        if (x as usize) == (program.len() - 1) {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus() {
        assert_eq!("2,0,0,0,99", Intcode::from("1,0,0,0,99").execute().string());
    }

    #[test]
    fn test_mult() {
        assert_eq!(Intcode::from("2,3,0,3,99").execute().string(), "2,3,0,6,99");
    }

    #[test]
    fn test_mult_b() {
        assert_eq!(
            Intcode::from("2,4,4,5,99,0").execute().string(),
            "2,4,4,5,99,9801"
        );
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            Intcode::from("1,1,1,4,99,5,6,0,99").execute().string(),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
