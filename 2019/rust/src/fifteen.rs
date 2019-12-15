use crate::intcode::*;
use std::io::Write;

use std::collections::{HashMap, HashSet};

type Position = (i32, i32);
type Map = HashMap<Position, Location>;
type Graph = HashMap<Position, Node>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Location {
    Wall,
    Space,
    Oxygen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    fn all() -> Vec<Self> {
        vec![North, East, South, West]
    }

    fn input(&self) -> i64 {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }

    fn from(&self, pos: Position) -> Position {
        match self {
            North => (pos.0, pos.1 - 1),
            South => (pos.0, pos.1 + 1),
            East => (pos.0 + 1, pos.1),
            West => (pos.0 - 1, pos.1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Unknown,
    Wall,
    Space(Vec<Direction>),
}

use Node::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suggestion {
    Move(Direction),
    Done,
}

use Suggestion::*;

fn create_graph(map: &Map, position: Position) -> Graph {
    let mut look_at: Vec<Position> = vec![position];
    let mut looking = HashSet::new();

    let mut graph = HashMap::with_capacity(map.len());

    while !look_at.is_empty() {
        let mut new = vec![];

        for entry in look_at {
            // println!("Looking at {:?}", entry);
            std::io::stdout().flush();
            looking.remove(&entry);

            graph.insert(
                entry,
                match map.get(&entry) {
                    Some(Location::Wall) => Wall,
                    Some(Location::Oxygen) | Some(Location::Space) => {
                        let mut dirs = vec![];

                        for dir in Direction::all() {
                            let npos = dir.from(entry);

                            // println!("Considering {:?} - {:?} from {:?}", dir, npos, entry);

                            if !looking.contains(&npos) && !graph.contains_key(&npos) {
                                new.push(npos);
                                looking.insert(npos);
                                dirs.push(dir);
                            }
                        }

                        Space(dirs)
                    }
                    None => Unknown,
                },
            );
        }

        look_at = new;
    }

    graph
}

fn worth_exploring(graph: &Graph, position: Position) -> u32 {
    match graph.get(&position).expect("Expected entry in graph") {
        Unknown => 1,
        Wall => std::u32::MAX,
        Space(dirs) => {
            let mut worth = std::u32::MAX;

            for dir in dirs {
                let npos = dir.from(position);
                let nworth = worth_exploring(graph, npos);

                if nworth < worth {
                    worth = nworth + 1;
                }
            }

            worth
        }
    }
}

fn next_move(map: &Map, position: Position) -> Suggestion {
    let graph = create_graph(map, position);

    //println!("Graph: {:?}", graph);

    // We should be in a space
    let dirs = Direction::all();
    assert!(if let Some(Space(dirs)) = graph.get(&position) {
        true
    } else {
        false
    });

    let mut best = (std::u32::MAX, North);

    for dir in dirs {
        let worth = worth_exploring(&graph, dir.from(position));

        if worth < best.0 {
            best = (worth, dir);
        }
    }

    if best.0 != std::u32::MAX {
        Move(best.1)
    } else {
        Done
    }
}

trait Print {
    fn print(&self, position: Position);
}

impl Print for Map {
    fn print(&self, position: Position) {
        let mut min = (0, 0);
        let mut max = (0, 0);

        for key in self.keys() {
            if key.0 < min.0 {
                min.0 = key.0;
            }
            if key.1 < min.1 {
                min.1 = key.1;
            }
            if key.0 > max.0 {
                max.0 = key.0;
            }
            if key.1 > max.1 {
                max.1 = key.1;
            }
        }

        for y in (min.1)..=(max.1) {
            print!("#");
            for x in (min.0)..=(max.0) {
                if (x, y) == position {
                    print!("R");
                } else {
                    match self.get(&(x, y)) {
                        Some(Location::Wall) => print!("@"),
                        Some(Location::Space) => print!("."),
                        Some(Location::Oxygen) => print!("O"),
                        None => print!(" "),
                    }
                }
            }
            println!("#");
        }

        for x in (min.0)..=(max.0) {
            print!("#");
        }
        println!("");
    }
}

fn map() -> (Map, Position) {
    let data = std::fs::read_to_string("15.txt").expect("Unable to read file");
    let mut program = Intcode::from(&data).inputc(vec![]);

    let mut map = HashMap::new();
    map.insert((0, 0), Location::Space);
    let mut position = (0, 0);
    let mut output = 0;

    let mut oxygen = None;

    loop {
        map.print(position);

        let suggest = next_move(&map, position);

        println!("Suggested next move: {:?}", suggest);

        match suggest {
            Done => return (map, oxygen.expect("Unable to find oxygen")),
            Move(dir) => {
                let mut vm = program.unwrapc();
                vm.add_input(dir.input());
                program = vm.execute();

                assert_eq!(program.output().len(), output + 1);

                match program.output()[output] {
                    0 => {
                        let wall = dir.from(position);
                        println!("Found wall: {:?}", wall);
                        map.insert(wall, Location::Wall);
                    }
                    1 => {
                        position = dir.from(position);
                        println!("Found space: {:?}", position);
                        map.insert(position, Location::Space);
                    }
                    2 => {
                        position = dir.from(position);
                        println!("Found oxygen: {:?}", position);
                        map.insert(position, Location::Oxygen);
                        oxygen = Some(position);
                    }
                    _ => panic!("Unexpected output"),
                }

                output += 1;
            }
        }
    }
}

fn find_steps_to_oxygen(map: &Map, graph: &Graph, start: Position) -> Option<u32> {
    let mut look_at = vec![start];
    let mut depth = 0;

    while !look_at.is_empty() {
        let mut next = vec![];
        for position in look_at {
            match map.get(&position).expect("Incomplete map provided") {
                Location::Oxygen => return Some(depth),
                _ => match graph.get(&position).expect("Incomplete graph provided") {
                    Unknown => panic!("Incomplete graph provided"),
                    Wall => {}
                    Space(dirs) => {
                        for dir in dirs {
                            next.push(dir.from(position));
                        }
                    }
                },
            }
        }
        depth += 1;
        look_at = next;
    }

    None
}

fn find_max_depth(graph: &Graph, start: Position) -> u32 {
    let mut look_at = vec![start];
    let mut depth = 0;

    while !look_at.is_empty() {
        let mut next = vec![];
        for position in look_at {
            match graph.get(&position).expect("Incomplete graph provided") {
                Unknown => panic!("Incomplete graph provided"),
                Wall => {}
                Space(dirs) => {
                    for dir in dirs {
                        next.push(dir.from(position));
                    }
                }
            }
        }
        depth += 1;
        look_at = next;
    }

    (depth - 1)
}

pub fn run_a() {
    let (map, _) = map();
    map.print((0, 0));
    let graph = create_graph(&map, (0, 0));
    println!(
        "15a: {}",
        find_steps_to_oxygen(&map, &graph, (0, 0)).expect("Unable to find oxygen")
    );
}

pub fn run_b() {
    let (map, oxygen) = map();
    map.print((0, 0));
    let graph = create_graph(&map, oxygen);
    println!("15b: {}", find_max_depth(&graph, oxygen));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        map().print((0, 0));
        assert!(false);
    }
}
