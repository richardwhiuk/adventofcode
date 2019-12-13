use crate::intcode::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlockType {
    Wall,
    Block,
    Paddle,
    Ball,
}

use BlockType::*;

impl BlockType {
    fn from(ty: i64) -> Self {
        match ty {
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            x => panic!("Unexpected block type: {}", x),
        }
    }
}

struct Game(HashMap<(i64, i64), BlockType>);

pub fn run_a() {
    let mut game = Game(HashMap::new());
    let data = std::fs::read_to_string("13.txt").expect("Unable to read file");
    for tile in Intcode::from(&data).execute().output.chunks(3) {
        let x = tile[0];
        let y = tile[1];
        if tile[2] == 0 {
            game.0.remove(&(tile[0], tile[1]));
        } else {
            game.0.insert((tile[0], tile[1]), BlockType::from(tile[2]));
        }
    }

    println!("12a: {}", game.0.values().filter(|b| **b == Block).count());
}
