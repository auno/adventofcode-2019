use std::collections::HashMap;
use std::io::{self, BufRead};

type RecipePart = (String, i64);

#[derive(Debug)]
struct Recipe {
    output: RecipePart,
    input: Vec<RecipePart>,
}

fn parse_recipe_part(part: String) -> RecipePart {
    let mut part = part.split(' ');
    let amount = part.next().unwrap().parse().unwrap();
    let element = part.next().unwrap();

    (element.to_string(), amount)
}

fn parse_recipe(line: String) -> (String, Recipe) {
    let mut parts: Vec<RecipePart> = line
        .replace(" => ", ", ")
        .split(",")
        .map(|part| part.trim().to_owned())
        .map(parse_recipe_part)
        .collect();

    let result = parts.pop().unwrap();
    let recipe = Recipe {
        output: result.to_owned(),
        input: parts,
    };

    (result.0, recipe)
}

fn read_recipes() -> HashMap<String, Recipe> {
    let recipes = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .map(parse_recipe)
        .collect();

    recipes
}

struct Factory {
    recipes: HashMap<String, Recipe>,
    consumed: HashMap<String, i64>,
    available: HashMap<String, i64>,
}

impl Factory {
    fn new(mut recipes: HashMap<String, Recipe>) -> Factory {
        let mut consumed: HashMap<String, i64> = HashMap::new();
        let mut available: HashMap<String, i64> = HashMap::new();

        let ore_recipe = Recipe {
            output: (String::from("ORE"), 1),
            input: vec![]
        };
        recipes.insert(String::from("ORE"), ore_recipe);

        for element in recipes.keys() {
            consumed.insert(element.to_owned(), 0);
            available.insert(element.to_owned(), 0);
        }

        Factory {
            recipes,
            consumed,
            available,
        }
    }

    fn produce(&mut self, element: &String, amount: i64) {
        let recipe = self.recipes.get(element).unwrap();
        let recipe_output_amount = recipe.output.1;
        let recipe_multiplier = (amount + recipe_output_amount - 1) / recipe_output_amount;

        for (recipe_element, recipe_amount) in recipe.input.to_owned() {
            let input_available = *self.available.get(&recipe_element).unwrap();
            let input_required = recipe_amount * recipe_multiplier;

            if input_available < input_required {
                let deficit = input_required - input_available;
                self.produce(&recipe_element, deficit);
            }

            let current_available = self.available.get_mut(&recipe_element).unwrap();
            *current_available -= input_required;

            let current_consumed = self.consumed.get_mut(&recipe_element).unwrap();
            *current_consumed += input_required;
        }

        let current_available = self.available.get_mut(element).unwrap();
        *current_available += recipe_multiplier * recipe_output_amount;

    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn lcmv(v: &[i64]) -> i64 {
    match v.len() {
        0 => 1,
        1 => v[0],
        2 => lcm(v[0], v[1]),
        _ => {
            let (head, tail) = v.split_first().unwrap();
            lcm(v[0], lcm(*head, lcmv(tail)))
        }
    }
}

fn main() {
    let fuel = String::from("FUEL");
    let ore = String::from("ORE");

    let mut factory = Factory::new(read_recipes());

    let mut periods: HashMap<String, i64> = HashMap::new();
    factory.produce(&fuel, 1);
    let mut i = 1;

    /* Find "periods" for individual elements availability coming back to zero */
    loop {
        for key in factory.recipes.keys() {
            if periods.contains_key(key) {
                continue;
            }

            let current_available = factory.available.get(key).unwrap().to_owned();
            if current_available == 0 {
                periods.insert(key.to_owned(), i);
            }
        }

        if periods.len() == factory.recipes.len() - 1 {
            break;
        }

        factory.produce(&fuel, 1);
        i += 1;
    }

    /* lcm of the "periods" seems to be a good approximate for the full period */
    let periods: Vec<i64> = periods.values().map(|v| v.to_owned()).collect();
    let period = lcmv(periods.as_slice());

    /* Determine amount of ore consumed in one "period" */
    factory.produce(&fuel, period - i);
    i += period - i;

    let consumed_ore = factory.consumed.get(&ore).unwrap().to_owned();
    let factor = 1_000_000_000_000 / consumed_ore;

    /* Scale all amounts proportionally to get as close to the ore target as possible without going over */
    for key in factory.recipes.keys() {
        let current_consumed = factory.consumed.get(key).unwrap().to_owned();
        factory.consumed.insert(key.to_owned(), current_consumed * factor);

        let current_available = factory.available.get(key).unwrap().to_owned();
        factory.available.insert(key.to_owned(), current_available * factor);
    }

    /* Iterate until just past the target */
    i *= factor;
    while factory.consumed.get(&ore).unwrap() < &1000000000000 {
        factory.produce(&fuel, 1);
        i += 1;
    }

    println!("{}", factory.available.get(&fuel).unwrap() - 1);
}
