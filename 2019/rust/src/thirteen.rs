use crate::intcode::*;
use std::collections::HashMap;
use std::io::Write;

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

struct Game {
    max: (i64, i64),
    map: HashMap<(i64, i64), BlockType>,
    paddle: i64,
    ball: i64,
}

impl Game {
    fn render(&self) {
        for y in 0..=(self.max.1) {
            for x in 0..=(self.max.0) {
                match self.map.get(&(x, y)) {
                    None => print!(" "),
                    Some(Block) => print!("#"),
                    Some(Paddle) => print!("P"),
                    Some(Wall) => print!("@"),
                    Some(Ball) => print!("*"),
                }
            }
            println!("");
        }
    }

    fn insert(&mut self, x: i64, y: i64, block: BlockType) {
        if x > self.max.0 {
            self.max.0 = x;
        }
        if y > self.max.1 {
            self.max.1 = y;
        }
        if block == Ball {
            self.ball = x;
        }
        if block == Paddle {
            self.paddle = x;
        }
        self.map.insert((x, y), block);
    }
}

pub fn run_a() {
    let mut game = Game {
        map: HashMap::new(),
        max: (0, 0),
        ball: 0,
        paddle: 0,
    };
    let data = std::fs::read_to_string("13.txt").expect("Unable to read file");
    for tile in Intcode::from(&data).execute().output.chunks(3) {
        let x = tile[0];
        let y = tile[1];
        if tile[2] == 0 {
            game.map.remove(&(tile[0], tile[1]));
        } else {
            game.insert(tile[0], tile[1], BlockType::from(tile[2]));
        }
    }

    println!(
        "12a: {}",
        game.map.values().filter(|b| **b == Block).count()
    );
}

pub fn run_b() {
    let mut game = Game {
        map: HashMap::new(),
        max: (0, 0),
        ball: 0,
        paddle: 0,
    };
    let data = std::fs::read_to_string("13.txt").expect("Unable to read file");
    let mut ic = Intcode::from(&data);
    ic.0[0] = 2;
    let mut ic = ic.inputc(vec![]);
    let mut offset = 0;
    let mut score = 0;
    loop {
        for tile in ic.output()[offset..].chunks(3) {
            let x = tile[0];
            let y = tile[1];
            if x == -1 && y == 0 {
                score = tile[2];
            } else if tile[2] == 0 {
                game.map.remove(&(tile[0], tile[1]));
            } else {
                game.insert(tile[0], tile[1], BlockType::from(tile[2]));
            }
            offset += 3;
        }
        if game.map.values().filter(|b| **b == Block).count() == 0 {
            println!("Score: {}", score);
            game.render();
            break;
        }
        match ic {
            Finished(_) => {
                break;

                println!("Score: {}", score);
            }
            NeedMoreInput(mut s) => {
                std::io::stdout().flush().unwrap();
                s.add_input(if game.paddle < game.ball {
                    1
                } else if game.paddle > game.ball {
                    -1
                } else {
                    0
                });
                ic = s.execute();
            }
        }
    }
}
