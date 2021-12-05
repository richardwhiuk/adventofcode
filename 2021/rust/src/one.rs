use std::fs::read_to_string;

fn main() {
    test("src/one.txt");
    test("src/one-test.txt");
}

fn test(file: &str) {
    let file = read_to_string(file).expect("failed to load");
    let lines = file
        .lines()
        .map(|l| -> u64 { l.parse().expect("not a number") });
    {
        let mut lines = lines.clone();
        let mut last: u64 = lines.next().unwrap();
        let mut increases = 0;
        for n in lines {
            if n > last {
                increases += 1;
            }
            last = n;
        }

        println!("A: Increases: {}", increases);
    }

    //let mut lines = file.lines();
    let mut window: Vec<u64> = lines.clone().take(3).collect();
    let mut lines: Vec<u64> = lines.clone().skip(1).collect();
    let mut increases = 0;
    loop {
        let prev: u64 = window.iter().sum();
        let next_window = lines.iter().take(3);
        if next_window.clone().count() == 3 {
            let next: u64 = next_window.sum();
            if next > prev {
                increases += 1;
            }
        } else {
            break;
        }
        window = lines.iter().map(|a| *a).take(3).collect();
        lines = lines.into_iter().skip(1).collect();
    }

    println!("B: Increases: {}", increases);
}
