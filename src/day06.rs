use crate::get_block_strings;

pub fn get_input() -> impl Iterator<Item = Vec<String>> {
    get_block_strings("/input/day-6-input.txt").map(|string| {
        string
            .split_whitespace()
            .map(|str| str.trim().to_owned())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day06::get_input;

    #[test]
    fn part1() {
        let sum: u16 = get_input()
            .map(|group| -> u16 {
                // number of questions to which group answered yes
                group
                    .iter()
                    .flat_map(|string| string.chars())
                    .collect::<HashSet<char>>()
                    .len() as u16
            })
            .sum();
        println!("Part 1: {}", sum);
    }

    #[test]
    fn part2() {
        let sum: u16 = get_input()
            .map(|group| -> u16 {
                // TODO support all UTF-8 code points
                let unique_characters = group
                    .iter()
                    .flat_map(|string| string.chars())
                    .collect::<HashSet<char>>();
                unique_characters
                    .iter()
                    .filter(|c| -> bool {
                        // whether or not c was answered by all in group
                        group
                            .iter()
                            .map(|string| string.chars().collect::<Vec<char>>())
                            .filter(|chars| chars.contains(c))
                            .count()
                            == group.len()
                    })
                    .count() as u16
            })
            .sum();
        println!("Part 2: {}", sum);
    }
}
