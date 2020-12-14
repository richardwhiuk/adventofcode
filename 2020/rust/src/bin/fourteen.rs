pub use rust::*;

fn main() -> Result<()> {
    // 1A
    a("input/fourteen-test.txt")?;
    a("input/fourteen.txt")?;

    // 1B
    b("input/fourteen-test2.txt")?;
    b("input/fourteen.txt")?;

    Ok(())
}

#[derive(Debug)]
enum Mask {
    One,
    Zero,
    Allow,
}

enum Instruction {
    Mask(Vec<Mask>),
    Set { address: u64, value: u64 },
}

fn read(path: &str) -> Result<Vec<Instruction>> {
    let mut res = vec![];

    let r = regex::Regex::new(r"(?:mask = ([X01]+)|mem\[(\d+)\] = (\d+))").unwrap();

    reader(path, |line| {
        let r = r.captures(&line).unwrap_or_else(|| panic!("Failed to match {}", line));

        res.push(if let Some(mask) = r.get(1) {
            use Mask::*;
            Instruction::Mask(
                mask.as_str()
                    .chars()
                    .rev()
                    .map(|c| match c {
                        'X' => Allow,
                        '1' => One,
                        '0' => Zero,
                        c => panic!("Unhittable char: {}", c),
                    })
                    .collect(),
            )
        } else {
            Instruction::Set {
                address: r.get(2).unwrap().as_str().parse().unwrap(),
                value: r.get(3).unwrap().as_str().parse().unwrap(),
            }
        });

        Ok(())
    })?;

    Ok(res)
}

fn a(path: &str) -> Result<()> {
    let instructions = read(path)?;

    let mut memory : HashMap<u64, u64> = HashMap::new();
    let mut bitmask = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                bitmask = mask;
            }
            Instruction::Set { address, mut value } => {
                debug!("Assign {} to {}", address, value);
                let mut mask: u64 = 1;
                for bit in &bitmask {
                    use Mask::*;
                    match bit {
                        Allow => {}
                        One => {
                            value |= mask;
                        }
                        Zero => {
                            value &= !mask;
                        }
                    }
                    mask <<= 1;
                }
                debug!("Setting {} to {}", address, value);
                memory.insert(address,value);
            }
        }
    }

    println!("A Result: {:?}", memory.values().fold(0, |c, v| c + v));

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let instructions = read(path)?;

    let mut memory : HashMap<u64, u64> = HashMap::new();
    let mut bitmask = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                bitmask = mask;
            }
            Instruction::Set { address, value } => {
                debug!("Assign {} to {}", address, value);
                let mut address = vec![address];
                let mut mask: u64 = 1;
                for (i, bit) in bitmask.iter().enumerate() {
                    debug!("Setting bit {} - {:?} - {}", i, bit, address.len());
                    use Mask::*;
                    match bit {
                        Allow => {
                            address = address.into_iter().map(|v| {
                               vec![v | mask, v & (!mask)]
                            }).flatten().collect();
                        }
                        One => {
                            address.iter_mut().for_each(|v| *v |= mask)
                        }
                        Zero => {}
                    }
                    mask <<= 1;
                }
                for address in address {
                    debug!("Setting {} to {}", address, value);
                    memory.insert(address,value);
                }
            }
        }
    }

    println!("B Result: {:?}", memory.values().fold(0, |c, v| c + v));


    Ok(())
}
