// --- Day 21: Allergen Assessment ---
// https://adventofcode.com/2020/day/21

use crate::get_lines;
use std::collections::{BTreeSet, HashMap, HashSet};

/// A substance used in a food. It may contain 0 or 1 _Allergen_.
pub type Ingredient = String;

/// A substance that may be harmful to some individuals. It is found in exactly one _Ingredient_.
pub type Allergen = String;

/// A food item you are considering taking on your journey
#[derive(Eq, PartialEq, Hash)]
pub struct Food {
    /// A comprehensive list of the ingredients used in this food, listed in a language you do not
    /// understand.
    ingredient_ids: Vec<usize>,

    /// Some or all of the allergens contained in this food, listed in a language you understand.
    /// Some allergens may be omitted.
    allergen_ids: Vec<usize>,
}

/// Read the puzzle input
///
/// Returns:
/// - the unique set of ingredients that can appear in any food
/// - all of the potential food items
pub fn get_input() -> (Vec<Ingredient>, Vec<Allergen>, HashSet<Food>) {
    let mut all_ingredients = BTreeSet::new();
    let mut all_allergens = BTreeSet::new();
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
            allergens.push(allergen.clone());
            all_allergens.insert(allergen.clone());
        }
        let food = (ingredients, allergens);
        foods.insert(food);
    }
    let mut ingredient_map = HashMap::new();
    let mut allergen_map = HashMap::new();
    let mut ingredients = Vec::with_capacity(all_ingredients.len());
    let mut allergens = Vec::with_capacity(all_allergens.len());
    for (index, ingredient) in all_ingredients.iter().enumerate() {
        ingredients.push(ingredient.clone()); // a "drain" operation would be ideal
        ingredient_map.insert(ingredient, index);
    }
    for (index, allergen) in all_allergens.iter().enumerate() {
        allergens.push(allergen.clone()); // a "drain" operation would be ideal
        allergen_map.insert(allergen, index);
    }
    let foods = foods
        .iter()
        .map(|(ingredients, allergens)| -> Food {
            Food {
                ingredient_ids: ingredients
                    .iter()
                    .map(|ingredient| ingredient_map[ingredient])
                    .collect(),
                allergen_ids: allergens
                    .iter()
                    .map(|allergen| allergen_map[allergen])
                    .collect(),
            }
        })
        .collect();

    (ingredients, allergens, foods)
}

#[cfg(test)]
mod tests {
    use crate::day21::{get_input, Food, Ingredient};
    use std::collections::{BTreeMap, HashMap, HashSet};

    #[test]
    fn part1() {
        let (ingredients, allergens, foods) = get_input();
        let mut allergen_to_food = (0..allergens.len())
            .map(|_| HashSet::new())
            .collect::<Vec<HashSet<&Food>>>();
        for food in &foods {
            for allergen_id in &food.allergen_ids {
                allergen_to_food[*allergen_id].insert(food);
            }
        }
        // "determine which ingredients can't possibly contain any of the allergens in any food in your list"
        let mut inert_ingredient_ids = (0..ingredients.len()).collect::<Vec<usize>>();

        for foods_that_contain_allergen in allergen_to_food {
            let mut ingredients_that_may_contain_allergen =
                (0..ingredients.len()).collect::<Vec<usize>>();

            for food in foods_that_contain_allergen {
                ingredients_that_may_contain_allergen
                    .retain(|ingredient_id| food.ingredient_ids.contains(ingredient_id));
            }
            inert_ingredient_ids.retain(|ingredient_id| {
                !ingredients_that_may_contain_allergen.contains(ingredient_id)
            });
        }

        // "How many times do any of those ingredients appear?"
        let mut sum = 0usize;
        for food in foods {
            for ingredient_id in &inert_ingredient_ids {
                if food.ingredient_ids.contains(ingredient_id) {
                    sum += 1;
                }
            }
        }
        println!("Part 1: {}", sum);
    }

    #[test]
    fn part2() {
        let (ingredients, allergens, foods) = get_input();
        let mut allergen_to_food = (0..allergens.len())
            .map(|_| HashSet::new())
            .collect::<Vec<HashSet<&Food>>>();
        for food in &foods {
            for allergen_id in &food.allergen_ids {
                allergen_to_food[*allergen_id].insert(food);
            }
        }
        // "determine which ingredients can't possibly contain any of the allergens in any food in your list"
        let mut dangerous_ingredients = HashSet::new();
        let mut allergen_to_ingredient = (0..allergens.len())
            .map(|_| HashSet::new())
            .collect::<Vec<HashSet<usize>>>();
        for (allergen_id, foods) in allergen_to_food.iter().enumerate() {
            let mut ingredients_that_may_contain_allergen =
                (0..ingredients.len()).collect::<HashSet<usize>>();
            for food in foods {
                ingredients_that_may_contain_allergen
                    .retain(|ingredient_id| food.ingredient_ids.contains(ingredient_id));
            }
            for dangerous_ingredient in ingredients_that_may_contain_allergen.clone() {
                dangerous_ingredients.insert(dangerous_ingredient);
            }
            allergen_to_ingredient[allergen_id] = ingredients_that_may_contain_allergen;
        }

        let mut ingredient_to_allergen = HashMap::new();
        while !dangerous_ingredients.is_empty() {
            let mut mapped_ingredients = HashSet::new();
            for dangerous_ingredient in dangerous_ingredients.clone() {
                let mut mapped_allergen = None;
                for (allergen_id, ingredients) in allergen_to_ingredient.iter().enumerate() {
                    if ingredients.len() == 1 && ingredients.contains(&dangerous_ingredient) {
                        // this is the only ingredient known to contain this allergen
                        ingredient_to_allergen.insert(dangerous_ingredient, allergen_id);
                        mapped_allergen = Some(allergen_id);
                        break;
                    }
                }
                if let Some(allergen_to_remove) = mapped_allergen {
                    allergen_to_ingredient[allergen_to_remove] = HashSet::with_capacity(0);
                    allergen_to_ingredient.iter_mut().for_each(|ingredients| {
                        ingredients.remove(&dangerous_ingredient);
                    });
                    mapped_ingredients.insert(dangerous_ingredient);
                }
            }
            for item in mapped_ingredients {
                dangerous_ingredients.remove(&item);
            }
        }
        let result = ingredient_to_allergen
            .iter()
            .map(|(ingredient_id, allergen_id)| (*allergen_id, &ingredients[*ingredient_id]))
            .collect::<BTreeMap<usize, &Ingredient>>()
            .iter()
            .map(|(_, ingredient)| String::from(*ingredient))
            .collect::<Vec<Ingredient>>()
            .join(",");
        println!("Part 2: {}", result);
    }
}
