use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use crate::day04::BufReadResult::{EndOfInput, EndOfBlock, PartialBlock, BufferingError};

/// A wrapper for a BufRead that splits around empty lines.
///
/// This allows one to iterate through blocks of text without needing to read the whole input into
/// memory at once. The specific delimiter it looks for is "\n\n". No other delimiters are supported.
struct Blocks<R: BufRead> {
    reader: R
}

enum BufReadResult<'a, E> {
    /// There are no more bytes to be read.
    EndOfInput,
    /// Part of a block is available, it may be the beginning of a block or a middle portion.
    PartialBlock(&'a [u8]),
    /// The provided array includes the end of the block. It may also be an entire block.
    EndOfBlock(&'a [u8]),
    /// An error occurred while reading from the underlying buffer.
    BufferingError(E),
}

impl<R: BufRead> Blocks<R> {

    /// Read a portion of the buffer.
    ///
    /// Read some number of bytes from the underlying buffer. This may need to be called multiple
    /// times in order to read a full block of text; blocks are delimited by empty lines.
    ///
    /// **Important:** This method does not consume bytes read from the underlying reader. Callers
    /// **must** consume the appropriate number of bytes.
    ///
    /// * `previous_byte` - the last byte read from a previous `try_read` invocation. This is needed
    ///                     because the delimiter is two bytes ("\n\n") and therefore may span
    ///                     two calls to `try_read`. If a previous byte is not available, provide
    ///                     any value other than '\n'.
    fn try_read(&mut self, previous_byte: u8) -> BufReadResult<std::io::Error> {
        return match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.is_empty() {
                    return EndOfInput;
                }
                let mut previous = previous_byte;
                for i in 0..buffer.len() {
                    let current = buffer[i];
                    if previous == b'\n' && current == b'\n' {
                        return EndOfBlock(&buffer[0..i + 1]);
                    }
                    previous = current;
                }
                PartialBlock(&buffer)
            }
            Err(error) => {
                BufferingError(error)
            }
        };
    }
}

impl<R: BufRead> Iterator for Blocks<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = vec!();

        loop {
            let previous_byte = if bytes.len() > 0 { bytes[bytes.len() - 1] } else { b'_' };
            let mut bytes_read = 0_usize;
            let mut complete = false;
            let mut result = None;

            match &self.try_read(previous_byte) {
                EndOfInput => {
                    if bytes.len() > 0 {
                        result = Some(String::from_utf8_lossy(&bytes).trim().to_string());
                    }
                    complete = true;
                }
                PartialBlock(partial) => {
                    bytes = [&bytes, *partial].concat();
                    bytes_read = partial.len();
                }
                EndOfBlock(partial) => {
                    bytes = [&bytes, *partial].concat();
                    result = Some(String::from_utf8_lossy(&bytes).trim().to_string());
                    bytes_read = partial.len();
                    complete = true;
                }
                BufferingError(error) => {
                    eprintln!("Error buffering blocks: {}", error);
                    complete = true;
                }
            }
            &self.reader.consume(bytes_read);
            if complete {
                return result;
            }
        }
    }
}

pub fn get_block_strings(file: &str) -> impl Iterator<Item=String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = String::from(manifest_dir) + file;
    let path = Path::new(&file_name);
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    Blocks { reader }
}

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