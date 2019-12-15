use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Recipe {
    ingredients: Vec<(u32, String)>,
    amount: u32,
}

impl Recipe {
    fn parse(input: String) -> (String, Self) {
        let parts: Vec<&str> = input.trim().split("=>").collect();
        let outputs: Vec<&str> = parts[1].trim().split(" ").collect();
        let ingredients = parts[0]
            .split(",")
            .map(|s| {
                let parts: Vec<&str> = s.trim().split(" ").collect();
                (
                    parts[0].parse().expect("Expected number"),
                    parts[1].to_string(),
                )
            })
            .collect();
        (
            outputs[1].to_string(),
            Self {
                amount: outputs[0].parse().expect("Expected number"),
                ingredients,
            },
        )
    }
}

pub fn run_a() {
    println!("14a: {}", determine_ore("14.txt"));
}

fn determine_ore(file: &'static str) -> u32 {
    let recipes: HashMap<String, Recipe> =
        BufReader::new(File::open(file).expect("Unable to open file"))
            .lines()
            .map(Result::unwrap)
            .map(Recipe::parse)
            .collect();

    let mut required: HashMap<String, u32> = HashMap::new();
    required.insert("FUEL".to_string(), 1);

    let mut surplus: HashMap<String, u32> = HashMap::new();

    while !complete(&required) {
        let mut next = HashMap::new();

        for requirement in required.iter() {
            if requirement.0 == "ORE" {
                add(&mut next, requirement.0.to_string(), *requirement.1);
            } else {
                // Try getting it from the surplus first
                match surplus.remove(requirement.0) {
                    Some(amount) => {
                        if amount > *requirement.1 {
                            surplus.insert(requirement.0.to_string(), amount - *requirement.1);
                        } else if amount < *requirement.1 {
                            add(
                                &mut next,
                                requirement.0.to_string(),
                                *requirement.1 - amount,
                            );
                        }
                    }
                    None => {
                        // Not available in the surplus
                        let recipe = recipes
                            .get(requirement.0)
                            .expect("Unable to create requirement");

                        let mut num = *requirement.1 / recipe.amount;

                        if (requirement.1 % recipe.amount) != 0 {
                            // Surplus
                            num += 1;
                            surplus.insert(
                                requirement.0.to_string(),
                                (num * recipe.amount) - requirement.1,
                            );
                        }

                        for ingredient in &recipe.ingredients {
                            add(&mut next, ingredient.1.clone(), ingredient.0 * num);
                        }
                    }
                }
            }
        }

        required = next;
    }

    *required.get("ORE").expect("Expect amount of ORE required")
}

fn add(next: &mut HashMap<String, u32>, ingredient: String, amount: u32) {
    let existing = next.entry(ingredient).or_insert(0);
    *existing += amount;
}

fn complete(requirements: &HashMap<String, u32>) -> bool {
    requirements.keys().all(|k| k == "ORE")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(determine_ore("14_test_1.txt"), 31);
        assert_eq!(determine_ore("14_test_2.txt"), 165);
        assert_eq!(determine_ore("14_test_3.txt"), 13312);
        assert_eq!(determine_ore("14_test_4.txt"), 180697);
        assert_eq!(determine_ore("14_test_5.txt"), 2210736);
        assert_eq!(determine_ore("14.txt"), 178154);
    }
}
