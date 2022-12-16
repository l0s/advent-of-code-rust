use crate::get_block_strings;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::str::FromStr;
use PacketItem::{List, Literal};

/// --- Day 13: Distress Signal ---
/// https://adventofcode.com/2022/day/13

#[derive(Clone)]
pub enum PacketItem {
    List(Box<Vec<PacketItem>>),
    Literal(u16),
}

impl Eq for PacketItem {}

impl PartialEq<Self> for PacketItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            List(items) => match other {
                List(other_items) => {
                    let common_length = items.len().min(other_items.len());
                    let mut x_iter = items.iter().take(common_length);
                    let mut y_iter = other_items.iter().take(common_length);
                    while let (Some(x), Some(y)) = (x_iter.next(), y_iter.next()) {
                        let result = x.cmp(y);
                        if result != Equal {
                            return result;
                        }
                    }
                    items.len().cmp(&other_items.len())
                }
                Literal(other_value) => self.cmp(&List(Box::new(vec![Literal(*other_value)]))),
            },
            Literal(value) => match other {
                List(other_items) => {
                    List(Box::new(vec![Literal(*value)])).cmp(&List(Box::new(other_items.to_vec())))
                }
                Literal(other_value) => value.cmp(other_value),
            },
        }
    }
}

impl FromStr for PacketItem {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<PacketItem>> = vec![];

        let mut number_buffer = String::new();
        for c in line.chars() {
            if c == '[' {
                stack.push(vec![]);
            } else if c == ']' {
                if !number_buffer.is_empty() {
                    let number = number_buffer.parse::<u16>().unwrap();
                    number_buffer.clear();
                    let mut last = stack.pop().unwrap();
                    last.push(Literal(number));
                    stack.push(last);
                }
                if stack.len() > 1 {
                    let completed = stack.pop().unwrap();
                    let mut last = stack.pop().unwrap();
                    last.push(List(Box::new(completed)));
                    stack.push(last);
                }
            } else if c == ',' {
                if !number_buffer.is_empty() {
                    let number = number_buffer.parse::<u16>().unwrap();
                    number_buffer.clear();
                    let mut last = stack.pop().unwrap();
                    last.push(Literal(number));
                    stack.push(last);
                }
            } else {
                number_buffer.push(c);
            }
        }

        // currently cannot parse a number literal on its own
        Ok(List(Box::new(stack.pop().unwrap())))
    }
}

pub fn get_input() -> impl Iterator<Item = (PacketItem, PacketItem)> {
    get_block_strings("day-13.txt").map(|block| -> (PacketItem, PacketItem) {
        let mut lines = block.split('\n');
        let left = lines.next().unwrap().parse::<PacketItem>().unwrap();
        let right = lines.next().unwrap().parse::<PacketItem>().unwrap();
        (left, right)
    })
}

#[cfg(test)]
pub mod tests {

    use crate::day13::PacketItem::{List, Literal};
    use crate::day13::{get_input, PacketItem};

    #[test]
    pub fn part1() {
        let mut result = 0;
        for (i, pair) in get_input().enumerate() {
            if pair.0 < pair.1 {
                eprintln!("{}", i + 1);
                result += i + 1;
            }
        }
        println!("Part 1: {}", result);
    }

    #[test]
    pub fn part2() {
        let mut packets = get_input()
            .flat_map(|(x, y)| vec![x, y])
            .collect::<Vec<PacketItem>>();
        packets.sort();

        let divider_x = List(Box::new(vec![List(Box::new(vec![Literal(2)]))]));
        let divider_y = List(Box::new(vec![List(Box::new(vec![Literal(6)]))]));

        let x_index = packets.binary_search(&divider_x).unwrap_err() + 1;
        let y_index = packets.binary_search(&divider_y).unwrap_err() + 2;
        let result = x_index * y_index;

        println!("Part 2: {}", result);
    }
}
