use std::fs::read_to_string;

fn main() {
    test("src/ten-test.txt");
    test("src/ten.txt");
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();

    let mut score = 0;
    let mut iscores = vec![];

    for line in data {
        let mut outstanding = vec![];
        let mut corrupt = false;

        for c in line {
            if c == '{' || c == '<' || c == '[' || c == '(' {
                outstanding.push(c);
                continue;
            }

            match (outstanding.pop(), c) {
                (Some('('), ')') => {}
                (Some('['), ']') => {}
                (Some('{'), '}') => {}
                (Some('<'), '>') => {}
                (_, ')') => {
                    score += 3;
                    corrupt = true;
                }
                (_, ']') => {
                    score += 57;
                    corrupt = true;
                }
                (_, '}') => {
                    score += 1197;
                    corrupt = true;
                }
                (_, '>') => {
                    score += 25137;
                    corrupt = true;
                }
                other => panic!("Unexpected pairing: {:?}", other),
            }

            if corrupt {
                break;
            }
        }
        if !corrupt {
            //println!("{:?}", outstanding);
            outstanding.reverse();
            let mut iscore: u64 = 0;
            for c in outstanding {
                iscore *= 5;
                iscore += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    c => panic!("Unexpected char: {}", c),
                };
            }

            iscores.push(iscore);
        }
    }

    println!("A: {}", score);

    iscores.sort_unstable();
    let middle = iscores.len() / 2;
    println!("B: {}", iscores[middle]);
}
