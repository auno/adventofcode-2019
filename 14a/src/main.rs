use std::collections::HashMap;
use std::io::{self, BufRead};

type RecipePart = (String, i32);

#[derive(Debug)]
struct Recipe {
    result: RecipePart,
    ingredients: Vec<RecipePart>,
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
        result: result.to_owned(),
        ingredients: parts,
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
    consumed: HashMap<String, i32>,
    available: HashMap<String, i32>,
}

impl Factory {
    fn new(mut recipes: HashMap<String, Recipe>) -> Factory {
        let mut consumed: HashMap<String, i32> = HashMap::new();
        let mut available: HashMap<String, i32> = HashMap::new();

        let ore_recipe = Recipe {
            result: (String::from("ORE"), 1),
            ingredients: vec![]
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

    fn produce(&mut self, element: &String) {
        let recipe = self.recipes.get(element).unwrap();
        let (_, result_amount) = recipe.result.to_owned();

        for (ingredient, ingredient_amount) in recipe.ingredients.to_owned() {
            self.consume(&ingredient, &ingredient_amount);
        }

        let current_amount = self.available.get(element).unwrap().to_owned();
        self.available.insert(element.to_owned(), current_amount + result_amount);
    }

    fn consume(&mut self, element: &String, amount: &i32) {
        while self.available.get(element).unwrap() < amount {
            self.produce(&element);
        }

        let current_consumed = self.consumed.get(element).unwrap().to_owned();
        self.consumed.insert(element.to_owned(), current_consumed + *amount);

        let current_available = self.available.get(element).unwrap().to_owned();
        self.available.insert(element.to_owned(), current_available - *amount);
    }
}

fn main() {
    let mut factory = Factory::new(read_recipes());
    factory.consume(&String::from("FUEL"), &1);

    let consumed_ore = factory.consumed.get(&String::from("ORE")).unwrap();
    println!("{}", consumed_ore);
}