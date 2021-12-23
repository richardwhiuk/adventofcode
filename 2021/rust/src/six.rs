use std::fs::read_to_string;

fn main() {
    test("src/six-test.txt");
    test("src/six.txt");
}

fn day(model: &mut Vec<u64>) {
    let new = model[0];
    for v in 0..=7 {
        model[v] = model[v + 1];
    }
    model[8] = new;
    model[6] += new;
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data: Vec<usize> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut model: Vec<u64> = vec![0; 9];

    for initial in data {
        model[initial] += 1;
    }

    for _day in 1..=80 {
        day(&mut model);
    }

    println!("A: {}", model.iter().cloned().sum::<u64>());

    for _day in 81..=256 {
        day(&mut model);
    }

    println!("A: {}", model.iter().cloned().sum::<u64>());
}
