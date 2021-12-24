use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    test("src/twelve-test1.txt");
    test("src/twelve-test2.txt");
    test("src/twelve-test3.txt");
    test("src/twelve.txt");
}

fn visit<'a, F, T>(routes: &'a HashMap<&'a str, Vec<&'a str>>, f: F) -> Vec<Vec<&'a str>>
where
    F: Fn(&str, &[&str], &Option<T>) -> Option<T>,
{
    let mut incomplete = vec![(vec!["start"], None)];
    let mut complete = vec![];

    while let Some((path, data)) = incomplete.pop() {
        // println!("Looking at path: {:?}", path);
        let last = path[path.len() - 1];
        if last == "end" {
            complete.push(path);
        } else if let Some(routes) = routes.get(last) {
            for route in routes {
                if let Some(c) = f(route, &path, &data) {
                    // println!("  Looking at: {}", route);
                    let mut np = path.clone();
                    np.push(route);
                    // println!("  Adding path: {:?}", np);
                    incomplete.push((np, Some(c)));
                } else {
                    continue;
                }
            }
        }
    }

    complete
}

fn test(f: &str) {
    let data = read_to_string(f).unwrap();

    let data: Vec<(&str, &str)> = data
        .lines()
        .map(|x| {
            let route: Vec<&str> = x.split('-').collect();
            (route[0], route[1])
        })
        .collect();

    let mut routes: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data {
        routes.entry(line.0).or_insert_with(Vec::new).push(line.1);
        routes.entry(line.1).or_insert_with(Vec::new).push(line.0);
    }

    let complete = visit(&routes, |route, path, _data| {
        if route.chars().next().unwrap().is_lowercase() && path.contains(&route) {
            return None;
        }
        Some(())
    });

    println!("A: {}", complete.len());

    let complete = visit(&routes, |route, path, data| {
        if route == "start" {
            return None;
        }
        if route.chars().next().unwrap().is_lowercase() {
            let c = path.iter().filter(|x| **x == route).count();
            if c > 0 {
                if c > 1 {
                    return None;
                }

                if let Some(data) = data {
                    if *data {
                        return None;
                    } else {
                        return Some(true);
                    }
                }
            }
        }

        if let Some(data) = data {
            Some(*data)
        } else {
            Some(false)
        }
    });

    // println!("Routes: {:?}", complete);

    println!("B: {}", complete.len());
}
