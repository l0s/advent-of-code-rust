// --- Day 16: Ticket Translation ---
// https://adventofcode.com/2020/day/16

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Range;

use crate::get_lines;

pub struct Ticket {
    numbers: Vec<u16>,
}

impl Ticket {
    pub fn is_valid(&self, fields: &HashSet<Field>) -> bool {
        self.get_invalid_numbers(fields).is_empty()
    }

    fn get_invalid_numbers(&self, fields: &HashSet<Field>) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::new();
        'outer: for number in self.numbers.iter() {
            for field in fields.iter() {
                if field.contains(*number) {
                    continue 'outer;
                }
            }
            result.push(*number);
        }
        result
    }
}

#[derive(Eq)]
pub struct Field {
    label: String,
    ranges: Vec<Range<u16>>,
}

impl Field {
    pub fn contains(&self, number: u16) -> bool {
        self.ranges.iter()
            .any(|range| range.contains(&number))
    }
}

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
        state.finish();
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

pub fn get_input() -> (Ticket, HashSet<Field>, Vec<Ticket>) {
    let mut fields: HashSet<Field> = HashSet::new();
    let mut nearby_tickets: Vec<Ticket> = Vec::new();

    let mut section = 0u8;
    let mut my_ticket: Option<Ticket> = None;
    for line in get_lines("day-16-input.txt") {
        if line.is_empty() {
            section += 1;
            continue;
        } else if line.trim().eq_ignore_ascii_case("your ticket:")
            || line.trim().eq_ignore_ascii_case("nearby tickets:") {
            continue;
        }
        match section {
            0 => {
                let mut sections = line.splitn(2, ": ");
                let label = sections.next()
                    .expect("Expected field label and ranges delimited by \": \"")
                    .trim()
                    .to_owned();
                let ranges = sections.next()
                    .expect("No ranges specified")
                    .trim()
                    .split(" or ")
                    .map(|string| -> Range<u16> {
                        let mut bounds = string.splitn(2, '-');
                        let start = bounds.next()
                            .expect("Missing lower bound of range")
                            .trim()
                            .parse::<u16>().expect("Cannot parse range start.");
                        let end = bounds.next()
                            .expect("Missing upper bound of range")
                            .trim()
                            .parse::<u16>()
                            .expect("Cannot parse range end.") + 1;
                        if let Some(component) = bounds.next() {
                            panic!("Unexpected range component encountered: {}", component);
                        }
                        Range { start, end }
                    }).collect::<Vec<Range<u16>>>();
                if let Some(section) = sections.next() {
                    panic!("Unexpected section: {}", section);
                }
                fields.insert(Field { label, ranges });
            }
            1 => {
                my_ticket =
                    Some(Ticket {
                        numbers: line.split(',')
                            .map(|section| section.trim()
                                .parse::<u16>()
                                .expect("Cannot parse ticket number."))
                            .collect::<Vec<u16>>()
                    });
            }
            2 => {
                nearby_tickets.push(Ticket {
                    numbers: line.split(',')
                        .map(|section| section.trim()
                            .parse::<u16>()
                            .expect("Cannot parse nearby ticket number."))
                        .collect::<Vec<u16>>()
                });
            }
            _ => panic!("Unexpected section starting with: {}", line)
        }
    }
    (my_ticket.expect("Ticket not issued."), fields, nearby_tickets)
}

#[cfg(test)]
mod tests {
    use crate::day16::get_input;

    #[test]
    fn part1() {
        let (_, fields, nearby_tickets) = get_input();
        let sum: u16 = nearby_tickets.iter()
            .flat_map(|ticket| ticket.get_invalid_numbers(&fields))
            .sum();
        println!("Part 1: {}", sum);
    }
}