// https://adventofcode.com/2020/day/15

use std::collections::HashMap;

use crate::get_lines;

/// Parse the puzzle input
///
/// *Panics* if any of the input numbers are not valid 32-bit unsigned integers
pub fn parse_numbers() -> Vec<u32> {
    get_lines("/input/day-15-input.txt")
        .flat_map(|line| {
            line.split(',')
                .map(|slice| slice.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|string| string.parse::<u32>())
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
pub fn get_last_number_spoken(num_rounds: usize) -> u32 {
    let numbers = parse_numbers();
    let (oral_history, last_number_spoken) = numbers.iter().enumerate().fold(
        (HashMap::new(), 0u32),
        |state, (index, number)| -> (HashMap<u32, Vec<usize>>, u32) {
            let mut oral_history = state.0;
            let history = oral_history.entry(*number).or_insert_with(|| vec![index]);
            history.insert(0, index);
            history.truncate(2);

            (oral_history, *number)
        },
    );

    (numbers.len()..num_rounds)
        .fold(
            (oral_history, last_number_spoken),
            |state, i| -> (HashMap<u32, Vec<usize>>, u32) {
                let mut oral_history = state.0;
                let mut last_number_spoken = state.1;

                let history = oral_history
                    .get(&last_number_spoken)
                    .expect("Number should have been spoken before.");
                last_number_spoken = match history.len() {
                    0 => panic!("Missing history for {}", last_number_spoken),
                    1 => {
                        // spoken only once before
                        // "If that was the first time the number has been spoken, the current player says
                        // 0."
                        let number_to_speak = 0u32;
                        let history = oral_history
                            .get_mut(&number_to_speak)
                            .expect("Missing mapping");
                        history.insert(0, i);
                        history.truncate(2);

                        number_to_speak
                    }
                    _ => {
                        // spoken 2+ times before
                        // "Otherwise, the number had been spoken before; the current player announces how
                        // many turns apart the number is from when it was previously spoken."
                        let last_mention = history[0] as u32;
                        let penultimate_mention = history[1] as u32;
                        let number_to_speak = last_mention - penultimate_mention;
                        let history = oral_history
                            .entry(number_to_speak)
                            .or_insert_with(|| vec![i]);
                        history.insert(0, i);
                        history.truncate(2);

                        number_to_speak
                    }
                };
                (oral_history, last_number_spoken)
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
        // FIXME this takes an unusually long time
        // "Impressed, the Elves issue you a challenge: determine the 30,000,000th number spoken."
        let num_rounds = 30_000_000usize;
        let last_number_spoken = get_last_number_spoken(num_rounds);
        println!("Part 1: {}", last_number_spoken);
    }
}
