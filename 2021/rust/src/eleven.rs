use std::fs::read_to_string;

fn main() {
    test("src/eleven-test.txt");
    test("src/eleven.txt");
}

fn flash(data: &mut Vec<Vec<u32>>) -> usize {
    let mut flashes: Vec<(usize, usize)> = vec![];
    let mut to_flash: Vec<(usize, usize)> = vec![];

    for (y, row) in data.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell += 1;
            if *cell > 9 {
                to_flash.push((x, y));
            }
        }
    }

    while let Some((x, y)) = to_flash.pop() {
        flashes.push((x, y));
        for (dx, dy) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let nx = (x as isize) + dx;
            let ny = (y as isize) + dy;
            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if let Some(cell) = data.get_mut(ny).and_then(|row| row.get_mut(nx)) {
                    *cell += 1;
                    if *cell == 10 {
                        to_flash.push((nx, ny));
                    }
                }
            }
        }
    }

    let flash_count = flashes.len();

    for (x, y) in flashes {
        data[y][x] = 0;
    }

    flash_count
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let mut data: Vec<Vec<u32>> = data
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_flashes = 0;

    for _step in 1..=100 {
        total_flashes += flash(&mut data);
    }

    println!("A: {}", total_flashes);

    let elements = data.len() * data[0].len();

    let mut count = 101;

    while flash(&mut data) != elements {
        count += 1;
    }

    println!("B: {}", count);
}
