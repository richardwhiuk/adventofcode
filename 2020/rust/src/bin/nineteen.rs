pub use rust::*;

fn main() -> Result<()> {
    a("input/nineteen-test.txt")?;
    a("input/nineteen.txt")?;

    b("input/nineteen-test.txt")?;
    b("input/nineteen.txt")?;

    Ok(())
}

#[derive(Default)]
struct Puzzle {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

enum RulePart {
    SubRule(usize),
    Character(char),
}

type Rule = Vec<Vec<RulePart>>;

enum PuzzleParse {
    Rules,
    Messages,
}

fn read(path: &str) -> Result<Puzzle> {
    let mut puzzle = Puzzle::default();
    use PuzzleParse::*;
    use RulePart::*;
    let mut mode = Rules;

    reader(path, |line| {
        if line == "" {
            mode = Messages;
        } else {
            match mode {
                Rules => {
                    let mut r = line.split(":");
                    let num: usize = r.next().unwrap().parse().unwrap();
                    let s = r
                        .next()
                        .unwrap()
                        .split("|")
                        .map(|s| {
                            s.trim()
                                .split(" ")
                                .map(|s| {
                                    let s = s.trim();
                                    if s.chars().nth(0).unwrap() == '"' {
                                        Character(s.chars().nth(1).unwrap())
                                    } else {
                                        SubRule(s.parse().expect("SubRule must be valid integer"))
                                    }
                                })
                                .collect()
                        })
                        .collect();
                    puzzle.rules.insert(num, s);
                }
                Messages => {
                    puzzle.messages.push(line);
                }
            }
        }
        Ok(())
    })?;

    Ok(puzzle)
}

impl Puzzle {
    fn check<'a>(&'a self, rule: usize, it: std::str::Chars<'a>) -> Vec<std::str::Chars<'a>> {
        let mut res = vec![];
        for rule_part in self.rules.get(&rule).unwrap() {
            // Take a copy of the  iterator
            let s = it.as_str().chars();

            let mut opts = vec![s];

            for item in rule_part {
                let old_opts = opts;
                opts = vec![];
                for mut opt in old_opts {
                    use RulePart::*;
                    match item {
                        SubRule(r) => opts.extend(self.check(*r, opt)),
                        Character(c) => match opt.next() {
                            Some(d) => {
                                if *c == d {
                                    opts.push(opt);
                                }
                            }
                            None => {}
                        },
                    }
                }
            }

            res.extend(opts);
        }

        res
    }
}

fn a(path: &str) -> Result<()> {
    let puzzle = read(path)?;

    solve(puzzle, "A")
}

fn solve(puzzle: Puzzle, phase: &str) -> Result<()> {
    let mut valid = 0;

    for message in &puzzle.messages {
        let c = message.chars();
        let opts = puzzle.check(0, c);
        for mut opt in opts {
            if opt.next().is_none() {
                valid += 1;
                break;
            }
        }
    }

    println!("{} Result: {}", phase, valid);

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let mut puzzle = read(path)?;

    // Fixup
    use RulePart::SubRule;
    puzzle
        .rules
        .insert(8, vec![vec![SubRule(42)], vec![SubRule(42), SubRule(8)]]);
    puzzle.rules.insert(
        11,
        vec![
            vec![SubRule(42), SubRule(31)],
            vec![SubRule(42), SubRule(11), SubRule(31)],
        ],
    );

    solve(puzzle, "B")
}
