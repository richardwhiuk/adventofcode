pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use std::collections::{HashMap, HashSet};
pub use std::fs::File;
pub use std::io::{BufRead, BufReader};

pub use log::debug;

pub fn read_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

pub fn reader<F>(path: &str, mut f: F) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    for line in read_lines(path)? {
        let line = line?;

        f(line)?;
    }

    Ok(())
}
