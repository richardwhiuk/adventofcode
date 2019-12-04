pub fn run_a() {
    let mut n = 0;

    for i in 108457..562041 {
        if mini_check(i) {
            n += 1;
        }
    }

    println!("4a: Found {}", n);
}

pub fn run_b() {
    let mut n = 0;

    for i in 108457..562041 {
        if mini_check_b(i) {
            n += 1;
        }
    }

    println!("4b: Found {}", n);
}

#[cfg(test)]
fn check(num: u64, min: u64, max: u64) -> bool {
    if num < min {
        return false;
    }

    if num > max {
        return false;
    }

    mini_check(num)
}

fn mini_check(num: u64) -> bool {
    let mut n = 10;
    let mut double = false;

    let mut left = num;

    while left > 0 {
        let c = left % 10;

        if c == n {
            double = true;
        } else if c > n {
            return false;
        }

        left = left / 10;
        n = c;
    }

    return double;
}

fn mini_check_b(num: u64) -> bool {
    let mut n = 10;
    let mut repeats = 0;
    let mut double = false;

    let mut left = num;

    while left > 0 {
        let c = left % 10;

        if c == n {
            repeats += 1;
        } else if c > n {
            return false;
        } else {
            if repeats == 1 {
                double = true;
            }

            repeats = 0;
        }

        left = left / 10;
        n = c;
    }

    if repeats == 1 {
        double = true;
    }

    return double;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(check(111111, 108457, 562041), true);
        assert_eq!(check(223450, 108457, 562041), false);
        assert_eq!(check(123789, 108457, 562041), false);
    }

    #[test]
    fn check_b() {
        assert_eq!(mini_check_b(112233), true);
        assert_eq!(mini_check_b(123444), false);
        assert_eq!(mini_check_b(111122), true);
    }
}
