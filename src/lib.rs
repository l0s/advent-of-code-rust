#[macro_use]
extern crate lazy_static;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

use crate::BufReadResult::{BufferingError, EndOfBlock, EndOfInput, PartialBlock};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines(file: &str) -> impl Iterator<Item = String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = String::from(manifest_dir) + file;
    let path = Path::new(&file_name);
    let file = File::open(path).expect("file not found"); // FIXME should let the client decide whether or not to panic
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap)
}

/// A wrapper for a BufRead that splits around empty lines.
///
/// This allows one to iterate through blocks of text without needing to read the whole input into
/// memory at once. The specific delimiter it looks for is "\n\n". No other delimiters are supported.
struct Blocks<R: BufRead> {
    reader: R,
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
            Err(error) => BufferingError(error),
        };
    }
}

impl<R: BufRead> Iterator for Blocks<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = vec![];

        loop {
            let previous_byte = if !bytes.is_empty() {
                bytes[bytes.len() - 1]
            } else {
                b'_'
            };
            let mut bytes_read = 0_usize;
            let mut complete = false;
            let mut result = None;

            match &self.try_read(previous_byte) {
                EndOfInput => {
                    if !bytes.is_empty() {
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
            self.reader.consume(bytes_read);
            if complete {
                return result;
            }
        }
    }
}

pub fn get_block_strings(file: &str) -> impl Iterator<Item = String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = String::from(manifest_dir) + file;
    let path = Path::new(&file_name);
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    Blocks { reader }
}
