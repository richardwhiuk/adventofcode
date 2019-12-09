use crate::intcode::*;

pub fn run_a() {
    let data = std::fs::read_to_string("7.txt").expect("Unable to read file");
    println!("7a: {}", determine_phases(&data, vec![0, 1, 2, 3, 4]).1);
}

pub fn run_b() {
    let data = std::fs::read_to_string("7.txt").expect("Unable to read file");
    println!("7b: {}", determine_phases(&data, vec![5, 6, 7, 8, 9]).1);
}

fn determine_phases(prog: &str, phases: Vec<i64>) -> (Vec<i64>, i64) {
    let mut best = vec![];
    let mut best_score = 0;

    let amplifier = Intcode::from(prog);

    for phases in get_phase_options(phases) {
        let input = run_amplifiers(phases.clone(), &amplifier);

        if input > best_score {
            best = phases.clone();
            best_score = input;
        }
    }

    return (best, best_score);
}

fn run_amplifiers(phases: Vec<i64>, amplifier: &Intcode) -> i64 {
    let mut amplifiers = vec![];
    let mut input = 0;

    for phase in &phases {
        let amplifier = amplifier.clone().inputc(vec![input, *phase]);
        input = amplifier.get_output().pop().expect("No output produced");
        amplifiers.push(amplifier);
    }

    let mut last = input;

    loop {
        let mut new_amps = vec![];

        for amplifier in amplifiers.drain(..) {
            match amplifier {
                Finished(_) => {
                    return last;
                }
                NeedMoreInput(mut amp) => {
                    amp.add_input(input);
                    let amplifier = amp.execute();
                    input = amplifier.get_output().pop().expect("No output generated");
                    new_amps.push(amplifier);
                }
            }
        }

        last = input;
        amplifiers = new_amps;
    }
}

fn get_phase_options(phases: Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = vec![];

    get_phase_option(phases, vec![], &mut result);

    result
}

fn get_phase_option(options: Vec<i64>, start: Vec<i64>, result: &mut Vec<Vec<i64>>) {
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
        assert_eq!(
            determine_phases(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                vec![0, 1, 2, 3, 4]
            ),
            (vec![4, 3, 2, 1, 0], 43210)
        );
    }
}
