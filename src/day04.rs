/// --- Day 4: Camp Cleanup ---
/// https://adventofcode.com/2022/day/4
use std::str::FromStr;

use crate::get_lines;

pub type SectionId = u8;

/// Someone responsible for cleaning a section of the camp
pub struct Elf {
    lower_section_id: SectionId,
    upper_section_id: SectionId,
}

impl Elf {
    /// Determine if this Elf's realm of responsibility fully encompasses that of the other elf
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.lower_section_id <= other.lower_section_id
            && self.upper_section_id >= other.upper_section_id
    }
}

impl FromStr for Elf {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split('-');
        let lower_section_id = components
            .next()
            .ok_or("No section range defined")?
            .parse::<SectionId>()
            .map_err(|_| "Unparseable lower bound")?;
        let upper_section_id = components
            .next()
            .ok_or("Range has no upper bound")?
            .parse::<SectionId>()
            .map_err(|_| "Unparseable upper bound")?;
        if components.next().is_some() {
            return Err("Invalid section range");
        }
        Ok(Self {
            lower_section_id,
            upper_section_id,
        })
    }
}

/// Two crew members responsible for cleaning part of the camp
pub struct Pair(Elf, Elf);

impl Pair {
    /// Identify an inefficiency in which one elf's responsibility fully encompasses the other's
    pub fn one_fully_contains_the_other(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    /// Identity an inefficiency in which there is at least one section for which both elves are
    /// responsible
    pub fn sections_overlap(&self) -> bool {
        (self.0.lower_section_id <= self.1.lower_section_id
            && self.0.upper_section_id >= self.1.lower_section_id)
            || (self.1.lower_section_id <= self.0.lower_section_id
                && self.1.upper_section_id >= self.0.lower_section_id)
    }
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(',');
        let x = components
            .next()
            .ok_or("No pair of elves defined")?
            .parse::<Elf>()?;
        let y = components
            .next()
            .ok_or("Only one elf found")?
            .parse::<Elf>()?;
        if components.next().is_some() {
            return Err("Too many elves specified");
        }
        Ok(Self(x, y))
    }
}

pub fn get_input() -> impl Iterator<Item = Pair> {
    get_lines("day-04.txt")
        .map(|line| line.parse::<Pair>())
        .map(Result::unwrap)
}

#[cfg(test)]
mod tests {

    use crate::day04::{get_input, Pair};

    #[test]
    fn part1() {
        let result = get_input()
            .filter(Pair::one_fully_contains_the_other)
            .count();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let result = get_input().filter(Pair::sections_overlap).count();

        println!("Part 2: {}", result);
    }
}
