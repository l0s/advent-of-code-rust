// --- Day 21: Allergen Assessment ---
// https://adventofcode.com/2020/day/21

use std::collections::HashSet;
use crate::get_lines;

type Ingredient = String;
type Allergen = String;

#[derive(Eq, PartialEq, Hash)]
pub struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

fn get_input<'i, 'a>() -> (HashSet<Ingredient>, HashSet<Allergen>, HashSet<Food>) {
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
    use std::collections::{HashMap, HashSet};
    use crate::day21::get_input;

    #[test]
    fn part1() {
        let (ingredients, _, foods) = get_input();
        let mut allergen_to_food = HashMap::new();
        for food in &foods {
            for allergen in &food.allergens {
                let set = allergen_to_food
                    .entry(allergen)
                    .or_insert_with(|| HashSet::new());
                set.insert(food);
            }
        }
        // "determine which ingredients can't possibly contain any of the allergens in any food in your list"
        let mut ingredients_without_allergens = ingredients.clone();

        for foods_that_contain_allergen in allergen_to_food.values() {
            let mut ingredients_that_may_contain_allergen = ingredients.clone();

            for food in foods_that_contain_allergen {
                ingredients_that_may_contain_allergen.retain(|ingredient| food.ingredients.contains(ingredient));
            }
            ingredients_without_allergens.retain(|ingredient| !ingredients_that_may_contain_allergen.contains(ingredient));
        }

        // "How many times do any of those ingredients appear?"
        let mut sum = 0usize;
        for food in foods {
            for ingredient in &ingredients_without_allergens {
                if food.ingredients.contains(&ingredient) {
                    sum += 1;
                }
            }
        }
        println!("Part 1: {}", sum);
    }

    #[test]
    fn part2() {

    }
}