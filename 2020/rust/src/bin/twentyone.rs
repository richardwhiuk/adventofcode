pub use rust::*;

fn main() -> Result<()> {
    ab("input/twentyone-test.txt")?;
    ab("input/twentyone.txt")?;

    Ok(())
}

type Puzzle = Vec<Recipe>;

#[derive(Default, Debug)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn read(path: &str) -> Result<Puzzle> {
    let mut puzzle = Puzzle::default();

    reader(path, |line| {
        let mut line = line.split(" (contains ");
        let ingredients = line
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.to_owned())
            .collect();
        let allergens = line
            .next()
            .unwrap()
            .split(")")
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.to_owned())
            .collect();
        puzzle.push(Recipe {
            ingredients,
            allergens,
        });
        Ok(())
    })?;

    Ok(puzzle)
}

fn ab(path: &str) -> Result<()> {
    let puzzle = read(path)?;

    let mut amap: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredients: HashSet<&str> = HashSet::new();

    use std::collections::hash_map::Entry;

    for recipe in &puzzle {
        for allergen in &recipe.allergens {
            match amap.entry(allergen) {
                Entry::Vacant(v) => {
                    v.insert(recipe.ingredients.iter().map(String::as_str).collect());
                }
                Entry::Occupied(mut o) => {
                    let m: HashSet<&str> = recipe.ingredients.iter().map(|x| x.as_str()).collect();
                    let i = o.get().intersection(&m).map(|x| *x).collect();
                    o.insert(i);
                }
            }
        }

        for ingredient in &recipe.ingredients {
            ingredients.insert(ingredient);
        }
    }

    let mut resolved: HashSet<&str> = HashSet::new();
    let mut converging = true;

    while converging {
        converging = false;

        for (_, ingredients) in &mut amap {
            let l = ingredients.len();
            if l == 1 {
                if resolved.insert(ingredients.iter().next().unwrap()) {
                    converging = true;
                }
            } else {
                for ingredient in &resolved {
                    if ingredients.remove(ingredient) {
                        converging = true;
                    }
                }
            }
        }
    }

    let mut allergen_free = ingredients.clone();

    for allergic_ingredients in amap.values() {
        allergen_free = allergen_free
            .difference(allergic_ingredients)
            .map(|x| *x)
            .collect();
    }

    let mut sum = 0;
    for recipe in &puzzle {
        let s: HashSet<&str> = recipe.ingredients.iter().map(|x| x.as_str()).collect();
        sum += s.intersection(&allergen_free).count();
    }

    println!("A Result: {}", sum);

    let mut result: Vec<&str> = amap.keys().copied().collect();
    result.sort();

    let result = result
        .into_iter()
        .map(|i| amap.get(i).unwrap().iter().next().unwrap().clone())
        .collect::<Vec<&str>>()
        .join(",");

    println!("B Result: {}", result);

    Ok(())
}
