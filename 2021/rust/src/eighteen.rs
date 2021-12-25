#![allow(dead_code)]
use std::fs::read_to_string;

fn main() {
    test_s("[1,2]\n[[3,4],5]");
    test_s("[[[[[9,8],1],2],3],4]");
    test_s("[7,[6,[5,[4,[3,2]]]]]");
    test_s("[[6,[5,[4,[3,2]]]],1]");
    test_s("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
    test_s("[9,1]");
    test_s("[[1,2],[[3,4],5]]");
    //    panic!();
    test("src/eighteen-test.txt");
    test("src/eighteen.txt");
}

#[derive(Debug, Clone)]
struct Pair(Value, Value);

#[derive(Debug)]
enum Explosion {
    No,
    Just(u32, u32),
    ScatterLeft(u32),
    ScatterRight(u32),
    Done,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Pair {
    fn add(self, right: Box<Pair>) -> Pair {
        Pair(Value::Pair(Box::new(self)), Value::Pair(right))
    }

    fn reduce(&mut self) {
        // println!("Before reduction: {}", self);
        loop {
            if self.explode() {
                // println!("Following explode: {}", self);
                continue;
            }
            if self.split() {
                // println!("Following split: {}", self);
                continue;
            }
            return;
        }
    }

    fn explode(&mut self) -> bool {
        use Explosion::*;
        !matches!(self._explode(0), No)
    }

    fn _explode(&mut self, depth: u8) -> Explosion {
        use Explosion::*;
        use Value::*;
        if depth == 4 {
            return Just(self.0.expect_number(), self.1.expect_number());
        }
        let explosion = match &mut self.0 {
            Number(_n) => No,
            Pair(ref mut p) => p._explode(depth + 1),
        };
        match explosion {
            No => {}
            Just(l, r) => {
                self.0 = Value::Number(0);
                self.1.scatter_right(r);
                return ScatterLeft(l);
            }
            ScatterLeft(n) => {
                return ScatterLeft(n);
            }
            ScatterRight(n) => {
                self.1.scatter_right(n);
                return Done;
            }
            Done => {
                return Done;
            }
        }
        let explosion = match &mut self.1 {
            Number(_n) => No,
            Pair(ref mut p) => p._explode(depth + 1),
        };
        match explosion {
            No => No,
            Just(l, r) => {
                self.0.scatter_left(l);
                self.1 = Value::Number(0);
                ScatterRight(r)
            }
            ScatterLeft(n) => {
                self.0.scatter_left(n);
                Done
            }
            ScatterRight(n) => ScatterRight(n),
            Done => Done,
        }
    }

    fn split(&mut self) -> bool {
        for val in [&mut self.0, &mut self.1] {
            let split = match val {
                Value::Pair(p) => {
                    if p.split() {
                        return true;
                    }
                    None
                }
                Value::Number(n) => {
                    if *n >= 10 {
                        Some(*n)
                    } else {
                        None
                    }
                }
            };
            if let Some(n) = split {
                *val = Value::Pair(Box::new(Pair(
                    Value::Number(n / 2),
                    Value::Number((n / 2) + n % 2),
                )));
                return true;
            }
        }
        false
    }

    fn magnitude(&self) -> u32 {
        (self.0.magnitude() * 3) + (self.1.magnitude() * 2)
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(u32),
    Pair(Box<Pair>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Pair(p) => std::fmt::Display::fmt(p, f),
        }
    }
}

impl Value {
    fn parse(input: &mut std::slice::Iter<'_, char>) -> Value {
        let a = input.next().unwrap();

        if *a == '[' {
            // Pair
            let left = Value::parse(input);
            let comma = input.next().unwrap();
            if *comma != ',' {
                panic!("Unexpected {} - expected ,", comma);
            }
            let right = Value::parse(input);
            let end = input.next().unwrap();
            if *end != ']' {
                panic!("Unexpected {} - expected ]", end);
            }
            Value::Pair(Box::new(Pair(left, right)))
        } else {
            Value::Number(a.to_digit(10).unwrap())
        }
    }

    fn expect_number(&self) -> u32 {
        match self {
            Value::Pair(p) => panic!("Expected number, got pair: {:?}", p),
            Value::Number(n) => *n,
        }
    }

    fn expect_pair(self) -> Box<Pair> {
        match self {
            Value::Pair(p) => p,
            Value::Number(n) => panic!("Expected Pair, got number: {}", n),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Value::Pair(p) => p.magnitude(),
            Value::Number(c) => *c,
        }
    }

    // Scatter from the left, to the right most one
    fn scatter_left(&mut self, n: u32) {
        match self {
            Value::Pair(p) => p.1.scatter_left(n),
            Value::Number(c) => {
                *c += n;
            }
        }
    }

    // Scatter from the right, so the left most one
    fn scatter_right(&mut self, n: u32) {
        match self {
            Value::Pair(p) => p.0.scatter_right(n),
            Value::Number(c) => {
                *c += n;
            }
        }
    }
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();
    test_s(&data);
}

fn test_s(data: &str) {
    let mut data: Vec<Box<Pair>> = data
        .lines()
        .map(|x| x.chars().collect())
        .map(|x: Vec<char>| Value::parse(&mut x.iter()).expect_pair())
        .collect();

    for pair in &mut data {
        // println!("Start: {}", pair);
        (*pair).reduce();
    }

    let mut pairs = data.clone().into_iter();
    let mut sum = pairs.next().unwrap();
    sum.reduce();
    for pair in pairs {
        sum = Box::new(sum.add(pair));
        sum.reduce();
    }

    // println!("Pair: {}", sum);
    println!("A: {}", sum.magnitude());

    let mut max = 0;

    for (i, pair_a) in data.iter().enumerate() {
        for (j, pair_b) in data.iter().enumerate() {
            if i != j {
                let mut res = pair_a.clone().add(pair_b.clone());
                res.reduce();
                let v = res.magnitude();
                if v > max {
                    // println!("A: {} B: {} Res: {} Max: {}", pair_a, pair_b, res, v);
                    max = v;
                }
            }
        }
    }

    println!("B: {}", max);

    // println!("-------------------");
}
