/// --- Day 6: Tuning Trouble ---
/// https://adventofcode.com/2022/day/6
use crate::get_lines;
use std::collections::{BTreeSet, VecDeque};

/// Characters received by the Elves' handheld communication device
pub fn get_signal() -> String {
    get_lines("day-06.txt").next().expect("No signal detected")
}

/// Determine which character of the buffer contains the start of a packet
/// Returns:
/// - `Some(usize)` - the index of the first packet if one exists
/// - `None` - if no packet is found
pub fn get_start_of_packet(data_stream: String) -> Result<usize, &'static str> {
    let distinct_characters = 4;
    let error = "No start of packet found";
    get_marker_position(data_stream, distinct_characters, error)
}

/// Determine which character of the buffer contains the start of the message
/// Returns:
/// - `Some(usize)` - the index of the start of the message if one exists
/// - `None` if there is no message
pub fn get_start_of_message(data_stream: String) -> Result<usize, &'static str> {
    get_marker_position(data_stream, 14, "No message found")
}

fn get_marker_position(
    data_stream: String,
    distinct_characters: usize,
    error: &str,
) -> Result<usize, &str> {
    let mut buffer = VecDeque::new();
    for (index, c) in data_stream.chars().enumerate() {
        if buffer.len() < distinct_characters {
            buffer.push_back(c);
            continue;
        }
        let set = buffer.iter().copied().collect::<BTreeSet<char>>();
        if set.len() >= buffer.len() {
            return Ok(index);
        }
        buffer.pop_front();
        buffer.push_back(c);
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use crate::day06::{get_signal, get_start_of_message, get_start_of_packet};

    #[test]
    fn part1() {
        let result = get_start_of_packet(get_signal()).unwrap();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let result = get_start_of_message(get_signal()).unwrap();

        println!("Part 2: {}", result);
    }
}
