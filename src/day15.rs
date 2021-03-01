// https://adventofcode.com/2020/day/15

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use hashers::fx_hash::FxHasher;

use crate::get_lines;

/// Parse the puzzle input
///
/// *Panics* if any of the input numbers are not valid 32-bit unsigned integers
pub fn parse_numbers() -> Vec<usize> {
    get_lines("/input/day-15-input.txt")
        .flat_map(|line| {
            line.split(',')
                .map(|slice| slice.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|string| string.parse::<usize>())
        .map(|result| result.expect("Invalid number"))
        .collect()
}

/// Get the last number spoken after playing the Elves' memory game for _num_rounds_ rounds.
///
/// Parameters:
/// - `num_rounds` - the number of turns in the game. Each turn involves a player speaking one
///                  number.
///
/// Returns: The last number spoken after the specified number of turns/rounds.
pub fn get_last_number_spoken(num_rounds: usize) -> usize {
    let numbers = parse_numbers();
    let (oral_history, last_number_spoken) = numbers.iter().enumerate().fold(
        (
            HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            0usize,
        ),
        |state, (index, number)| -> (HashMap<usize, usize, _>, usize) {
            let mut oral_history = state.0;
            oral_history.insert(*number, index + 1);

            (oral_history, *number)
        },
    );

    (numbers.len()..num_rounds)
        .fold(
            (oral_history, last_number_spoken),
            |state, i| -> (HashMap<usize, usize, _>, usize) {
                let mut oral_history = state.0;
                let last_number_spoken = state.1;

                let next_number_to_speak = match oral_history.entry(last_number_spoken) {
                    Entry::Occupied(mut entry) => {
                        let last_mention = entry.insert(i);
                        i - last_mention
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(i);
                        0usize
                    }
                };

                (oral_history, next_number_to_speak)
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use crate::day15::get_last_number_spoken;

    #[test]
    fn part1() {
        // "Their question for you is: what will be the 2020th number spoken?"
        let num_rounds = 2020usize;
        let last_number_spoken = get_last_number_spoken(num_rounds);
        println!("Part 1: {}", last_number_spoken);
    }

    #[test]
    fn part2() {
        // "Impressed, the Elves issue you a challenge: determine the 30,000,000th number spoken."
        let num_rounds = 30_000_000usize;
        let last_number_spoken = get_last_number_spoken(num_rounds);
        println!("Part 1: {}", last_number_spoken);
    }
}
