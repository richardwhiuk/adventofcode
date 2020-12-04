use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result as StdResult;
use log::debug;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(PartialEq)]
enum Valid {
    Invalid,
    Present,
    Valid,
}

impl std::default::Default for Valid {
    fn default() -> Self {
        Valid::Invalid
    }
}

impl Valid {
    fn present(&self) -> bool {
        *self == Valid::Valid || *self == Valid::Present
    }
}

#[derive(Default)]
struct Passport {
    byr: Valid,
    iyr: Valid,
    eyr: Valid,
    hgt: Valid,
    hcl: Valid,
    ecl: Valid,
    pid: Valid,
    cid: Valid,
}

fn valid_year(s: &str, min: u16, max: u16) -> bool {
    let year: StdResult<u16, _> = s.parse();

    match year {
        Ok(year) => year >= min && year <= max,
        Err(_) => false,
    }
}

fn valid_height(s: &str) -> bool {
    if s.ends_with("cm") {
        let cm: StdResult<u16, _> = s[0..s.len() - 2].parse();
        match cm {
            Ok(cm) => cm >= 150 && cm <= 193,
            Err(_) => false,
        }
    } else if s.ends_with("in") {
        let l: StdResult<u16, _> = s[0..s.len() - 2].parse();
        match l {
            Ok(l) => l >= 59 && l <= 76,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn valid_hair(s: &str) -> bool {
    if s.len() != 7 || s.chars().nth(0) != Some('#') {
        false
    } else {
        for c in s[1..].chars() {
            if !((c >= '0' || c <= '9') || (c >= 'a' || c <= 'f')) {
                return false;
            }
        }
        true
    }
}

fn valid_eye(s: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|c| *c == s)
}

fn valid_no(s: &str) -> bool {
    if s.len() != 9 {
        false
    } else {
        s.chars().all(|c| c.is_numeric())
    }
}

impl Passport {
    fn present(&self) -> bool {
        self.byr.present()
            && self.iyr.present()
            && self.eyr.present()
            && self.hgt.present()
            && self.hcl.present()
            && self.ecl.present()
            && self.pid.present()
    }

    fn valid(&self) -> bool {
        if self.byr != Valid::Valid {
            debug!("Invalid byr");
            return false;
        }
        if self.iyr != Valid::Valid {
            debug!("Invalid iyr");
            return false;
        }
        if self.eyr != Valid::Valid {
            debug!("Invalid eyr");
            return false;
        }
        if self.hgt != Valid::Valid {
            debug!("Invalid hgt");
            return false;
        }
        if self.hcl != Valid::Valid {
            debug!("Invalid hcl");
            return false;
        }
        if self.ecl != Valid::Valid {
            debug!("Invalid ecl");
            return false;
        }
        if self.pid != Valid::Valid {
            debug!("Invalid pid");
            return false;
        }
        true
    }

    fn add_data(&mut self, line: String) {
        for data in line.split(" ") {
            let mut values = data.split(":");
            let key = values.next().unwrap();
            let value = values.next().unwrap();

            match key {
                "byr" => {
                    if valid_year(value, 1920, 2002) {
                        self.byr = Valid::Valid
                    } else {
                        self.byr = Valid::Present;
                    }
                }
                "iyr" => {
                    if valid_year(value, 2010, 2020) {
                        self.iyr = Valid::Valid
                    } else {
                        self.iyr = Valid::Present;
                    }
                }
                "eyr" => {
                    if valid_year(value, 2020, 2030) {
                        self.eyr = Valid::Valid
                    } else {
                        self.eyr = Valid::Present;
                    }
                }
                "hgt" => {
                    if valid_height(value) {
                        self.hgt = Valid::Valid
                    } else {
                        self.hgt = Valid::Present;
                    }
                }
                "hcl" => {
                    if valid_hair(value) {
                        self.hcl = Valid::Valid
                    } else {
                        self.hcl = Valid::Present;
                    }
                }
                "ecl" => {
                    if valid_eye(value) {
                        self.ecl = Valid::Valid
                    } else {
                        self.ecl = Valid::Present;
                    }
                }
                "pid" => {
                    if valid_no(value) {
                        self.pid = Valid::Valid
                    } else {
                        self.pid = Valid::Present;
                    }
                }
                "cid" => self.cid = Valid::Valid,
                unk => panic!("Unexpected key: {}", unk),
            };
        }
    }
}

fn main() -> Result<()> {
    a("input/four-test.txt")?;
    b("input/four-test.txt")?;

    a("input/four.txt")?;
    b("input/four.txt")?;

    b("input/four-valid.txt")?;

    Ok(())
}

fn a(path: &str) -> Result<()> {
    let p = read(path, Passport::present)?;

    println!("Result: {}", p.len());

    Ok(())
}

fn b(path: &str) -> Result<()> {
    let p = read(path, |p| {
        if p.valid() {
            debug!("valid");
            true
        } else {
            debug!("invalid");
            false
        }
    })?;

    println!("Result: {}", p.len());

    Ok(())
}

fn read<F>(path: &str, f: F) -> Result<Vec<Passport>>
where
    F: Fn(&Passport) -> bool,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut in_progress: Option<Passport> = None;
    let mut passports = vec![];

    for line in reader.lines() {
        let line = line?;
        in_progress = if let Some(mut passport) = in_progress.take() {
            if line.len() == 0 {
                if f(&passport) {
                    passports.push(passport);
                }
                None
            } else {
                passport.add_data(line);
                Some(passport)
            }
        } else {
            if line.len() != 0 {
                let mut passport = Passport::default();
                passport.add_data(line);
                Some(passport)
            } else {
                None
            }
        }
    }

    if let Some(passport) = in_progress.take() {
        if f(&passport) {
            passports.push(passport);
        }
    }

    Ok(passports)
}
