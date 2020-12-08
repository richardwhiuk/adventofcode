use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    a("input/eight-test.txt")?;
    b("input/eight-test.txt")?;

    a("input/eight.txt")?;
    b("input/eight.txt")?;

    Ok(())
}

fn a(path: &str) -> Result<()> {
    let instructions = read(path)?;

    let result = run(&instructions, |_, _| {})?;

    println!("Result: {}", result.1);

    Ok(())
}

fn run<F>(instructions: &Vec<Instruction>, mut f: F) -> Result<(bool, i32)>
where
    F: FnMut(usize, &mut Instruction),
{
    let mut acc = 0;
    let mut ix = 0;
    let mut used = HashSet::new();

    while !used.contains(&ix) && ix < instructions.len() {
        used.insert(ix);

        let mut instruction = instructions[ix].clone();

        f(ix, &mut instruction);

        debug!("{}: {:?} - {}", ix, instruction, acc);

        use Instruction::*;

        match instruction {
            Acc(a) => {
                acc += a;
                ix += 1;
            }
            Jmp(a) => {
                ix = ((ix as i32) + a) as usize;
            }
            Nop => {
                ix += 1;
            }
        }
    }

    Ok((used.contains(&ix), acc))
}

fn b(path: &str) -> Result<()> {
    let instructions = read(path)?;

    let mut result = (true, 0);

    let mut mutated = HashSet::new();

    while result.0 {
        let mut changed = false;
        result = run(&instructions, |ix, instruction| {
            use Instruction::*;
            if !changed {
                match instruction {
                    Jmp(_) => {
                        if !mutated.contains(&ix) {
                            mutated.insert(ix);
                            *instruction = Nop;
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        })?;
    }

    println!("Result: {}", result.1);

    Ok(())
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn read(path: &str) -> Result<Vec<Instruction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = vec![];

    let re = regex::Regex::new(r"(nop|acc|jmp) ([+-])(\d+)")?;

    for line in reader.lines() {
        let line = line?;

        let cap = re
            .captures(&line)
            .unwrap_or_else(|| panic!("Unexpcted instruction: {}", line));
        use Instruction::*;

        match cap.get(1).unwrap().as_str() {
            "nop" => res.push(Nop),
            instr => {
                let positive = cap.get(2).unwrap().as_str() == "+";

                let mut number: i32 = cap.get(3).unwrap().as_str().parse()?;

                if !positive {
                    number = -number;
                }

                match instr {
                    "acc" => res.push(Acc(number)),
                    "jmp" => res.push(Jmp(number)),
                    _ => panic!("Unknown instruction"),
                }
            }
        }
    }

    Ok(res)
}
