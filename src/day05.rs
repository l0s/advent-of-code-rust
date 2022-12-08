/// --- Day 5: ---
/// https://adventofcode.com/2022/day/5
use crate::get_block_strings;
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn get_part1_input() -> (Vec<VecDeque<char>>, Vec<CrateMover9000Instruction>) {
    let mut iterator = get_block_strings("day-05.txt");
    let stacks = iterator.next().expect("Stack specification is missing");
    let stacks = parse_stacks(&stacks);
    let instructions = iterator.next().expect("Instructions missing");
    let instructions = instructions
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<CrateMover9000Instruction>())
        .map(Result::unwrap)
        .collect::<Vec<CrateMover9000Instruction>>();
    (stacks, instructions)
}

pub fn get_part2_input() -> (Vec<VecDeque<char>>, Vec<CrateMover9001Instruction>) {
    let mut iterator = get_block_strings("day-05.txt");
    let stacks = iterator.next().expect("Stack specification is missing");
    let stacks = parse_stacks(&stacks);
    let instructions = iterator.next().expect("Instructions missing");
    let instructions = instructions
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<CrateMover9001Instruction>())
        .map(Result::unwrap)
        .collect::<Vec<CrateMover9001Instruction>>();
    (stacks, instructions)
}

fn parse_stacks(lines: &str) -> Vec<VecDeque<char>> {
    let mut stacks = vec![VecDeque::new(); 9];
    for line in lines.split('\n') {
        let mut stack_index = None;
        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                stack_index = Some(i / 4);
            } else if let Some(index) = stack_index {
                stacks[index].push_back(c);
                stack_index = None;
            }
        }
    }
    stacks
}

pub trait Instruction: FromStr {
    fn execute(&self, stacks: Vec<VecDeque<char>>) -> Vec<VecDeque<char>>;
}

pub struct CrateMover9000Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for CrateMover9000Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split(' ');
        let count = components
            .nth(1)
            .ok_or_else(|| "count not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse count: {}", parse_error))?;
        let from = components
            .nth(1)
            .ok_or_else(|| "Source stack not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse source stack: {}", parse_error))?
            - 1;
        let to = components
            .nth(1)
            .ok_or_else(|| "Destination stack not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse destination stack: {}", parse_error))?
            - 1;
        Ok(Self { count, from, to })
    }
}

impl Instruction for CrateMover9000Instruction {
    fn execute(&self, mut stacks: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
        let s: &mut [VecDeque<char>] = stacks.borrow_mut();
        for _ in 0..self.count {
            let tmp = s[self.from].pop_front();
            s[self.to].push_front(
                tmp.expect("CrateMover attempted to remove an item from an empty stack"),
            );
        }
        s.to_vec()
    }
}

pub struct CrateMover9001Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for CrateMover9001Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split(' ');
        let count = components
            .nth(1)
            .ok_or_else(|| "count not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse count: {}", parse_error))?;
        let from = components
            .nth(1)
            .ok_or_else(|| "Source stack not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse source stack: {}", parse_error))?
            - 1;
        let to = components
            .nth(1)
            .ok_or_else(|| "Destination stack not specified".to_string())?
            .parse::<usize>()
            .map_err(|parse_error| format!("Unable to parse destination stack: {}", parse_error))?
            - 1;
        Ok(Self { count, from, to })
    }
}

impl Instruction for CrateMover9001Instruction {
    fn execute(&self, mut stacks: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
        let mut buffer = VecDeque::with_capacity(self.count);
        for _ in 0..self.count {
            buffer.push_front(
                stacks[self.from]
                    .pop_front()
                    .expect("CrateMover attempted to remove an item from an empty stack"),
            );
        }
        while let Some(item) = buffer.pop_front() {
            stacks[self.to].push_front(item);
        }
        stacks
    }
}

pub fn summarise_stacks(stacks: &Vec<VecDeque<char>>) -> String {
    let mut result = String::new();
    for stack in stacks {
        if let Some(c) = stack.front() {
            result.push(*c);
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use crate::day05::{get_part1_input, get_part2_input, summarise_stacks, Instruction};

    #[test]
    fn part1() {
        let (mut stacks, instructions) = get_part1_input();
        for instruction in instructions {
            stacks = instruction.execute(stacks);
        }
        let result = summarise_stacks(&stacks);

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let (mut stacks, instructions) = get_part2_input();
        for instruction in instructions {
            stacks = instruction.execute(stacks);
        }
        let result = summarise_stacks(&stacks);

        println!("Part 2: {}", result);
    }
}
