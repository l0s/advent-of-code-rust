use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

use crate::day07::ParseError::{
    BagColourMissing, BagCountMissing, ContainerColourMissing, NothingContained,
};
use crate::get_lines;

pub fn get_input() -> HashMap<String, Rule> {
    get_lines("day-7-input.txt")
        .flat_map(|sentence| sentence.parse::<Rule>())
        .map(|rule| (rule.container_colour.to_owned(), rule))
        .collect()
}

pub struct Rule {
    container_colour: String,
    contained_counts: HashMap<String, u32>,
}

pub enum ParseError {
    ContainerColourMissing,
    NothingContained,
    BagCountMissing,
    BagColourMissing,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(sentence: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG_SUFFIX: Regex = Regex::new(" bag.*$").unwrap();
        }
        let mut components = sentence.splitn(2, " bags contain ");
        let container_colour = components.next();
        if container_colour.is_none() {
            return Err(ContainerColourMissing);
        }
        let container_colour = container_colour.unwrap().trim().to_owned();
        let contained = components.next();
        if contained.is_none() {
            return Err(NothingContained);
        }
        let contained = contained.unwrap().trim();
        if contained.eq("no other bags.") {
            return Ok(Rule {
                container_colour,
                contained_counts: HashMap::new(),
            });
        }
        let contained_counts = contained
            .split(", ")
            .flat_map(|phrase| -> Result<(String, u32), Self::Err> {
                let phrase = BAG_SUFFIX.replace(phrase, "");
                let mut phrase_components = phrase.splitn(2, ' ');

                let count = phrase_components
                    .next()
                    .and_then(|string| string.parse::<u32>().ok());
                if count.is_none() {
                    return Err(BagCountMissing);
                }

                let colour = phrase_components.next();
                if colour.is_none() {
                    return Err(BagColourMissing);
                }

                Ok((colour.unwrap().trim().to_owned(), count.unwrap()))
            })
            .collect::<HashMap<String, u32>>();
        Ok(Rule {
            container_colour,
            contained_counts,
        })
    }
}

impl Rule {
    pub fn can_contain(&self, colour: &str, rule_map: &HashMap<String, Rule>) -> bool {
        if self.contained_counts.contains_key(colour) {
            return true;
        }
        self.contained_counts
            .keys()
            .map(|key| rule_map.get(key))
            .any(|rule| -> bool {
                match rule {
                    None => false,
                    Some(rule) => rule.can_contain(colour, rule_map),
                }
            })
    }

    pub fn count_contained(&self, rule_map: &HashMap<String, Rule>) -> u32 {
        let sum = &self
            .contained_counts
            .iter()
            .flat_map(|(colour, multiplier)| -> Option<u32> {
                rule_map.get(colour).map(|sub_rule| -> u32 {
                    let base = sub_rule.count_contained(rule_map);
                    base * multiplier
                })
            })
            .sum();
        sum + 1u32
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::get_input;

    #[test]
    fn part1() {
        let map = get_input();
        let count = map
            .values()
            .filter(|rule| rule.can_contain("shiny gold", &map))
            .count();
        println!("Part 1: {}", count);
    }

    #[test]
    fn part2() {
        let map = get_input();
        let bag = map.get("shiny gold").unwrap();
        let result = bag.count_contained(&map) - 1;
        println!("Part 2: {}", result);
    }
}
