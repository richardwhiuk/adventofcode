pub use rust::*;

fn main() -> Result<()> {
    a("input/sixteen-test.txt")?;
    a("input/sixteen.txt")?;

    b("input/sixteen-test.txt")?;
    b("input/sixteen.txt")?;

    Ok(())
}

type Ticket = Vec<usize>;

type Range = (usize, usize);

struct Data {
    rules: HashMap<String, Vec<Range>>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(PartialEq)]
enum ParseMode {
    ReadingRules,
    ReadingYourTicket,
    ReadingNearbyTickets,
}

fn read(path: &str) -> Result<Data> {
    use ParseMode::*;
    let mut nearby_tickets = vec![];
    let mut your_ticket = None;
    let mut rules = HashMap::new();
    let mut parse_mode = ReadingRules;

    reader(path, |line| {
        if line != "" {
            if line == "your ticket:" {
                parse_mode = ReadingYourTicket;
            } else if line == "nearby tickets:" {
                parse_mode = ReadingNearbyTickets;
            } else if parse_mode == ReadingRules {
                let mut rule = line.split(':');
                let name = rule.next().expect("name of rule").to_owned();
                let ranges = rule
                    .next()
                    .map(|ranges| {
                        ranges
                            .split(" or ")
                            .map(|range| {
                                let mut rit = range.split('-');
                                let min = rit
                                    .next()
                                    .expect("range min")
                                    .trim()
                                    .parse()
                                    .unwrap_or_else(|_| panic!("range min: {}: {}", line, range));
                                let max = rit
                                    .next()
                                    .expect("range max")
                                    .trim()
                                    .parse()
                                    .expect("range max");
                                (min, max)
                            })
                            .collect()
                    })
                    .expect("ranges");
                rules.insert(name, ranges);
            } else {
                let ticket = line
                    .split(',')
                    .map(|v| v.parse().unwrap_or_else(|_| panic!("ticket: {}", line)))
                    .collect();
                if parse_mode == ReadingYourTicket {
                    your_ticket = Some(ticket);
                } else {
                    nearby_tickets.push(ticket);
                }
            }
        }

        Ok(())
    })?;

    Ok(Data {
        rules,
        your_ticket: your_ticket.expect("your ticket"),
        nearby_tickets,
    })
}

fn a(path: &str) -> Result<()> {
    let data = read(path)?;
    let mut sum = 0;

    for ticket in data.nearby_tickets {
        for value in ticket {
            let mut found = false;
            for ranges in data.rules.values() {
                found = in_range(ranges, value);
                if found {
                    break;
                }
            }
            if !found {
                sum += value;
            }
        }
    }

    println!("A Result: {}", sum);

    Ok(())
}

fn in_range(ranges: &Vec<Range>, value: usize) -> bool {
    let mut found = false;
    for range in ranges {
        if value >= range.0 && value <= range.1 {
            found = true;
            break;
        }
    }

    found
}

fn b(path: &str) -> Result<()> {
    let data = read(path)?;

    let mut mapping: Vec<HashSet<&String>> = Vec::with_capacity(data.your_ticket.len());

    for _ in &data.your_ticket {
        mapping.push(data.rules.keys().collect());
    }

    for ticket in data.nearby_tickets {
        let mut valid = true;
        for (_, value) in ticket.iter().enumerate() {
            let mut found = false;
            for (_, ranges) in &data.rules {
                found = in_range(ranges, *value);
                if found {
                    break;
                }
            }
            if !found {
                valid = false;
                break;
            }
        }

        if valid {
            for (i, value) in ticket.iter().enumerate() {
                for rule in mapping[i].clone().iter() {
                    if !in_range(&data.rules[*rule], *value) {
                        mapping[i].remove(rule);
                    }
                }
            }
        }
    }

    let mut converging = true;
    let mut chosen: HashSet<&String> = HashSet::with_capacity(data.rules.len());

    while converging {
        converging = false;

        for rules in &mut mapping {
            if rules.len() == 1 {
                if chosen.insert(rules.iter().next().unwrap()) {
                    converging = true;
                }
            } else {
                for rule in &chosen {
                    if rules.remove(rule) {
                        converging = true;
                    }
                }
            }
        }
    }

    println!("Mapping: {:?}", mapping);

    let mut res: u64 = 1;

    for (i, rule) in mapping.iter().enumerate() {
        if rule.len() != 1 {
            panic!("Too many remaining rules");
        }
        if rule.into_iter().next().unwrap().starts_with("departure") {
            res *= data.your_ticket[i] as u64;
        }
    }

    println!("B Result: {}", res);

    Ok(())
}
