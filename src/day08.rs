use std::str::FromStr;

use crate::get_lines;

use crate::day08::Operation::{Accumulate, Jump, NoOp};
use crate::day08::ParseError::{
    InvalidArgument, InvalidOperation, MissingArgument, MissingOperation,
};
use std::num::ParseIntError;

#[derive(PartialEq)]
pub enum Operation {
    /// Increase or decrease the accumulator by the value in the argument.
    /// Move to the instruction directly below.
    Accumulate,
    /// Do nothing then move to the instruction directly below.
    NoOp,
    /// Jump to a new instruction relative to the current one offset by the argument.
    Jump,
}

impl Operation {
    pub fn update_total(&self, previous_total: i32, argument: i32) -> i32 {
        match &self {
            Accumulate => previous_total + argument,
            NoOp => previous_total,
            Jump => previous_total,
        }
    }

    pub fn update_index(&self, previous_index: usize, argument: i32) -> usize {
        match &self {
            Accumulate => previous_index + 1,
            NoOp => previous_index + 1,
            Jump => (previous_index as i32 + argument) as usize,
        }
    }
}

pub enum ParseError {
    InvalidOperation(String),
    MissingOperation(String),
    MissingArgument(String),
    InvalidArgument(ParseIntError),
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "acc" => Ok(Accumulate),
            "nop" => Ok(NoOp),
            "jmp" => Ok(Jump),
            _ => Err(InvalidOperation(String::from(s))),
        }
    }
}

/// A boot code instruction
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    /// Compute the new value of the accumulator
    ///
    /// Parameters:
    /// - `previous_total` - the current accumulator value
    ///
    /// Returns: the new accumulator value
    pub fn update_total(&self, previous_total: i32) -> i32 {
        self.operation.update_total(previous_total, self.argument)
    }

    /// Identify the index of the next instruction to execute
    ///
    /// Parameters:
    /// - `previous_index` - the current instruction index
    ///
    /// Returns: the index of the next instruction to execute
    pub fn update_index(&self, previous_index: usize) -> usize {
        self.operation.update_index(previous_index, self.argument)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.splitn(2, ' ');
        let operation = match components.next() {
            None => return Err(MissingOperation(String::from(s))),
            Some(operation) => match operation.parse::<Operation>() {
                Ok(operation) => operation,
                Err(e) => return Err(e),
            },
        };

        let argument = match components.next() {
            None => return Err(MissingArgument(String::from(s))),
            Some(argument) => match argument.parse::<i32>() {
                Ok(argument) => argument,
                Err(parse_int_error) => return Err(InvalidArgument(parse_int_error)),
            },
        };

        Ok(Instruction {
            operation,
            argument,
        })
    }
}

pub fn get_instructions() -> Vec<Instruction> {
    get_lines("/input/day-8-input.txt")
        .flat_map(|line| line.parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day08::Operation::{Accumulate, Jump, NoOp};
    use crate::day08::{get_instructions, Instruction};

    #[test]
    fn part1() {
        let instructions = get_instructions();
        let mut visited = vec![false; instructions.len()];
        let mut index = 0_usize;
        let mut accumulator = 0_i32;

        while !visited[index] {
            visited[index] |= true;
            let instruction = &instructions[index];
            accumulator = instruction.update_total(accumulator);
            index = instruction.update_index(index);
        }
        println!("Part 1: {}", accumulator);
    }

    #[test]
    fn part2() {
        let instructions = get_instructions();
        // "After some careful analysis, you believe that exactly one instruction is corrupted."
        'instructions: for i in 0..instructions.len() {
            let to_replace = &instructions[i];
            if to_replace.operation == Accumulate {
                // "No acc instructions were harmed in the corruption of this boot code."
                continue;
            }
            let operation = if to_replace.operation == NoOp {
                Jump
            } else {
                NoOp
            };
            let replacement = Instruction {
                operation,
                argument: to_replace.argument,
            };

            let mut index = 0_usize;
            let mut accumulator = 0_i32;
            let mut visited = vec![false; instructions.len()];

            // "The program is supposed to terminate by attempting to execute an instruction
            // immediately after the last instruction in the file."
            while index < instructions.len() {
                let instruction = if index == i {
                    &replacement
                } else {
                    &instructions[index]
                };
                if visited[index] {
                    // replacing the instruction causes an infinite loop
                    // try replacing a different one

                    // NB: Simply re-visiting this instruction is an infinite loop because the
                    // argument to each instruction is constant and never dependent on the value of
                    // "accumulator"
                    continue 'instructions;
                }
                visited[index] |= true;
                accumulator = instruction.update_total(accumulator);
                index = instruction.update_index(index);
            }
            println!("Part 2: {}", accumulator);
            break;
        }
    }
}
