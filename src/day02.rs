use unicode_segmentation::UnicodeSegmentation;

use crate::get_lines;

fn get_input() -> impl Iterator<Item=String> {
    get_lines("/input/day-2-input.txt")
}

trait Entry {
    fn is_valid(&self) -> bool;
    fn build(line: String) -> Self;
}

struct SledEntry {
    c: String,
    password: String,
    min_iterations: usize,
    max_iterations: usize,
}

impl Entry for SledEntry {
    fn is_valid(&self) -> bool {
        let count: usize = self.password
            .graphemes(true)
            .filter(|grapheme| *grapheme == self.c)
            .count() as usize;
        count >= self.min_iterations && count <= self.max_iterations
    }

    fn build(line: String) -> Self {
        let mut components = line.split_whitespace();
        let range_string = components.next().unwrap();
        let c = components.next().unwrap().replace(":", "");
        let password = components.next().unwrap().to_owned();

        let mut range_components = range_string.split('-')
            .map(|component| component.parse::<usize>())
            .map(Result::unwrap);
        let min_iterations = range_components.next().unwrap();
        let max_iterations = range_components.next().unwrap();

        SledEntry {
            c,
            password,
            min_iterations,
            max_iterations,
        }
    }
}

struct TobogganEntry {
    c: String,
    password: String,
    first_position: usize,
    second_position: usize,
}

impl Entry for TobogganEntry {
    fn is_valid(&self) -> bool {
        let password_chars = &self.password
            .graphemes(true)
            .collect::<Vec<&str>>();
        let x = password_chars[&self.first_position - 1];
        let y = password_chars[&self.second_position - 1];
        (x == &self.c) ^ (y == &self.c)
    }

    fn build(line: String) -> Self {
        let mut components = line.split_whitespace();
        let position_str = components.next().unwrap();
        let c = components.next().unwrap().replace(":", "");
        let password = components.next().unwrap().to_owned();
        let mut position_components = position_str.split('-')
            .map(|component| component.parse::<usize>())
            .map(Result::unwrap);
        let first_position: usize = position_components.next().unwrap();
        let second_position: usize = position_components.next().unwrap();
        TobogganEntry {
            c,
            password,
            first_position,
            second_position,
        }
    }
}

mod tests {
    use crate::day02::{Entry, get_input, SledEntry, TobogganEntry};

    #[test]
    fn part1() {
        let count = get_input().map(SledEntry::build).filter(SledEntry::is_valid).count();
        println!("Part 1: {}", count);
    }

    #[test]
    fn part2() {
        let count = get_input().map(TobogganEntry::build).filter(TobogganEntry::is_valid).count();
        println!("Part 1: {}", count);
    }
}