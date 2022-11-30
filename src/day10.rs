use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::get_lines;

// https://adventofcode.com/2020/day/10

/// Get a sorted, unique list of the joltages of all the available adapters.
///
/// *Panics* if any line in the input contains a malformed joltage
///
/// "Always prepared, you make a list of all of the joltage adapters in your bag."
pub fn get_joltage_adapters() -> Vec<u8> {
    let mut adapters = get_lines("day-10-input.txt")
        .map(|line| line.parse())
        .map(|result| result.unwrap()) // panic on invalid joltage
        .collect::<Vec<u8>>();
    adapters.sort_unstable();
    adapters
}

/// Count the number of ways _adapters_ can be arranged to connect a charging outlet to a device.
///
/// Parameters:
/// * `adapters` - a unique and sorted list of the joltages of the available adapters. This method
///                will not return a correct value if the vector is not unique and sorted.
/// * `from` - the joltage of the charging outlet
/// * `to` - the joltage of the device that needs to be charged
/// * `cache` - a history of precomputed arrangement counts. This will be updated prior to the
///             method returning.
pub fn count_adapter_arrangements(
    adapters: &[u8],
    from: u8,
    to: u8,
    cache: &mut HashMap<u64, u64>,
) -> u64 {
    let hash = || {
        let mut hasher = DefaultHasher::new();
        hasher.write_u8(from);
        hasher.write_u8(to);
        adapters.hash(&mut hasher);
        hasher.finish()
    };

    let hash = hash();
    if cache.contains_key(&hash) {
        return cache[&hash];
    }

    if adapters.is_empty() {
        return u64::from(from + 3 == to);
    } else if adapters.len() == 1 {
        let last = adapters[0];
        return u64::from(last == to - 3);
    }

    let mut retval = 0u64;
    for i in 0..3usize {
        if i >= adapters.len() {
            break;
        }
        let first = adapters[i];
        let difference = first - from;
        if (1..=3).contains(&difference) {
            let remaining = if i < adapters.len() {
                adapters.split_at(i + 1).1
            } else {
                &[0u8]
            };
            retval += count_adapter_arrangements(remaining, first, to, cache);
        }
    }
    cache.insert(hash, retval);
    retval
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day10::{count_adapter_arrangements, get_joltage_adapters};

    #[test]
    fn test() {
        let joltages = get_joltage_adapters();
        let target_joltage = joltages
            .last()
            .expect("At least one joltage adapter is required.")
            + 3u8;

        // "If you use every adapter in your bag at once, what is the distribution of joltage
        // differences between the charging outlet, the adapters, and your device?"
        // "In addition, your device has a built-in joltage adapter rated for 3 jolts higher than
        // the highest-rated adapter in your bag."
        let mut difference_distribution = [0u8, 0u8, 1u8];

        // "Treat the charging outlet near your seat as having an effective joltage rating of 0."
        let mut current_joltage = 0u8;

        while current_joltage < target_joltage - 3 {
            let next = joltages
                .iter()
                .find(|candidate| {
                    **candidate > current_joltage
                        && **candidate - current_joltage >= 1
                        && **candidate - current_joltage <= 3
                })
                .expect("Required joltage adapter not available.");
            let difference = *next as usize - current_joltage as usize;
            difference_distribution[difference - 1] += 1;
            current_joltage = *next;
        }
        println!(
            "Part 1: {}",
            difference_distribution[0] as u16 * difference_distribution[2] as u16
        );
        println!(
            "Part 2: {}",
            count_adapter_arrangements(&joltages, 0, target_joltage, &mut HashMap::new())
        );
    }
}
