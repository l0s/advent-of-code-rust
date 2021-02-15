use std::collections::HashMap;

use regex::Regex;

use crate::get_lines;

fn get_input() -> HashMap<String, Rule> {
    get_lines("/input/day-7-input.txt")
        .map(|sentence| Rule::from_sentence(sentence))
        .flatten()
        .map(|rule| (rule.container_colour.to_owned(), rule))
        .collect()
}

struct Rule {
    container_colour: String,
    contained_counts: HashMap<String, u32>,
}

impl Rule {
    fn from_sentence(sentence: String) -> Option<Rule> {
        lazy_static! {
            static ref RULE_COMPONENT_SEPARATOR: Regex = Regex::new(" bags contain ").unwrap();
            static ref CONTAINED_PHRASE_SEPARATOR: Regex = Regex::new(", ").unwrap();
            static ref BAG_SUFFIX: Regex = Regex::new(" bag.*$").unwrap();
        }
        let mut components = RULE_COMPONENT_SEPARATOR.splitn(&sentence, 2);
        let container_colour = components.next();
        if container_colour.is_none() {
            return None;
        }
        let container_colour = container_colour.unwrap().trim().to_owned();
        let contained = components.next();
        if contained.is_none() {
            return None;
        }
        let contained = contained.unwrap().trim();
        if contained.eq("no other bags.") {
            return Some(Rule { container_colour, contained_counts: HashMap::new() });
        }
        let contained_counts = CONTAINED_PHRASE_SEPARATOR.split(contained)
            .map(|phrase| -> Option<(String, u32)> {
                let phrase = BAG_SUFFIX.replace(phrase, "");
                let mut phrase_components = phrase.splitn(2, ' ');

                let count = phrase_components.next().and_then(|string| string.parse::<u32>().ok());
                if count.is_none() {
                    return None;
                }

                let colour = phrase_components.next();
                if colour.is_none() {
                    return None;
                }

                Some((colour.unwrap().trim().to_owned(), count.unwrap()))
            })
            .flatten()
            .collect::<HashMap<String, u32>>();
        Some(Rule { container_colour, contained_counts })
    }

    fn can_contain(&self, colour: &str, rule_map: &HashMap<String, Rule>) -> bool {
        if (&self).contained_counts.contains_key(colour) {
            return true;
        }
        (&self).contained_counts.keys()
            .map(|key| rule_map.get(key))
            .any(|rule| -> bool {
                match rule {
                    None => false,
                    Some(rule) => rule.can_contain(colour, rule_map)
                }
            })
    }

    fn count_contained(&self, rule_map: &HashMap<String, Rule>) -> u32 {
        let sum = &self.contained_counts.iter()
            .flat_map(|(colour, multiplier)| -> Option<u32> {
                let sub_rule = rule_map.get(colour);
                if sub_rule.is_none() {
                    return None;
                }
                let sub_rule = sub_rule.unwrap();
                let base = sub_rule.count_contained(rule_map);

                Some(base * multiplier)
            }).sum();
        sum + 1u32
    }
}

mod tests {
    use crate::day07::get_input;

    #[test]
    fn part1() {
        let map = get_input();
        let count = map.values()
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