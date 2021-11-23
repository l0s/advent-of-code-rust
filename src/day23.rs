// --- Day 23: Crab Cups ---
// https://adventofcode.com/2020/day/22

use crate::get_lines;

type Cup = u8;

pub struct Game {
    circle: Vec<Cup>,
    lowest_label: Cup,
    highest_label: Cup,
    current_index: usize,
}

impl Game {
    fn take(&mut self) -> Cup {
        let mut index = self.current_index + 1;
        if index >= self.circle.len() {
            index = 0;
        }
        let result = self.circle.remove(index);
        if self.current_index >= self.circle.len() {
            self.current_index = self.circle.len() - 1;
        }
        result
    }

    fn get_index(&self, label: Cup) -> usize {
        for (index, cup) in self.circle.iter().enumerate() {
            if *cup == label {
                return index;
            }
        }
        panic!("Cannot find label: {}", label)
    }

    pub fn perform_move(&mut self) {
        let first = self.take();
        let second = self.take();
        let third = self.take();
        let current_cup = self.circle[self.current_index];
        let mut destination_label = current_cup - 1;
        if destination_label < self.lowest_label {
            destination_label = self.highest_label;
        }
        while destination_label == first
            || destination_label == second
            || destination_label == third
        {
            destination_label -= 1;
            if destination_label < self.lowest_label {
                destination_label = self.highest_label;
            }
        }
        let mut destination = self.get_index(destination_label) + 1;
        if destination >= self.circle.len() {
            destination = 0;
        }
        self.circle.insert(destination, third);
        self.circle.insert(destination, second);
        self.circle.insert(destination, first);
        let mut index = self.get_index(current_cup) + 1;
        if index >= self.circle.len() {
            index = 0;
        }
        self.current_index = index;
    }

    pub fn get_cup_order(&self) -> Vec<Cup> {
        let start = self.get_index(1);
        (1..self.circle.len())
            .map(|i| (i + start) % self.circle.len())
            .map(|index| self.circle[index])
            .collect()
    }
}

pub fn get_input() -> Vec<Cup> {
    let mut lines = get_lines("day-23-input.txt");
    let line = lines.next().unwrap();
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day23::{get_input, Game};

    #[test]
    fn part1() {
        let cups = get_input();
        let mut min = u8::MAX;
        let mut max = u8::MIN;
        for cup in &cups {
            min = min.min(*cup);
            max = max.max(*cup);
        }
        let mut game = Game {
            circle: cups,
            lowest_label: min,
            highest_label: max,
            current_index: 0,
        };
        for _ in 1..=100 {
            game.perform_move();
        }
        let order = game.get_cup_order();
        println!("Part 1: {:?}", order);
    }
}
