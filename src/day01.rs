use crate::get_lines;

pub fn get_elves() -> Vec<Elf> {
    let mut items = vec![];
    let mut result = vec![];
    for line in get_lines("day-01.txt") {
        if line.is_empty() {
            result.push(Elf {
                item_calories: items,
            });
            items = vec![];
        } else {
            items.push(line.parse::<usize>().unwrap());
        }
    }
    if !items.is_empty() {
        result.push(Elf {
            item_calories: items,
        });
    }
    result
}

pub struct Elf {
    item_calories: Vec<usize>,
}

impl Elf {
    pub fn calories_carried(&self) -> usize {
        self.item_calories.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::{get_elves, Elf};

    #[test]
    fn part1() {
        let mut elves = get_elves();
        elves.sort_unstable_by(|x, y| y.calories_carried().cmp(&x.calories_carried()));

        println!("Part 1: {}", elves[0].calories_carried());
    }

    #[test]
    fn part2() {
        let mut elves = get_elves();
        elves.sort_unstable_by(|x, y| y.calories_carried().cmp(&x.calories_carried()));

        let (elves, _) = elves.split_at(3);
        let result: usize = elves.iter().map(Elf::calories_carried).sum();
        println!("Part 2: {}", result);
    }
}
