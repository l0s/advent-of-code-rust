/// --- Day 1: Calorie Counting ---
/// https://adventofcode.com/2022/day/1
use crate::get_block_strings;
use std::cmp::Ordering;

type CalorieCount = u32;

pub fn get_elves(max: usize) -> Vec<Elf> {
    let mut result = vec![];
    for elf in get_block_strings("day-01.txt")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.parse::<CalorieCount>().expect("Invalid calorie count"))
                .sum()
        })
        .map(|calories_carried| Elf { calories_carried })
    {
        let index = match result.binary_search(&elf) {
            Ok(index) => index,
            Err(index) => index,
        };
        result.insert(index, elf);
        if result.len() > max {
            result.remove(result.len() - 1);
        }
    }
    result
}

#[derive(Debug)]
pub struct Elf {
    calories_carried: CalorieCount,
}

impl Eq for Elf {}

impl PartialEq<Self> for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories_carried == other.calories_carried
    }
}

impl PartialOrd<Self> for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.calories_carried.partial_cmp(&self.calories_carried)
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        other.calories_carried.cmp(&self.calories_carried)
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::{get_elves, CalorieCount};

    #[test]
    fn part1() {
        let elves = get_elves(1);
        let result: CalorieCount = elves.iter().map(|elf| elf.calories_carried).sum();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let elves = get_elves(3);
        let result: CalorieCount = elves.iter().map(|elf| elf.calories_carried).sum();

        println!("Part 2: {}", result);
    }
}
