#![allow(dead_code)]
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    test("src/fifteen-test.txt");
    test("src/fifteen.txt");
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<Vec<u32>> = data
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).expect("digit")).collect())
        .collect();

    let lx = data[0].len();
    let ly = data.len();

    println!("A: {}", route(&data, lx - 1, ly - 1));
    println!("B: {}", route(&data, (lx * 5) - 1, (ly * 5) - 1));
    // panic!();
}

fn route(data: &[Vec<u32>], tx: usize, ty: usize) -> i32 {
    let tx = tx as isize;
    let ty = ty as isize;
    // println!("Target: {}, {}", tx, ty);

    let mut paths: BinaryHeap<(i32, Vec<(usize, usize)>)> = BinaryHeap::new();
    paths.push((0, vec![(0, 0)]));

    let mut points = HashMap::new();
    let mx = data[0].len();
    let my = data.len();

    while let Some((risk, path)) = paths.pop() {
        // println!("Checking path: {:?} with risk: {}", path, risk);
        let (x, y) = path[path.len() - 1];
        if (x as isize) == tx && (y as isize) == ty {
            let risk = -risk;
            // println!("Found route: {:?} with risk: {}", path, risk);
            // print(data, tx as usize, ty as usize);
            return risk;
        }

        if let Some(or) = points.get(&(x, y)) {
            if risk >= *or {
                continue;
            }
        }
        points.insert((x, y), risk);

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = (x as isize) + dx;
            let ny = (y as isize) + dy;

            if nx > tx || ny > ty || nx < 0 || ny < 0 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            let nr = get_risk(data, nx, ny, mx, my);

            if let Some(risk) = points.get(&(nx, ny)) {
                if nr >= *risk {
                    continue;
                }
            }
            let mut path = path.clone();
            path.push((nx, ny));
            let rnr = risk - nr;
            let path = (rnr, path);
            // println!("  Adding path: {:?} with risk: {} => {}", path, nr, rnr);
            points.insert((x, y), rnr);

            paths.push(path);
        }
    }

    panic!("Failed to route");
}

fn print(data: &[Vec<u32>], tx: usize, ty: usize) {
    let mx = data[0].len() - 1;
    let my = data.len() - 1;
    for y in 0..=ty {
        for x in 0..=tx {
            print!("{}", get_risk(data, x, y, mx, my));
        }
        println!();
    }
}

fn get_risk(data: &[Vec<u32>], nx: usize, ny: usize, mx: usize, my: usize) -> i32 {
    let mapx = nx % mx;
    let mapy = ny % my;
    let ar = (nx / mx) + (ny / my);
    let nr = data[mapy][mapx] + (ar as u32);
    let nr = ((nr - 1) % 9) + 1;
    nr as i32
}
