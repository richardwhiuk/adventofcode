use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run_a() {
    println!("6a: {}", routes_from_file("6.txt"));
}

pub fn run_b() {
    println!("6b: {}", path_len_from_file("6.txt"));
}

fn routes_from_file(path: &str) -> i32 {
    routes(orbits_from_file(path))
}

fn path_len_from_file(path: &str) -> usize {
    path_len(&orbits_from_file(path))
}

fn orbits_from_file(path: &str) -> Vec<(String, String)> {
    BufReader::new(File::open(path).expect("Unable to open file"))
        .lines()
        .map(|s| s.expect("Failed to get line"))
        .map(|s| {
            let parts: Vec<_> = s.split(')').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}

fn path_len(pairs: &Vec<(String, String)>) -> usize {
    let map = orbits(&pairs);

    let mut our_path = determine_path(vec![], "YOU", &map);
    let mut santa_path = determine_path(vec![], "SAN", &map);

    loop {
        if our_path.last() == santa_path.last() {
            our_path.pop();
            santa_path.pop();
        } else {
            break;
        }
    }

    our_path.len() + santa_path.len()
}

fn determine_path(
    mut path: Vec<String>,
    start: &str,
    map: &HashMap<String, String>,
) -> Vec<String> {
    match map.get(start) {
        Some(elem) => {
            path.push(elem.clone());
            determine_path(path, elem, map)
        }
        None => path,
    }
}

fn orbits(pairs: &Vec<(String, String)>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for pair in pairs {
        if let Some(_) = map.insert(pair.1.clone(), pair.0.clone()) {
            panic!("Repeated entry");
        }
    }

    map
}

fn routes(pairs: Vec<(String, String)>) -> i32 {
    let map = orbits(&pairs);

    let mut routes = 0;

    for pair in pairs {
        routes += count_routes(&map, &pair.1);
    }

    return routes;
}

fn count_routes(links: &HashMap<String, String>, key: &String) -> i32 {
    let mut routes = 0;

    if let Some(path) = links.get(key) {
        routes += 1;

        routes += count_routes(&links, path);
    }

    return routes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(routes_from_file("6_test.txt"), 42);
    }

    #[test]
    fn test_b() {
        assert_eq!(path_len_from_file("6_test_2.txt"), 4);
    }
}
