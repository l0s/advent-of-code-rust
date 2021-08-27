// --- Day 16: Ticket Translation ---
// https://adventofcode.com/2020/day/16

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Range;

use crate::get_lines;

/// A high-speed train ticket
pub struct Ticket {
    /// The numbers that appear on the ticket in the order that they appear
    numbers: Vec<usize>,
}

impl Ticket {
    /// Determine if the ordered list of numbers represents a valid ticket given the field
    /// definitions.
    ///
    /// Parameters
    /// - `fields` - the known rules that ticket fields must follow
    ///
    /// Returns: true if and only if this ticket satisfies the specified rules
    pub fn is_valid(&self, fields: &HashSet<Field>) -> bool {
        self.get_invalid_numbers(fields).is_empty()
    }

    /// Find all of the numbers on the ticket that cannot be valid
    ///
    /// Parameters
    /// - `fields` the known rules that ticket fields must follow
    ///
    /// Returns: all of the numbers on the ticket that cannot correspond to any of the fields
    fn get_invalid_numbers(&self, fields: &HashSet<Field>) -> Vec<usize> {
        /*
        Using a generator here would be ideal once that feature is stable:
        https://doc.rust-lang.org/std/ops/trait.Generator.html .
        This will allow us to support short-circuiting
        */
        let mut result: Vec<usize> = Vec::new();
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

/// The rules for a ticket field
#[derive(Eq, Clone)]
pub struct Field {
    /// The name of the field
    label: String,

    /// The valid ranges for this field on a ticket
    ranges: Vec<Range<usize>>,
}

impl Field {
    /// Determine if this field can contain a certain number
    ///
    /// Parameters
    /// - `number` - a number on a ticket
    ///
    /// Returns: true if and only if the number _can_ correspond to this field
    pub fn contains(&self, number: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&number))
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

/// Parse the problem input
///
/// Returns a tuple with the following values:
/// - The high-speed train ticket assigned to you
/// - the valid ranges for the ticket fields
/// - the numbers on all the nearby tickets, sourced via the airport security cameras
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
            || line.trim().eq_ignore_ascii_case("nearby tickets:")
        {
            continue;
        }
        match section {
            0 => {
                let mut sections = line.splitn(2, ": ");
                let label = sections
                    .next()
                    .expect("Expected field label and ranges delimited by \": \"")
                    .trim()
                    .to_owned();
                let ranges = sections
                    .next()
                    .expect("No ranges specified")
                    .trim()
                    .split(" or ")
                    .map(|string| -> Range<usize> {
                        let mut bounds = string.splitn(2, '-');
                        let start = bounds
                            .next()
                            .expect("Missing lower bound of range")
                            .trim()
                            .parse::<usize>()
                            .expect("Cannot parse range start.");
                        let end = bounds
                            .next()
                            .expect("Missing upper bound of range")
                            .trim()
                            .parse::<usize>()
                            .expect("Cannot parse range end.")
                            + 1;
                        Range { start, end }
                    })
                    .collect::<Vec<Range<usize>>>();
                fields.insert(Field { label, ranges });
            }
            1 => {
                my_ticket = Some(Ticket {
                    numbers: line
                        .split(',')
                        .map(|section| {
                            section
                                .trim()
                                .parse::<usize>()
                                .expect("Cannot parse ticket number.")
                        })
                        .collect::<Vec<usize>>(),
                });
            }
            2 => {
                nearby_tickets.push(Ticket {
                    numbers: line
                        .split(',')
                        .map(|section| {
                            section
                                .trim()
                                .parse::<usize>()
                                .expect("Cannot parse nearby ticket number.")
                        })
                        .collect::<Vec<usize>>(),
                });
            }
            _ => panic!("Unexpected section starting with: {}", line),
        }
    }
    (
        my_ticket.expect("Ticket not issued."),
        fields,
        nearby_tickets,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::day16::{get_input, Field, Ticket};

    #[test]
    fn part1() {
        let (_, fields, nearby_tickets) = get_input();
        let ticket_scanning_error_rate: usize = nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.get_invalid_numbers(&fields))
            .sum();
        println!("Part 1: {}", ticket_scanning_error_rate);
    }

    #[test]
    fn part2() {
        let (my_ticket, fields, nearby_tickets) = get_input();

        // "Now that you've identified which tickets contain invalid values, discard those tickets
        // entirely. Use the remaining valid tickets to determine which field is which."
        let valid_tickets = nearby_tickets
            .iter()
            .filter(|candidate| candidate.is_valid(&fields))
            .collect::<Vec<&Ticket>>();

        // "Using the valid ranges for each field, determine what order the fields appear on the
        // tickets. The order is consistent between all tickets"
        let mut unmapped_indices = (0..my_ticket.numbers.len()).collect::<HashSet<usize>>();
        let mut field_table: HashMap<String, usize> = HashMap::new();
        let mut unmapped_fields = fields.iter().collect::<HashSet<&Field>>();
        while !unmapped_fields.is_empty() {
            let mut indices_to_remove: HashSet<usize> = HashSet::new();
            for field_index in &unmapped_indices {
                let mut candidates = unmapped_fields.clone();
                let mut to_remove: HashSet<&Field> = HashSet::new();
                for ticket in &valid_tickets {
                    let number = ticket.numbers[*field_index];
                    for potential_field in &candidates {
                        if !potential_field.contains(number) {
                            to_remove.insert(potential_field);
                        }
                    }
                    for disqualified in &to_remove {
                        candidates.remove(disqualified);
                    }
                }
                if candidates.is_empty() {
                    panic!("No candidate fields for index: {}", field_index);
                } else if candidates.len() == 1 {
                    // map candidate to index
                    let field = candidates
                        .drain()
                        .next()
                        .expect("There should be exactly one candidate.");
                    field_table.insert(field.label.clone(), *field_index);
                    unmapped_fields.remove(&field);
                    indices_to_remove.insert(*field_index);
                }
            }
            for index in indices_to_remove {
                unmapped_indices.remove(&index);
            }
        }
        // "Once you work out which field is which, look for the six fields on your ticket that
        // start with the word departure. What do you get if you multiply those six values
        // together?"
        let numbers = &my_ticket.numbers;
        let product = field_table
            .iter()
            .filter(|(label, _)| label.starts_with("departure"))
            .map(|(_, number)| *number)
            .map(|number| numbers[number])
            .reduce(|x, y| x * y)
            .expect("Unable to deduce any field position mappings");
        println!("Part 2: {}", product);
    }
}
