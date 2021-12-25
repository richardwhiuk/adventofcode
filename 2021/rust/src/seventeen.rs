#![allow(dead_code)]
use std::fs::read_to_string;

fn main() {
    test("src/seventeen-test.txt");
    test("src/seventeen.txt");
}

fn attempt_y(mut y_vel: i64, min_y: i64, max_y: i64) -> Option<i64> {
    let mut highest_y = 0;
    let mut y_pos = 0;
    let mut hit = false;
    loop {
        if y_pos > highest_y {
            highest_y = y_pos;
        }
        if y_vel < 0 && y_pos < min_y {
            if hit {
                return Some(highest_y);
            } else {
                return None;
            }
        } else if y_pos >= min_y && y_pos <= max_y {
            hit = true;
        }
        y_pos += y_vel;
        y_vel -= 1;
    }
}

fn attempt_x(mut x_vel: i64, min_x: i64, max_x: i64) -> bool {
    let mut x_pos = 0;
    loop {
        if x_pos >= min_x && x_pos <= max_x {
            return true;
        }
        if x_pos > max_x {
            return false;
        }
        if x_vel == 0 {
            return false;
        }
        x_pos += x_vel;
        x_vel -= 1;
    }
}

fn attempt(mut x_vel: i64, mut y_vel: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;
    // print!(" ");
    loop {
        // print!(" ({},{})", x_pos, y_pos);
        if x_pos >= min_x && x_pos <= max_x && y_pos >= min_y && y_pos <= max_y {
            // println!(" - Success");
            return true;
        }
        if x_pos > max_x && x_vel >= 0 {
            // println!(" - Outside x bounds");
            return false;
        }
        if y_pos < min_y && y_vel <= 0 {
            // println!(" - Outside y bounds");
            return false;
        }
        x_pos += x_vel;
        y_pos += y_vel;
        if x_vel != 0 {
            x_vel -= 1;
        }
        y_vel -= 1;
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    // target area: x=138..184, y=-125..-71
    let re = regex::Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let data = re.captures(&data).unwrap();
    let lx = data.get(1).unwrap().as_str().parse().unwrap();
    let rx = data.get(2).unwrap().as_str().parse().unwrap();
    let uy = data.get(3).unwrap().as_str().parse().unwrap();
    let dy = data.get(4).unwrap().as_str().parse().unwrap();

    let mut max = 0;
    let mut ys = vec![];
    for yvel in -500..500 {
        let attempt = attempt_y(yvel, uy, dy);
        if let Some(attempt) = attempt {
            ys.push(yvel);
            if attempt > max {
                max = attempt;
            }
        }
    }
    println!("A: {:?}", max);

    let mut xs = vec![];
    for xvel in 0..500 {
        let attempt = attempt_x(xvel, lx, rx);
        if attempt {
            xs.push(xvel);
        }
    }

    let mut count = 0;
    for x in xs {
        for y in &ys {
            // println!("Trying {}, {}", x, y);
            if attempt(x, *y, lx, rx, uy, dy) {
                count += 1;
                // println!("P: {},{}", x, y);
            }
        }
    }

    println!("B: {}", count);
}
