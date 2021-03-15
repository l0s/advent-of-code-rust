// https://adventofcode.com/2020/day/15

use std::collections::hash_map::{Entry, RandomState};
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};

use hashers::fx_hash::FxHasher;

use crate::get_lines;

/// Parse the puzzle input
///
/// *Panics* if any of the input numbers are not valid array indices
pub fn parse_numbers() -> Vec<usize> {
    get_lines("day-15-input.txt")
        .flat_map(|line| {
            line.split(',')
                .map(|slice| slice.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|string| string.parse::<usize>())
        .map(|result| result.expect("Invalid number"))
        .collect()
}

/// A variant of [Van Eck's sequence](http://oeis.org/A181391) that starts with a specific seed of
/// numbers.
///
/// This Iterator has as many elements as the maximum value of a `usize`.
///
/// Parameters:
/// - `S` - the hash function to use for keeping track of the last time a number was spoken
pub struct VanEckSequence<S: BuildHasher = RandomState> {
    /// the first items in the sequence
    seed: Vec<usize>,
    /// A mapping of sequence value to the last index into the sequence at which it appeared
    oral_history: HashMap<usize, usize, S>,
    /// The index of the _next_ number to speak
    index: usize,
    last_number_spoken: usize,
}

impl<S: BuildHasher> VanEckSequence<S> {
    /// Create a new sequence
    ///
    /// Parameters:
    /// - `seed` - the first items in the sequence.
    /// - `oral_history` - a cache to store the last time each number was spoken.
    fn with_cache(seed: Vec<usize>, oral_history: HashMap<usize, usize, S>) -> VanEckSequence<S> {
        VanEckSequence {
            seed,
            oral_history,
            index: 0usize,
            last_number_spoken: 0usize,
        }
    }
}

impl VanEckSequence<BuildHasherDefault<FxHasher>> {
    /// Create a new sequence
    ///
    /// Parameters:
    /// - `seed` - the first items in the sequence.
    pub fn new(seed: Vec<usize>) -> VanEckSequence<BuildHasherDefault<FxHasher>> {
        VanEckSequence::with_cache(
            seed,
            HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
        )
    }
}

impl<S: BuildHasher> Iterator for VanEckSequence<S> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_number_to_speak = if self.index < self.seed.len() {
            let next_number_to_speak = self.seed[self.index];
            self.oral_history
                .insert(next_number_to_speak, self.index + 1);
            next_number_to_speak
        } else {
            match self.oral_history.entry(self.last_number_spoken) {
                Entry::Occupied(mut entry) => {
                    let last_mention = entry.insert(self.index);
                    self.index - last_mention
                }
                Entry::Vacant(entry) => {
                    entry.insert(self.index);
                    0usize
                }
            }
        };

        self.index += 1;
        self.last_number_spoken = next_number_to_speak;
        Some(next_number_to_speak)
    }
}

/// Get the last number spoken after playing the Elves' memory game for _num_rounds_ turns.
///
/// Parameters:
/// - `num_rounds` - the number of turns in the game. Each turn involves a player speaking one
///                  number.
///
/// Returns: The last number spoken after the specified number of turns/rounds.
pub fn get_last_number_spoken(num_rounds: usize) -> usize {
    let numbers = parse_numbers();
    VanEckSequence::new(numbers)
        .nth(num_rounds - 1)
        .expect("Sequence should be unbounded")
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
        println!("Part 2: {}", last_number_spoken);
    }
}
