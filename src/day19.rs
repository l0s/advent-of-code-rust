// --- Day 19: Monster Messages ---
// https://adventofcode.com/2020/day/19

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::day19::Rule::{MatchAll, MatchAnySet, MatchSingleCharacter};
use crate::get_lines;

pub enum Rule {
    MatchSingleCharacter(char),
    MatchAll(Vec<usize>),
    MatchAnySet(Vec<Vec<usize>>),
}

impl Rule {
    pub fn matches(&self, message: String, rules: &HashMap<usize, Rule>) -> bool {
        let mut prefixes = HashSet::new();
        prefixes.insert(message);
        self.matching_suffixes(&prefixes, rules)
            .iter()
            .filter(|suffix| suffix.is_empty()).next().is_some()
    }

    fn matching_suffixes(&self, prefixes: &HashSet<String>, rules: &HashMap<usize, Rule>) -> HashSet<String> { // TODO BTreeSet or Trie?
        match self {
            MatchSingleCharacter(c) => {
                prefixes.iter()
                    .flat_map(|prefix| -> HashSet<String> {
                        let mut result = HashSet::new();
                        if prefix.starts_with(|first| first == *c) {
                            let (_, suffix) = prefix.split_at(1);
                            result.insert(String::from(suffix));
                        }
                        result
                    }).collect()
            }
            MatchAll(ids) => {
                prefixes.iter()
                    .flat_map(|prefix| -> HashSet<String> {
                        let mut result = HashSet::new();
                        result.insert(prefix.to_owned());
                        for id in ids {
                            let rule = rules.get(id).unwrap();
                            let suffixes = rule.matching_suffixes(&result, rules);
                            result = suffixes;
                            if result.is_empty() {
                                break;
                            }
                        }
                        result
                    }).collect()
            }
            MatchAnySet(id_sets) => {
                prefixes.iter()
                    .flat_map(|prefix| -> HashSet<String> {
                        id_sets.iter().flat_map(|set| -> HashSet<String> {
                            let mut result = HashSet::new();
                            result.insert(prefix.to_owned());
                            MatchAll(set.to_owned()).matching_suffixes(&result, rules)
                        }).collect()
                    }).collect()
            }
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            let c = s.strip_prefix('"')
                .map(|s| s.strip_suffix('"'))
                .flatten()
                .map(|s| -> Option<char> {
                    if s.len() != 1 {
                        return None;
                    }
                    s.chars().next()
                })
                .flatten()
                .expect(&*format!("Error parsing character: {}", s)); // TODO return error
            Ok(MatchSingleCharacter(c))
        } else if s.contains(" | ") {
            let potential_rule_sets = s.split(" | ")
                .map(|section| section.trim()
                    .split(' ')
                    .map(|c| c.parse::<usize>().expect(&*format!("Invalid rule ID: {}", c))) // TODO return error
                    .collect::<Vec<usize>>())
                .collect();
            Ok(MatchAnySet(potential_rule_sets))
        } else {
            let rule_ids = s.trim()
                .split(' ')
                .map(|c| c.parse::<usize>().unwrap()) // TODO return error
                .collect::<Vec<usize>>();
            Ok(MatchAll(rule_ids))
        }
    }
}

fn parse_rule(string: &str) -> (usize, Rule) {
    let mut sections = string.trim().split(": ");
    if let Some(id_string) = sections.next() {
        let id = id_string.trim().parse::<usize>().expect(&*format!("Unparseable id: {}", id_string));
        if let Some(value_string) = sections.next() {
            let value_string = value_string.trim();
            if let Ok(rule) = Rule::from_str(value_string) {
                let result = (id, rule);
                assert!(sections.next().is_none(), "Too many sections: {}", string);
                return result;
            }
        }
    }

    panic!("Malformed rule: {}", string);
}

/// Get the puzzle input
///
/// Returns
/// - the rules that valid messages should obey
/// - the received messages
pub fn get_input() -> (HashMap<usize, Rule>, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = vec![];
    let mut section = 0;
    for line in get_lines("day-19-input.txt") {
        if line.is_empty() {
            section += 1;
            continue;
        }
        if section == 0 {
            let (id, rule) = parse_rule(&*line);
            rules.insert(id, rule);
            // eprintln!("-- rule: {}", rule.to_string());
        } else if section == 1 {
            // eprintln!("-- message: {}", line.to_string());
            messages.push(line);
        } else {
            panic!("Unexpected section");
        }
    }
    (rules, messages)
}

#[cfg(test)]
mod tests {
    use crate::day19::{get_input, Rule};

    #[test]
    fn part1() {
        let (rules, messages) = get_input();
        let rule = rules.get(&0usize).unwrap();
        let count = messages.iter()
            .filter(|message| rule.matches(message.to_owned().to_owned(), &rules))
            .count();
        println!("Part 1: {}", count);
    }

    #[test]
    fn part2() {
        let (mut rules, messages) = get_input();
        rules.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
        rules.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());
        let rule = rules.get(&0usize).unwrap();
        let count = messages.iter()
            .filter(|message| rule.matches(message.to_owned().to_owned(), &rules))
            .count();
        println!("Part 2: {}", count);
    }
}