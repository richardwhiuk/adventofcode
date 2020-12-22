pub use rust::*;

fn main() -> Result<()> {
    a("input/twenty-test.txt")?;
    a("input/twenty.txt")?;

    b("input/twenty-test.txt")?;
    b("input/twenty.txt")?;

    Ok(())
}

type TileId = u32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Spot {
    Empty,
    Wave,
    Monster,
}

impl std::default::Default for Spot {
    fn default() -> Self {
        Spot::Empty
    }
}

#[derive(Default, Debug, Clone)]
struct Tile(Vec<Vec<Spot>>);

type Puzzle = HashMap<TileId, Tile>;

enum ParseMode {
    Title,
    Lines,
}

fn read(path: &str) -> Result<Puzzle> {
    let mut puzzle = Puzzle::default();
    use ParseMode::*;
    let mut mode = Title;
    let mut tile = None;
    let mut tile_id = None;

    reader(path, |line| {
        if line == "" {
            mode = Title;
            if let Some(tile) = tile.take() {
                puzzle.insert(tile_id.unwrap(), tile);
            }
        } else {
            match mode {
                Title => {
                    let mut t = line.split(" ");
                    let _ = t.next();
                    let t = t.next().unwrap().split(":").next().unwrap();
                    tile_id = Some(t.parse().unwrap());
                    mode = Lines;
                    tile = Some(Tile::default());
                }
                Lines => {
                    use Spot::*;
                    tile.as_mut().map(|tile| {
                        tile.0.push(
                            line.chars()
                                .map(|c| if c == '#' { Wave } else { Empty })
                                .collect(),
                        )
                    });
                }
            }
        }
        Ok(())
    })?;

    if let Some(tile) = tile.take() {
        puzzle.insert(tile_id.unwrap(), tile);
    }

    Ok(puzzle)
}

impl Tile {
    fn top(&self) -> Vec<Spot> {
        self.0[0].clone()
    }
    fn bottom(&self) -> Vec<Spot> {
        self.0.last().unwrap().clone()
    }
    fn left(&self) -> Vec<Spot> {
        self.0.iter().map(|x| x[0]).collect()
    }
    fn right(&self) -> Vec<Spot> {
        self.0.iter().map(|x| *x.last().unwrap()).collect()
    }
    fn orientations(&self) -> Vec<Tile> {
        vec![
            self.clone(),
            self.hflip(),
            self.vflip(),
            self.rotate(1),
            self.rotate(2),
            self.rotate(3),
            self.rotate(1).hflip(),
            self.rotate(1).vflip(),
        ]
    }
    fn hflip(&self) -> Tile {
        Tile(
            self.0
                .iter()
                .map(|c| {
                    let mut c = c.clone();
                    c.reverse();
                    c
                })
                .collect(),
        )
    }
    fn vflip(&self) -> Tile {
        let mut c = self.0.clone();
        c.reverse();
        Tile(c)
    }
    fn rotate(&self, rotations: usize) -> Tile {
        let mut base = self.0.clone();
        for _ in 0..rotations {
            let mut res = vec![];
            for (y, row) in base.iter().enumerate() {
                let mut r = vec![];
                for (x, _) in row.iter().enumerate() {
                    // (y, x
                    r.push(base[row.len() - x - 1][y])
                }
                res.push(r)
            }
            base = res;
        }
        Tile(base)
    }
}

type Point = (i32, i32);

type Grid = HashMap<Point, (TileId, Tile)>;

fn valid_locations(grid: &Grid) -> HashSet<Point> {
    let mut locations = HashSet::new();

    for p in grid.keys() {
        for nloc in &[
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ] {
            if !grid.contains_key(nloc) {
                locations.insert(*nloc);
            }
        }
    }

    locations
}

fn can_insert(tile: &Tile, p: Point, grid: &Grid) -> bool {
    if let Some(left) = grid.get(&(p.0 - 1, p.1)) {
        if left.1.right() != tile.left() {
            return false;
        }
    }
    if let Some(right) = grid.get(&(p.0 + 1, p.1)) {
        if right.1.left() != tile.right() {
            return false;
        }
    }
    if let Some(above) = grid.get(&(p.0, p.1 - 1)) {
        if above.1.bottom() != tile.top() {
            return false;
        }
    }
    if let Some(below) = grid.get(&(p.0, p.1 + 1)) {
        if below.1.top() != tile.bottom() {
            return false;
        }
    }

    true
}

fn add_all(puzzle: &Puzzle, remaining: Vec<TileId>, grid: Grid, indent: u32) -> Option<Grid> {
    if remaining.len() == 0 {
        return Some(grid);
    } else {
        // Try each remaining tile
        for next in &remaining {
            // Try each orientation of that tile
            for orientation in puzzle[next].orientations() {
                // Try each valid location in the grid
                for location in valid_locations(&grid) {
                    // Check if we can insert
                    if can_insert(&orientation, location, &grid) {
                        //                        for _ in 0..indent { print!("    "); }
                        //                        println!("Inserted: }", next);
                        let mut g = grid.clone();
                        g.insert(location, (*next, orientation.clone()));

                        let left = remaining.iter().filter(|x| *x != next).copied().collect();

                        if let Some(grid) = add_all(puzzle, left, g, indent + 1) {
                            return Some(grid);
                        }
                    }
                }
            }
            //            for _ in 0..indent { print!("    "); }
            //            println!("Failed to insert: {}", next);
            //            puzzle[next].print();
            //            println!("Into grid");
            //            print_grid(&grid);
            //            println!("");
        }

        return None;
    }
}

impl Tile {
    fn insert(&mut self, x: usize, y: usize, val: Spot) {
        if self.0.len() <= y {
            self.0.resize_with(y + 1, Default::default);
        }
        if self.0[y].len() <= x {
            self.0[y].resize_with(x + 1, Default::default);
        }
        self.0[y][x] = val;
    }
    fn print(&self) {
        for row in &self.0 {
            for e in row {
                use Spot::*;
                match e {
                    Wave => print!("#"),
                    Empty => print!("."),
                    Monster => print!("O"),
                }
            }
            println!("");
        }
    }
}

fn determine_grid(path: &str) -> Result<Grid> {
    let puzzle = read(path)?;

    let mut grid: Grid = HashMap::new();

    let mut remaining: Vec<TileId> = puzzle.keys().copied().collect();
    let first = remaining.pop().unwrap();
    println!("Starting with: {}", first);
    grid.insert((0, 0), (first, puzzle[&first].clone()));

    Ok(add_all(&puzzle, remaining, grid, 1).expect("Failed to find valid grid"))
}

fn a(path: &str) -> Result<()> {
    let grid = determine_grid(path)?;

    print_grid(&grid);

    let mut product: u64 = 1;
    let (min, max) = det_min_max(&grid);

    for k in &[min, (min.0, max.1), max, (max.0, min.1)] {
        product *= grid.get(k).expect("Missing tile").0 as u64;
    }

    println!("A Result: {}", product);

    Ok(())
}

fn det_min_max(grid: &Grid) -> (Point, Point) {
    let mut min = (0, 0);
    let mut max = (0, 0);
    for n in grid.keys() {
        if n.0 < min.0 {
            min.0 = n.0
        } else if n.0 > max.0 {
            max.0 = n.0
        }
        if n.1 < min.1 {
            min.1 = n.1
        } else if n.1 > max.1 {
            max.1 = n.1
        }
    }

    (min, max)
}

fn print_grid(grid: &Grid) {
    let (min, max) = det_min_max(grid);

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            print!("{:?}\t", grid.get(&(x, y)).map(|(id, _tile)| id));
        }
        println!();
    }
    /*
        for e in grid.values() {
            println!("Tile: {}", e.0);
            e.1.print();
        }
    */
}

/*
 01234567890123456789
0                  #
1#    ##    ##    ###
2 #  #  #  #  #  #
*/

fn flag_sea_monsters(t: &Tile) -> Tile {
    let monster = &[
        (0, 1),
        (1, 2),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        (16, 2),
        (17, 1),
        (18, 0),
        (18, 1),
        (19, 1),
    ];
    let rows = t.0.len();
    let cols = t.0[0].len();

    println!("Map is {}x{}, monster is 3x20", rows, cols);

    // Height 3, so can ignore bottom two rows
    let rows = rows - 2;

    // Length 20, so can ignore last 19 columns
    let cols = cols - 19;

    for mut t in t.orientations() {
        let mut monsters = vec![];

        for by in 0..rows {
            for bx in 0..cols {
                let mut found_monster = true;
                for m in monster {
                    if t.0[by + m.1][bx + m.0] != Spot::Wave {
                        found_monster = false;
                        break;
                    }
                }
                if found_monster {
                    monsters.push((bx, by));
                }
            }
        }

        if monsters.len() > 0 {
            for n in monsters {
                for m in monster {
                    t.0[n.1 + m.1][n.0 + m.0] = Spot::Monster;
                }
            }

            return t;
        }
    }

    panic!("Failed to find any monsters");
}

fn b(path: &str) -> Result<()> {
    println!("Path: {}", path);
    let grid = determine_grid(path)?;

    // Build a giant map tile:
    let mut t = Tile::default();
    let (min, max) = det_min_max(&grid);

    let isz = grid.get(&(0, 0)).unwrap().1 .0.len() as i32;

    let base_y = 0 - min.1;
    let base_x = 0 - min.0;

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let inner = grid.get(&(x, y)).unwrap();
            let by = ((base_y + y) * (isz - 2)) as usize;
            let bx = ((base_x + x) * (isz - 2)) as usize;
            debug!("Placing {},{} at {},{}", x, y, bx, by);
            for iy in 1..(isz - 1) {
                let iy = iy as usize;
                for ix in 1..(isz - 1) {
                    let ix = ix as usize;
                    t.insert(bx + ix - 1, by + iy - 1, inner.1 .0[iy][ix]);
                }
            }
        }
    }

    let t = flag_sea_monsters(&t);

    t.print();
    println!("");

    let res: u32 =
        t.0.iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c == Spot::Wave { 1 as u32 } else { 0 })
                    .sum::<u32>()
            })
            .sum();
    println!("B Result: {}", res);

    Ok(())
}
