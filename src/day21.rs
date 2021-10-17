// --- Day 21: Allergen Assessment ---
// https://adventofcode.com/2020/day/21

use std::collections::HashSet;
use crate::get_lines;

pub type Ingredient = String;
pub type Allergen = String;

#[derive(Eq, PartialEq, Hash)]
pub struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

pub fn get_input() -> (HashSet<Ingredient>, HashSet<Allergen>, HashSet<Food>) {
    let mut ingredients = HashSet::new();
    let mut allergens = HashSet::new();
    let mut foods = HashSet::new();

    for line in get_lines("day-21-input.txt") {
        let mut split = line.split(" (contains ");
        let ingredient_list = split.next().expect("Missing ingredients");
        let allergen_list = split.next().expect("Missing allergens");
        if split.next().is_some() {
            panic!("More components found");
        }
        let mut food_ingredients = Vec::new();
        for ingredient in ingredient_list.split(' ')
            .map(|i| String::from(i)) {
            food_ingredients.push(ingredient.clone());
            ingredients.insert(ingredient);
        }
        let mut food_allergens = Vec::new();
        for allergen in allergen_list.replace(')', "").split(", ").map(|a| String::from(a)) {
            food_allergens.push(allergen.clone());
            allergens.insert(allergen);
        }
        let food = Food {
            ingredients: food_ingredients,
            allergens: food_allergens,
        };
        foods.insert(food);
    }

    (ingredients, allergens, foods)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap, HashSet};
    use crate::day21::{Allergen, get_input, Ingredient};

    #[test]
    fn part1() {
        let (ingredients, _, foods) = get_input();
        let mut allergen_to_food = HashMap::new();
        for food in &foods {
            for allergen in &food.allergens {
                let set = allergen_to_food
                    .entry(allergen)
                    .or_insert_with(HashSet::new);
                set.insert(food);
            }
        }
        // "determine which ingredients can't possibly contain any of the allergens in any food in your list"
        let mut inert_ingredients = ingredients.clone();

        for foods_that_contain_allergen in allergen_to_food.values() {
            let mut ingredients_that_may_contain_allergen = ingredients.clone();

            for food in foods_that_contain_allergen {
                ingredients_that_may_contain_allergen.retain(|ingredient| food.ingredients.contains(ingredient));
            }
            inert_ingredients.retain(|ingredient| !ingredients_that_may_contain_allergen.contains(ingredient));
        }

        // "How many times do any of those ingredients appear?"
        let mut sum = 0usize;
        for food in foods {
            for ingredient in &inert_ingredients {
                if food.ingredients.contains(ingredient) {
                    sum += 1;
                }
            }
        }
        println!("Part 1: {}", sum);
    }

    #[test]
    fn part2() {
        let (ingredients, _, foods) = get_input();
        let mut allergen_to_food = HashMap::new();
        for food in &foods {
            for allergen in &food.allergens {
                let set = allergen_to_food
                    .entry(allergen)
                    .or_insert_with(HashSet::new);
                set.insert(food);
            }
        }
        // "determine which ingredients can't possibly contain any of the allergens in any food in your list"
        let mut inert_ingredients = ingredients.clone(); // FIXME invert this to capture dangerous ingredients
        let mut allergen_to_ingredient = HashMap::new();
        for (allergen, foods) in allergen_to_food {
            let mut ingredients_that_may_contain_allergen = ingredients.clone();
            for food in foods {
                ingredients_that_may_contain_allergen.retain(|ingredient| food.ingredients.contains(ingredient));
            }
            eprintln!("-- {} may be found in {:?}", allergen, ingredients_that_may_contain_allergen);
            inert_ingredients.retain(|ingredient| !ingredients_that_may_contain_allergen.contains(ingredient));
            allergen_to_ingredient.insert(allergen, ingredients_that_may_contain_allergen);
        }

        let mut ingredient_to_allergen = HashMap::new();
        let mut dangerous_ingredients = ingredients.difference(&inert_ingredients)
            .collect::<HashSet<&Ingredient>>();
        while !dangerous_ingredients.is_empty() {
            let mut mapped_ingredients = HashSet::new();
            for dangerous_ingredient in dangerous_ingredients.clone() {
                let mut mapped_allergen = None;
                for (allergen, ingredients) in allergen_to_ingredient.clone() {
                    if ingredients.len() == 1 && ingredients.contains(dangerous_ingredient) {
                        // this is the only ingredient known to contain this allergen
                        ingredient_to_allergen.insert(dangerous_ingredient, allergen);
                        eprintln!("-- mapping {} to {}", dangerous_ingredient, allergen);
                        mapped_allergen = Some(allergen);
                        break;
                    }
                }
                if let Some(allergen_to_remove) = mapped_allergen {
                    allergen_to_ingredient.remove(allergen_to_remove);
                    allergen_to_ingredient.iter_mut().for_each(|(_, ingredients)| {
                        ingredients.remove(dangerous_ingredient);
                    });
                    mapped_ingredients.insert(dangerous_ingredient);
                } else {
                    eprintln!("-- not yet able to map {}", dangerous_ingredient);
                }
            }
            for item in mapped_ingredients {
                dangerous_ingredients.remove(item);
            }
        }
        let result = ingredient_to_allergen.iter()
            .map(|(ingredient, allergen)| (*allergen, *ingredient))
            .collect::<BTreeMap<&Allergen, &Ingredient>>()
            .iter()
            .map(|(_, ingredient)| String::from(*ingredient))
            .collect::<Vec<Ingredient>>()
            .join(",");
        println!("Part 2: {}", result);
    }
}