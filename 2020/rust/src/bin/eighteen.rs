pub use rust::*;

fn main() -> Result<()> {
    a("input/eighteen-test.txt")?;
    a("input/eighteen.txt")?;

    b("input/eighteen-test.txt")?;
    b("input/eighteen.txt")?;

    Ok(())
}

#[derive(Debug)]
enum ParseToken {
    Number(u64),
    Add,
    Multiply,
    OpenBracket,
    CloseBracket,
}

fn read(path: &str) -> Result<Vec<Vec<ParseToken>>> {
    let mut expression = vec![];

    reader(path, |line| {
        let mut it = line.chars().peekable();

        let mut exp = vec![];
        use ParseToken::*;
        while let Some(x) = it.next() {
            match x {
                '+' => exp.push(Add),
                '*' => exp.push(Multiply),
                '(' => exp.push(OpenBracket),
                ')' => exp.push(CloseBracket),
                c if ('0' <= c && '9' >= c) => {
                    let mut numb = c.to_digit(10).unwrap();

                    loop {
                        match it.peek() {
                            Some(c) if (*c >= '0' && *c <= '9') => {
                                let c = it.next().unwrap();
                                debug!("Found digit: {} - 0: {:?}, 9: {:?}", c, c >= '0', c <= '9');
                                let d = c.to_digit(10).unwrap();
                                numb = (numb * 10) + d;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    exp.push(Number(numb as u64))
                }
                _ => {}
            }
        }
        expression.push(exp);

        Ok(())
    })?;

    Ok(expression)
}

#[derive(Debug)]
enum ParseTokenTree {
    Token(ParseToken),
    Tree(ParseTree),
}

#[derive(Debug)]
enum ParseTree {
    Add(Box<ParseTree>, Box<ParseTree>),
    Multiply(Box<ParseTree>, Box<ParseTree>),
    Number(u64),
}

impl ParseTree {
    fn from(it: &mut impl Iterator<Item = ParseToken>, prec: bool) -> ParseTree {
        let mut res = vec![];

        // Convert brackets
        while let Some(pt) = it.next() {
            match pt {
                ParseToken::OpenBracket => {
                    res.push(ParseTokenTree::Tree(ParseTree::from(it, prec)));
                }
                ParseToken::CloseBracket => {
                    break;
                }
                token => {
                    res.push(ParseTokenTree::Token(token));
                }
            }
        }

        // Precedence: *
        let res = if prec {
            let mut it = res.into_iter();
            let mut res = vec![];

            while let Some(pt) = it.next() {
                match pt {
                    ParseTokenTree::Token(ParseToken::Add) => {
                        let a = match res.pop().unwrap() {
                            ParseTokenTree::Tree(pt) => pt,
                            ParseTokenTree::Token(ParseToken::Number(i)) => ParseTree::Number(i),
                            pt => panic!("Unexpected token: {:?} expected number or brackets", pt),
                        };
                        let b = match it.next().unwrap() {
                            ParseTokenTree::Tree(pt) => pt,
                            ParseTokenTree::Token(ParseToken::Number(i)) => ParseTree::Number(i),
                            pt => panic!("Unexpected token: {:?} expected number or brackets", pt),
                        };
                        res.push(ParseTokenTree::Tree(ParseTree::Add(
                            Box::new(a),
                            Box::new(b),
                        )));
                    }
                    pt => {
                        res.push(pt);
                    }
                }
            }

            res
        } else {
            res
        };

        let mut it = res.into_iter();
        let mut res = vec![];

        // Precedence: +
        while let Some(pt) = it.next() {
            match pt {
                ParseTokenTree::Token(ParseToken::Multiply)
                | ParseTokenTree::Token(ParseToken::Add) => {
                    let a = match res.pop().unwrap() {
                        ParseTokenTree::Tree(pt) => pt,
                        ParseTokenTree::Token(ParseToken::Number(i)) => ParseTree::Number(i),
                        pt => panic!("Unexpected token: {:?} expected number or brackets", pt),
                    };
                    let b = match it.next().unwrap() {
                        ParseTokenTree::Tree(pt) => pt,
                        ParseTokenTree::Token(ParseToken::Number(i)) => ParseTree::Number(i),
                        pt => panic!("Unexpected token: {:?} expected number or brackets", pt),
                    };
                    res.push(ParseTokenTree::Tree(match pt {
                        ParseTokenTree::Token(ParseToken::Multiply) => {
                            ParseTree::Multiply(Box::new(a), Box::new(b))
                        }
                        ParseTokenTree::Token(ParseToken::Add) => {
                            ParseTree::Add(Box::new(a), Box::new(b))
                        }
                        _ => {
                            unreachable!();
                        }
                    }));
                }
                _ => {
                    res.push(pt);
                }
            }
        }

        assert_eq!(res.len(), 1);

        match res.pop().unwrap() {
            ParseTokenTree::Tree(t) => t,
            pt => {
                panic!("Unexpected parse result: {:?}", pt);
            }
        }
    }

    /*
        fn operand(it: &mut impl Iterator<Item=ParseToken>, prec: bool) -> ParseTree {
            match it.next().unwrap() {
                ParseToken::Number(i) => ParseTree::Number(i),
                ParseToken::OpenBracket => ParseTree::from(it, prec),
                pt => {
                    panic!("Unexpected token: {:?}", pt);
                }
            }
        }

        fn from(it: &mut impl Iterator<Item=ParseToken>, prec: bool) -> ParseTree {
            let mut a = ParseTree::operand(it);
            while let Some(pt) = it.next() {
                match pt {
                    ParseToken::Add => {
                        let b = if prec {
                            ParseTree::from(it, prec);
                        } else {
                            ParseTree::operand(it, prec);
                        };

                        a = ParseTree::Add(Box::new(a), Box::new(b))
                    }
                    ParseToken::Multiply => {
                        let b = ParseTree::operand(it);
                        a = ParseTree::Multiply(Box::new(a), Box::new(b))
                    }
                    ParseToken::CloseBracket => {
                        return a;
                    }
                    pt => {
                        panic!("Unexpected token: {:?}", pt);
                    }
                }
            }
            a
        }
    */

    fn evaluate(&self) -> u64 {
        use ParseTree::*;
        match self {
            Number(i) => *i,
            Add(a, b) => a.evaluate() + b.evaluate(),
            Multiply(a, b) => a.evaluate() * b.evaluate(),
        }
    }
}

fn a(path: &str) -> Result<()> {
    puzzle(path, false, "A")
}

fn puzzle(path: &str, prec: bool, phase: &str) -> Result<()> {
    println!("{}::", path);
    println!("");
    let expressions = read(path)?;

    let mut sum = 0;

    for expression in expressions {
        println!("{:?}", expression);

        let res = ParseTree::from(&mut expression.into_iter(), prec);

        println!("{:?}", res);

        let res = res.evaluate();

        println!("=> {}", res);

        sum += res;
    }

    println!("{} Result: {}", phase, sum);
    println!("");

    Ok(())
}

fn b(path: &str) -> Result<()> {
    puzzle(path, true, "B")
}
