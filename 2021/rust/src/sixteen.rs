#![allow(dead_code)]
use std::fs::read_to_string;

fn main() {
    test("src/sixteen-test1.txt");
    test("src/sixteen-test2.txt");
    test("src/sixteen-test3.txt");
    test("src/sixteen-test4.txt");
    test("src/sixteen-test5.txt");
    test("src/sixteen-test6.txt");
    test("src/sixteen-test7.txt");
    test("src/sixteen.txt");
}

type Iter<'a> = std::slice::IterMut<'a, char>;

fn indent(x: u32) {
    for _ in 0..x {
        print!(" ");
    }
}

fn take<'a>(data: &mut (&mut Iter<'a>, &mut u32), n: u32) -> u32 {
    let mut r = 0;
    for _ in 0..n {
        r *= 2;
        r += data.0.next().unwrap().to_digit(2).unwrap();
    }
    *data.1 += n;
    r
}

fn parse(data: &mut Iter<'_>, depth: u32) -> (u32, u32, u64) {
    let mut len = 0;
    let mut vt = 0;
    let mut data = (data, &mut len);
    let data = &mut data;
    let version = take(data, 3);
    //indent(depth);
    //println!("Version: {}", version);
    vt += version;
    let ty = take(data, 3);
    //indent(depth);
    //println!("Type: {}", ty);
    let value = if ty == 4 {
        let mut cont = true;
        let mut res: u64 = 0;
        while cont {
            let c = take(data, 1);
            if c == 0 {
                cont = false;
            }
            res *= 16;
            res += take(data, 4) as u64;
        }
        res
    } else {
        let mut values: Vec<u64> = vec![];
        let ltid = take(data, 1);
        //indent(depth);
        //println!("Length Type ID: {}", ltid);
        if ltid == 0 {
            let mut total_length = take(data, 15);
            //indent(depth);
            //println!("Length: {}", total_length);
            while total_length > 0 {
                //indent(depth + 1);
                //println!("Subpacket");
                let p = parse(data.0, depth + 2);
                total_length -= p.0;
                *data.1 += p.0;
                vt += p.1;
                values.push(p.2);
            }
        } else {
            let number = take(data, 11);

            for _i in 1..=number {
                //indent(depth + 1);
                //println!("Subpacket: {}", _i);
                let p = parse(data.0, depth + 2);
                *data.1 += p.0;
                vt += p.1;
                values.push(p.2);
            }
        }

        match ty {
            0 => values.into_iter().sum(),
            1 => values.into_iter().product(),
            2 => values.into_iter().min().unwrap(),
            3 => values.into_iter().max().unwrap(),
            5 => {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    };
    //indent(depth);
    //println!("Value: {}", value);

    (len, vt, value)
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    let data = data.trim();
    println!("{}", data);
    let mut data: Vec<char> = data
        .chars()
        .map(|x| {
            format!("{:04b}", x.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    // println!("{:?}", data);

    let packet = parse(&mut data.iter_mut(), 0);
    println!("A: {}", packet.1);
    println!("B: {}", packet.2);
}
