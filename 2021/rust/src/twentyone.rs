#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    test("src/twentyone-test.txt");
    test("src/twentyone.txt");
}

fn test(f: &str) {
    let player_re = regex::Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();

    let data = read_to_string(f).unwrap();
    let data: HashMap<u8, u32> = data
        .lines()
        .map(|x| {
            let m = player_re.captures(x).unwrap();
            (
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    let mut p1_pos = data[&1] - 1;
    let mut p2_pos = data[&2] - 1;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die = 0;

    let mut p1_turn = true;

    let mut rolls = 0;

    while p1_score < 1000 && p2_score < 1000 {
        let (pos, score) = if p1_turn {
            // print!("Player 1 ");
            (&mut p1_pos, &mut p1_score)
        } else {
            // print!("Player 2 ");
            (&mut p2_pos, &mut p2_score)
        };

        for _ in 1..=3 {
            // print!("{},", die + 1);
            *pos += die + 1;
            *pos = *pos % 10;
            die += 1;
            die = die % 100;
        }
        *score += *pos + 1;
        rolls += 3;
        p1_turn = !p1_turn;
        // println!(" score: {} pos: {}", score, *pos + 1);
    }

    let loss = if p1_score >= 1000 { p2_score } else { p1_score };
    // println!("Rolls: {}, Loss: {}", rolls, loss);
    println!("A: {}", rolls * loss);
}
