pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use std::fs::File;
pub use std::io::{BufRead, BufReader};

pub use log::debug;

pub fn reader<F>(path: &str, mut f: F) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        f(line)?;
    }

    Ok(())
}
