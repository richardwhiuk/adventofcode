#![allow(dead_code)]
use std::fs::read_to_string;

fn main() {
    test("src/thirteen-test.txt");
    test("src/thirteen.txt");
}

fn insert(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    while grid.len() <= y {
        grid.push(vec![]);
    }
    while grid[y].len() <= x {
        grid[y].push(false);
    }
    grid[y][x] = true;
}

fn fold(grid: &mut Vec<Vec<bool>>, axis: &str, line: usize) {
    if axis == "x" {
        for y in 0..(grid.len()) {
            for x in (line + 1)..grid[y].len() {
                if grid[y][x] {
                    insert(grid, (2 * line) - x, y);
                }
            }
            grid[y].resize(line, false);
        }
    } else {
        for y in (line + 1)..(grid.len()) {
            for x in 0..grid[y].len() {
                if grid[y][x] {
                    insert(grid, x, (2 * line) - y);
                }
            }
        }

        grid.resize_with(line, Vec::new);
    }
}

fn print(grid: &[Vec<bool>]) {
    for row in grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<&str> = data.lines().collect();

    let mut points: Vec<(usize, usize)> = vec![];
    let mut folds: Vec<(&str, usize)> = vec![];

    let mut fold_mode = false;

    let fold_re = regex::Regex::new(r"fold along ([x|y])=(\d+)").unwrap();

    for line in data {
        if line.is_empty() {
            fold_mode = true;
        } else if !fold_mode {
            let point: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            points.push((point[0], point[1]));
        } else {
            let fold = fold_re.captures(line).unwrap();
            folds.push((
                fold.get(1).unwrap().as_str(),
                fold.get(2).unwrap().as_str().parse().unwrap(),
            ));
        }
    }

    let mut grid: Vec<Vec<bool>> = vec![];

    for point in points {
        insert(&mut grid, point.0, point.1);
    }

    let mut folds = folds.iter_mut();
    let ffold = folds.next().unwrap();

    fold(&mut grid, ffold.0, ffold.1);

    let mut total = 0;
    for row in &grid {
        for cell in row {
            if *cell {
                total += 1;
            }
        }
    }
    //    print(&grid);
    println!("A: {}", total);

    for f in folds {
        fold(&mut grid, f.0, f.1);
    }

    print(&grid);
}
