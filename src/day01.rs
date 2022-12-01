use crate::get_lines;
use std::cmp::Ordering;

pub fn get_elves(max: usize) -> Vec<Elf> {
    let mut calories_carried = 0usize;
    let mut result = vec![];
    for line in get_lines("day-01.txt") {
        if line.is_empty() {
            let elf = Elf { calories_carried };
            calories_carried = 0;

            let index = match result.binary_search(&elf) {
                Ok(index) => index,
                Err(index) => index,
            };
            result.insert(index, elf);
            if result.len() > max {
                result.remove(result.len() - 1);
            }
        } else {
            calories_carried += line.parse::<usize>().unwrap();
        }
    }
    if calories_carried > 0 {
        let elf = Elf { calories_carried };
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
    calories_carried: usize,
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
    use crate::day01::get_elves;

    #[test]
    fn part1() {
        let elves = get_elves(1);

        println!("Part 1: {}", elves[0].calories_carried);
    }

    #[test]
    fn part2() {
        let elves = get_elves(3);
        let result: usize = elves.iter().map(|elf| elf.calories_carried).sum();

        println!("Part 2: {}", result);
    }
}
