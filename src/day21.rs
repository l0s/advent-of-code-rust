// --- Day 21: Allergen Assessment ---
// https://adventofcode.com/2020/day/21

use crate::get_lines;
use std::collections::HashSet;

/// A substance used in a food. It may contain 0 or 1 _Allergen_.
pub type Ingredient = String;

/// A substance that may be harmful to some individuals. It is found in exactly one _Ingredient_.
pub type Allergen = String;

/// A food item you are considering taking on your journey
#[derive(Eq, PartialEq, Hash)]
pub struct Food {
    /// A comprehensive list of the ingredients used in this food, listed in a language you do not
    /// understand.
    ingredients: Vec<Ingredient>,

    /// Some or all of the allergens contained in this food, listed in a language you understand.
    /// Some allergens may be omitted.
    allergens: Vec<Allergen>,
}

/// Read the puzzle input
///
/// Returns:
/// - the unique set of ingredients that can appear in any food
/// - all of the potential food items
pub fn get_input() -> (HashSet<Ingredient>, HashSet<Food>) {
    let mut all_ingredients = HashSet::new();
    let mut foods = HashSet::new();

    for line in get_lines("day-21-input.txt") {
        let mut split = line.split(" (contains ");
        let ingredient_list = split.next().expect("Missing ingredients");
        let allergen_list = split.next().expect("Missing allergens");
        if split.next().is_some() {
            panic!("More components found");
        }
        let mut ingredients = Vec::new();
        for ingredient in ingredient_list.split(' ').map(|i| String::from(i)) {
            ingredients.push(ingredient.clone());
            all_ingredients.insert(ingredient);
        }
        let mut allergens = Vec::new();
        for allergen in allergen_list
            .replace(')', "")
            .split(", ")
            .map(|a| String::from(a))
        {
            allergens.push(allergen);
        }
        let food = Food {
            ingredients,
            allergens,
        };
        foods.insert(food);
    }

    (all_ingredients, foods)
}

#[cfg(test)]
mod tests {
    use crate::day21::{get_input, Allergen, Ingredient};
    use std::collections::{BTreeMap, HashMap, HashSet};

    #[test]
    fn part1() {
        let (ingredients, foods) = get_input();
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
                ingredients_that_may_contain_allergen
                    .retain(|ingredient| food.ingredients.contains(ingredient));
            }
            inert_ingredients
                .retain(|ingredient| !ingredients_that_may_contain_allergen.contains(ingredient));
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
        let (ingredients, foods) = get_input();
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
        let mut dangerous_ingredients = HashSet::new();
        let mut allergen_to_ingredient = HashMap::new();
        for (allergen, foods) in allergen_to_food {
            let mut ingredients_that_may_contain_allergen = ingredients.clone();
            for food in foods {
                ingredients_that_may_contain_allergen
                    .retain(|ingredient| food.ingredients.contains(ingredient));
            }
            for dangerous_ingredient in ingredients_that_may_contain_allergen.clone() {
                dangerous_ingredients.insert(dangerous_ingredient);
            }
            allergen_to_ingredient.insert(allergen, ingredients_that_may_contain_allergen);
        }

        let mut ingredient_to_allergen = HashMap::new();
        while !dangerous_ingredients.is_empty() {
            let mut mapped_ingredients = HashSet::new();
            for dangerous_ingredient in dangerous_ingredients.clone() {
                let mut mapped_allergen = None;
                for (allergen, ingredients) in allergen_to_ingredient.clone() {
                    if ingredients.len() == 1 && ingredients.contains(&dangerous_ingredient) {
                        // this is the only ingredient known to contain this allergen
                        ingredient_to_allergen.insert(dangerous_ingredient.clone(), allergen);
                        mapped_allergen = Some(allergen);
                        break;
                    }
                }
                if let Some(allergen_to_remove) = mapped_allergen {
                    allergen_to_ingredient.remove(allergen_to_remove);
                    allergen_to_ingredient
                        .iter_mut()
                        .for_each(|(_, ingredients)| {
                            ingredients.remove(&dangerous_ingredient);
                        });
                    mapped_ingredients.insert(dangerous_ingredient.clone());
                }
            }
            for item in mapped_ingredients {
                dangerous_ingredients.remove(&item);
            }
        }
        let result = ingredient_to_allergen
            .iter()
            .map(|(ingredient, allergen)| (*allergen, ingredient))
            .collect::<BTreeMap<&Allergen, &Ingredient>>()
            .iter()
            .map(|(_, ingredient)| String::from(*ingredient))
            .collect::<Vec<Ingredient>>()
            .join(",");
        println!("Part 2: {}", result);
    }
}
