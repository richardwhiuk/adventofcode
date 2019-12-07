use crate::intcode::Intcode;

pub fn run_a() {
    let data = std::fs::read_to_string("7.txt").expect("Unable to read file");
    println!("7a: {}", determine_phases(&data).1);
}

fn determine_phases(prog: &str) -> (Vec<i32>, i32) {
    let mut best = vec![];
    let mut best_score = 0;

    let amplifier = Intcode::from(prog);

    for phases in get_phase_options() {

        let mut input = 0;

        for phase in &phases {
            let output = amplifier.clone().input(vec![input, *phase]).output;

            input = output[0];
        }

        if input > best_score {
            best = phases.clone();
            best_score = input;
        }
    }

    return (best, best_score)
}

fn get_phase_options() -> Vec<Vec<i32>> {
    let mut result = vec![];
    
    get_phase_option(vec![0,1,2,3,4], vec![], &mut result);

    result
}

fn get_phase_option(options : Vec<i32>, start: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if options.len() == 0 {
        result.push(start);
    } else {
        for option in 0..(options.len()) {
            let mut options = options.clone();
            let mut start = start.clone();
            start.push(options.remove(option));
            get_phase_option(options, start, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(determine_phases("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), (vec![4,3,2,1,0], 43210));
    }
}
