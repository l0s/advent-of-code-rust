// https://adventofcode.com/2020/day/14

use std::num::ParseIntError;
use std::ops::BitXor;
use std::str::FromStr;

use regex::Regex;

use crate::day14::Command::{SetMask, SetMemory};
use crate::day14::ParseError::{
    InvalidAddress, InvalidInstruction, InvalidMask, InvalidValue, MissingLeftValue,
    MissingRightValue,
};
use crate::get_lines;

/// An instruction in the sea port's computer system initialisation programme
///
/// A ferry will use a series of commands to dock at the sea port.
pub enum Command {
    /// Update the current bit mask
    /// Parameters:
    /// - bitmask - A 36-character String containing only the characters '0', '1', or 'X'.
    ///             The most significant bit is to the left and the least significant bit is to the
    ///             right.
    SetMask(String),

    /// Update the program's memory
    /// Parameters
    /// - `address`
    /// - `value`
    SetMemory(u64, u64),
}

/// An error that may be raised when parsing the initialisation programme.
#[derive(Debug)]
pub enum ParseError {
    MissingLeftValue(String),
    MissingRightValue(String),
    InvalidInstruction(String),
    InvalidAddress(String),
    InvalidValue(ParseIntError),
    InvalidMask(String),
}

/// Apply a mask to an integer value prior to storing it in memory. This is used by version 1 of the
/// ferry's docking programme decoder chip.
///
/// Parameters
/// - `mask` - a 36-bit, big-endian mask consisting of the characters '0', '1', and 'X'
/// - `value` - a 36-bit, unsigned integer (the most-significant 28 bits are unmasked)
///
/// Returns: The result of masking the least-significant 36 bits
pub fn mask_value(mask: &str, value: &u64) -> u64 {
    let mask_chars = mask.chars().collect::<Vec<char>>();
    (0..36usize).fold(*value, |result, i| -> u64 {
        let flag = 1u64 << i;
        let mask_value = mask_chars.get(mask.len() - i - 1).expect("Invalid mask");
        match mask_value {
            'X' => result, // "an X leaves the bit in the value unchanged"
            '0' => result & std::u64::MAX.bitxor(flag),
            '1' => result | flag,
            _ => panic!("Invalid mask value: {}", mask_value),
        }
    })
}

/// Apply a mask to a memory address. This is used by version 2 of the ferry's docking programme
/// decoder chip.
///
/// Parameters:
/// - `mask` - a 36-bit, big-endian mask consisting of the characters '0', '1', and 'X'
/// - `address` - a 36-bit, unsigned integer representing a memory address (the most-significant 28
///               bits are unmasked)
///
/// Returns: All the memory addresses that should be updated
pub fn mask_address(mask: &str, address: &u64) -> Vec<u64> {
    let mask_chars = mask.chars().collect::<Vec<char>>();
    let spec = (0..36usize)
        .map(|i| -> char {
            let mask_value = mask_chars.get(mask.len() - i - 1).expect("Invalid mask");
            let address_value = (address & (1 << i)) >> i;
            match mask_value {
                // "If the bitmask bit is X, the corresponding memory address bit is floating."
                // "If the bitmask bit is 1, the corresponding memory address bit is overwritten with
                // 1."
                'X' | '1' => mask_value.to_owned(),
                // "If the bitmask bit is 0, the corresponding memory address bit is unchanged."
                '0' => address_value
                    .to_string()
                    .chars()
                    .next()
                    .expect("Invalid address bit"),
                _ => panic!("Invalid mask value: {}", mask_value),
            }
        })
        .collect::<Vec<char>>();
    explode(spec)
}

/// Expand an address specification into all the matching addresses.
///
/// Parameters:
/// - `spec` - a 36-character address specification consisting of the characters '0', '1', and 'X'
///            For every 'X', two variants will be generated, one in which it is replaced by '0',
///            and one in which it is replaced by '1'.
///
/// Returns: All the possible memory locations. The length is 2^(number of Xs in `spec`).
fn explode(spec: Vec<char>) -> Vec<u64> {
    let floating_indices = spec
        .iter()
        .enumerate()
        .filter_map(|(index, bit)| if *bit == 'X' { Some(index) } else { None })
        .collect::<Vec<usize>>();
    explode_indices(spec, floating_indices.as_slice())
}

/// Expand an address specification into all the matching addresses.
///
/// Parameters:
/// - `spec` - a 36-character address specification consisting of the characters '0', '1', and 'X'
///            For every 'X', two variants will be generated, one in which it is replaced by '0',
///            and one in which it is replaced by '1'.
/// - `floating_indices` - the indices of every 'X' in `spec`.
///
/// Returns: All the possible memory locations. The length is 2^floating_indices.len().
fn explode_indices(spec: Vec<char>, floating_indices: &[usize]) -> Vec<u64> {
    if floating_indices.is_empty() {
        return vec![to_int(spec)];
    }
    let floating_index = floating_indices[0];
    let mut result: Vec<u64> = Vec::new();
    let mut copy = spec;
    let sub = floating_indices.split_at(1).1;
    copy[floating_index] = '0';
    explode_indices(copy.clone(), sub)
        .iter()
        .for_each(|address| result.push(*address));
    copy[floating_index] = '1';
    explode_indices(copy, sub)
        .iter()
        .for_each(|address| result.push(*address));
    result
}

/// Convert a vector of 0s and 1s to an integer.
///
/// Parameters:
/// - `chars` - a vector of length 36 for which each character is either '0' or '1'.
///
/// Returns: the integer representation of `chars`
fn to_int(chars: Vec<char>) -> u64 {
    (0..36usize).fold(0u64, |result, i| -> u64 {
        let bit = chars[i];
        match bit {
            '0' => result,
            '1' => result | (1u64 << i),
            _ => panic!(format!("Invalid bit: {}", bit)),
        }
    })
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.splitn(2, " = ");
        let left_value = match components.next() {
            None => return Err(MissingLeftValue(line.to_owned())),
            Some(string) => string.trim(),
        };
        let right_value = match components.next() {
            None => return Err(MissingRightValue(line.to_owned())),
            Some(string) => string.trim(),
        };
        match left_value {
            "mask" => {
                let mask = right_value.to_owned();
                if right_value.len() != 36 {
                    // "The bitmask is always given as a string of 36 bits"
                    return Err(InvalidMask(mask));
                }
                if right_value
                    .chars()
                    .any(|c| c != '0' && c != '1' && c != 'X')
                {
                    return Err(InvalidMask(mask));
                }
                Ok(SetMask(mask))
            }
            _ => {
                lazy_static! {
                    /// A regular expression that matches a memory-assignment lvalue.
                    /// It includes a single capture group that contains the memory address.
                    static ref MEM: Regex = Regex::new("mem\\[([0-9]+)\\]").unwrap();
                }
                match MEM.captures(left_value) {
                    None => Err(InvalidInstruction(left_value.to_owned())),
                    Some(captures) => {
                        if let Some(matcher) = captures.get(1) {
                            match matcher.as_str().parse::<u64>() {
                                Err(_) => return Err(InvalidAddress(matcher.as_str().to_owned())),
                                Ok(address) => match right_value.parse::<u64>() {
                                    Err(e) => Err(InvalidValue(e)),
                                    Ok(value) => Ok(SetMemory(address, value)),
                                },
                            }
                        } else {
                            Err(InvalidAddress(left_value.to_owned()))
                        }
                    }
                }
            }
        }
    }
}

/// Parse the puzzle input
pub fn parse_initialisation_programme() -> impl Iterator<Item = Command> {
    get_lines("/input/day-14-input.txt")
        .map(|line| line.parse::<Command>())
        .map(|result| result.expect("Unparseable line"))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day14::{mask_address, mask_value, parse_initialisation_programme, Command};

    #[test]
    fn part1() {
        let (memory, _) = parse_initialisation_programme().fold(
            (HashMap::new(), "X".repeat(36)),
            move |mut state, command| -> (HashMap<u64, u64>, String) {
                match command {
                    Command::SetMask(mask) => (state.0, mask),
                    Command::SetMemory(address, value) => {
                        let value = mask_value(&state.1, &value);
                        state.0.insert(address, value);
                        state
                    }
                }
            },
        );
        // "To initialize your ferry's docking program, you need the sum of all values left in
        // memory after the initialization program completes."
        let result = memory.values().sum::<u64>();
        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let (memory, _) = parse_initialisation_programme().fold(
            (HashMap::new(), "0".repeat(36)),
            move |mut state, command| -> (HashMap<u64, u64>, String) {
                match command {
                    Command::SetMask(mask) => (state.0, mask),
                    Command::SetMemory(address, value) => {
                        mask_address(&state.1, &address).iter().for_each(|address| {
                            state.0.insert(address.to_owned(), value);
                        });
                        state
                    }
                }
            },
        );
        // "To initialize your ferry's docking program, you need the sum of all values left in
        // memory after the initialization program completes."
        let result = memory.values().sum::<u64>();
        println!("Part 2: {}", result);
    }
}
