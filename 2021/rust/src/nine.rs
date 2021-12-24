use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    test("src/nine-test.txt");
    test("src/nine.txt");
}

fn with_neighbours<F>(data: &[Vec<u32>], x: usize, y: usize, mut f: F)
where
    F: FnMut(usize, usize, u32) -> bool,
{
    for diff in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
        let ny = (y as isize) + diff.0;
        let nx = (x as isize) + diff.1;
        if ny >= 0 && nx >= 0 {
            let ny = ny as usize;
            let nx = nx as usize;

            if let Some(v) = data.get(ny).and_then(|nline| nline.get(nx)) {
                //println!("  Neighbour: {}", *v);
                if !f(nx, ny, *v) {
                    break;
                }
            }
        }
    }
}

fn with_low_point<F>(data: &[Vec<u32>], mut f: F)
where
    F: FnMut(usize, usize, u32),
{
    for (y, line) in data.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            //println!("Checking: {}", cell);
            let mut low = true;
            with_neighbours(data, x, y, |_x, _y, v| {
                if v <= *cell {
                    low = false;
                    return false;
                }
                true
            });
            if low {
                //println!("Found low point: {}", *cell);
                f(x, y, *cell)
            }
        }
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<Vec<u32>> = data
        .lines()
        .map(|x| {
            x.trim()
                .chars()
                .map(|x| {
                    x.to_digit(10)
                        .unwrap_or_else(|| panic!("Unexpected digit: {}", x))
                })
                .collect()
        })
        .collect();

    let mut total = 0;

    with_low_point(&data, |_x, _y, v| {
        total += 1 + v;
    });

    println!("A: {}", total);

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    with_low_point(&data, |x, y, _v| {
        //println!("Basin with low point: {},{}", x, y);
        let mut basin = HashSet::new();
        let mut search: Vec<(usize, usize)> = vec![(x, y)];

        while let Some((x, y)) = search.pop() {
            // println!("  Searching: {}, {}", x, y);
            if basin.contains(&(x, y)) {
                continue;
            }
            if data[y][x] == 9 {
                continue;
            }
            basin.insert((x, y));
            with_neighbours(&data, x, y, |x, y, _v| {
                // println!("    Got neighbour: {}, {}", x, y);
                search.push((x, y));
                true
            });
        }
        // println!("Found basin: {:?}", basin);

        let len = basin.len();
        if len > second {
            third = second;
            if len > first {
                second = first;
                first = len;
            } else {
                second = len;
            }
        } else if len > third {
            third = len;
        }
    });

    println!("B: {}", first * second * third);
}
