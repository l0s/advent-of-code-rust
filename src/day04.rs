use std::collections::HashMap;

use advent_of_code::get_block_strings;

fn get_input() -> impl Iterator<Item=String> {
    get_block_strings("/input/day-4-input.txt")
}

fn get_blocks() -> impl Iterator<Item=HashMap<String, String>> {
    get_input()
        .map(|block| -> HashMap<String, String> {
            block.split_whitespace()
                .map(|string| string.trim())
                .filter(|block_entry| !block_entry.is_empty())
                .map(|block_entry| -> (String, String) {
                    let mut components = block_entry.splitn(2, ':');
                    let key = components.next().unwrap();
                    let value = components.next().unwrap();
                    (key.trim().to_owned(), value.trim().to_owned())
                }).collect()
        })
}

mod tests {
    use std::collections::HashSet;

    use regex::Regex;

    use crate::day04::get_blocks;

    #[test]
    fn part1() {
        let count_valid = get_blocks()
            .filter(|block| block.contains_key("byr"))
            .filter(|block| block.contains_key("iyr"))
            .filter(|block| block.contains_key("eyr"))
            .filter(|block| block.contains_key("hgt"))
            .filter(|block| block.contains_key("hcl"))
            .filter(|block| block.contains_key("ecl"))
            .filter(|block| block.contains_key("pid"))
            .count();
        println!("Part 1: {}", count_valid);
    }

    #[test]
    fn part2() {
        let valid_hair_colour = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let valid_eye_colours: HashSet<&'static str> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter()
                .cloned()
                .collect();
        let valid_passport_id = Regex::new(r"^[0-9]{9}$").unwrap();

        let count_valid = get_blocks()
            .filter(|block| -> bool {
                let birth_year: Option<u16> = block.get("byr")
                    .map(|string| string.parse::<u16>().unwrap())// FIXME how can I avoid unwrapping?
                    .filter(|year| *year >= 1920 && *year <= 2002);
                birth_year.is_some()
            }).filter(|block| -> bool {
                let issue_year: Option<u16> = block.get("iyr")
                    .map(|year| year.parse::<u16>().unwrap()) // FIXME how can I avoid unwrapping?
                    .filter(|year| *year >= 2010 && *year <= 2020);
                issue_year.is_some()
            }).filter(|block| -> bool {
                let expiration_year: Option<u16> = block.get("eyr")
                    .map(|year| year.parse::<u16>().unwrap())
                    .filter(|year| *year >= 2020 && *year <= 2030);
                expiration_year.is_some()
            }).filter(|block| -> bool {
                let height = block.get("hgt")
                    .filter(|string| -> bool {
                        if string.ends_with("cm") {
                            let cm = string.replace("cm", "").parse::<u8>();
                            if cm.is_err() {
                                return false;
                            }
                            let cm = cm.unwrap();
                            cm >= 150 && cm <= 193
                        } else if string.ends_with("in") {
                            let inches = string.replace("in", "").parse::<u8>();
                            if inches.is_err() {
                                return false;
                            }
                            let inches = inches.unwrap();
                            inches >= 59 && inches <= 76
                        } else {
                            false
                        }
                    });
                height.is_some()
            }).filter(|block| -> bool {
                let hair_colour = block.get("hcl")
                    .filter(|colour| valid_hair_colour.is_match(colour));
                hair_colour.is_some()
            }).filter(|block| -> bool {
                let eye_colour = block.get("ecl")
                    .filter(|colour| valid_eye_colours.contains(colour as &str));
                eye_colour.is_some()
            }).filter(|block| -> bool {
                let passport_id = block.get("pid")
                    .filter(|id| valid_passport_id.is_match(id));
                passport_id.is_some()
            }).count();
        println!("Part 2: {}", count_valid);
    }
}