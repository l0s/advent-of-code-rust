use std::collections::HashSet;
/// --- Day 3: Rucksack Reorganization ---
/// https://adventofcode.com/2022/day/3
use std::str::FromStr;

use crate::get_lines;

/// A container with supplies for a jungle journey. "Each rucksack has two large compartments. All
/// items of a given type are meant to go into exactly one of the two compartments."
pub struct Rucksack {
    items: HashSet<char>,
    compartments: (HashSet<char>, HashSet<char>),
}

impl Rucksack {
    pub fn priority(&self) -> Result<u32, &'static str> {
        let mut intersection = self.compartments.0.intersection(&self.compartments.1);
        if let Some(common_item) = intersection.next() {
            if intersection.next().is_some() {
                return Err("Multiple common items between the compartments");
            }
            return Ok(priority(*common_item));
        }
        Err("No common items between the compartments")
    }
}

/// A value to assist in item reärrangement
fn priority(item: char) -> u32 {
    let item = item as u32;
    if item >= 'a' as u32 && item <= 'z' as u32 {
        item - 'a' as u32 + 1
    } else {
        item - 'A' as u32 + 27
    }
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().collect::<Vec<char>>();
        if items.len() % 2 != 0 {
            return Err("The items cannot be evenly divided between the two compartments");
        }
        let compartments = items.split_at(items.len() / 2);
        let compartments = (
            compartments.0.iter().copied().collect::<HashSet<char>>(),
            compartments.1.iter().copied().collect::<HashSet<char>>(),
        );
        let items = items.iter().copied().collect::<HashSet<char>>();
        Ok(Self {
            items,
            compartments,
        })
    }
}

/// A group of elves' rucksacks. There is exactly one common item shared by each group member.
pub struct Group {
    members: (Rucksack, Rucksack, Rucksack),
}

impl Group {
    /// The priority of shared item
    pub fn badge_priority(&self) -> Result<u32, &'static str> {
        let badge = self.badge()?;
        Ok(priority(badge))
    }
    fn badge(&self) -> Result<char, &'static str> {
        let intersection = self
            .members
            .0
            .items
            .intersection(&self.members.1.items)
            .copied()
            .collect::<HashSet<char>>();
        let mut intersection = intersection.intersection(&self.members.2.items);
        if let Some(badge) = intersection.next() {
            if intersection.next().is_some() {
                return Err("Multiple items in common between members of the group");
            }
            return Ok(*badge);
        }
        Err("No items in common between members of the group")
    }
}

pub fn get_input() -> impl Iterator<Item = Rucksack> {
    get_lines("day-03.txt")
        .map(|line| line.parse::<Rucksack>())
        .map(Result::unwrap)
}

#[cfg(test)]
mod tests {

    use crate::day03::{get_input, Group};

    #[test]
    fn part1() {
        let result: u32 = get_input()
            .map(|rucksack| rucksack.priority().unwrap())
            .sum();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let mut groups = vec![];
        let mut iter = get_input();
        loop {
            if let Some(first) = iter.next() {
                let second = iter.next().expect("Group only has one item");
                let third = iter.next().expect("Group only has two items");
                let members = (first, second, third);
                groups.push(Group { members });
            } else {
                break;
            }
        }
        let result: u32 = groups
            .iter()
            .map(Group::badge_priority)
            .map(Result::unwrap)
            .sum();

        println!("Part 2: {}", result);
    }
}
