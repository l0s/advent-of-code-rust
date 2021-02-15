// https://adventofcode.com/2020/day/13

use core::fmt;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use crate::get_lines;

use crate::day13::ParseError::{EarliestDepartureNotSpecified, InvalidBusId, InvalidDepartureTime,
                               NoBusesSpecified};

/// A shuttle bus that departs from the sea port, travels to the airport, then several other
/// destinations, then returns to the sea port
#[derive(Debug, Copy, Clone)]
struct Bus {
    /// The time it takes the bus to complete a circuit in minutes, which also uniquely identifies
    /// the bus
    id: u16,

    /// The bus' position in the time table
    index: usize,
}

impl Bus {
    /// Determine how long a passenger would have to wait (in minutes) to board this bus at the sea
    /// port.
    ///
    /// Parameters:
    /// - `earliest_departure` - the earliest time (in minutes) at which the passenger can board
    ///                          _any_ bus at the sea port
    ///
    /// Returns: the amount of time in minutes the passenger would have to wait for this particular
    ///          bus
    fn get_time_to_wait(&self, earliest_departure: u32) -> u32 {
        let missed_count: u32 = earliest_departure / self.id as u32; // round down
        ((missed_count + 1) * self.id as u32) - earliest_departure
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "Bus( id: {}, index: {} )",
               self.id, self.index)
    }
}

impl Eq for Bus {}

impl PartialEq for Bus {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

/// A bus that **a specific passenger** is considering boarding from the sea port
#[derive(Debug, Copy, Clone)]
struct BusCandidate {
    /// The bus under consideration
    bus: Bus,

    /// The earliest time the passenger can board _any_ bus from the sea port
    earliest_departure: u32,

    /// The amount of time the passenger would need to wait for this particular bus at the sea port
    time_to_wait: u32,
}

impl Display for BusCandidate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "BusCandidate( bus: {}, earliest_departure: {}, time_to_wait: {}",
               self.bus, self.earliest_departure, self.time_to_wait)
    }
}

impl Eq for BusCandidate {}

impl PartialEq for BusCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.bus == other.bus
    }

    fn ne(&self, other: &Self) -> bool {
        self.bus != other.bus
    }
}

impl PartialOrd for BusCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BusCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time_to_wait < other.time_to_wait {
            return Ordering::Less;
        } else if self.time_to_wait > other.time_to_wait {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

/// An error that could arise from parsing the problem input
#[derive(Debug, Clone)]
enum ParseError {
    InvalidBusId(ParseIntError),
    EarliestDepartureNotSpecified,
    InvalidDepartureTime(ParseIntError),
    NoBusesSpecified,
}

/// Parse a bus time table on behalf of a specific passenger.
///
/// Parameters:
/// - `earliest_departure` - The earliest time in minutes the passenger can board a shuttle bus
/// - `string` - A comma-delimited bus time table. Each entry will either be an 'x' representing an
///              out-of-service bus or a bus ID.
///
/// Returns: All of the bus candidates. For any malformed bus value, a `ParseError` is returned.
fn parse_buses(earliest_departure: u32, string: String) -> Vec<Result<BusCandidate, ParseError>> {
    string.split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(index, id)| -> Result<BusCandidate, ParseError> {
            let bus_id = id.parse::<u16>();
            if bus_id.is_err() {
                return Err(InvalidBusId(bus_id.unwrap_err()));
            }
            let bus_id = bus_id.unwrap();
            let bus = Bus { id: bus_id, index };
            if earliest_departure % bus_id as u32 == 0 {
                return Ok(BusCandidate { bus, earliest_departure, time_to_wait: 0 });
            }
            Ok(BusCandidate {
                bus,
                earliest_departure,
                time_to_wait: bus.get_time_to_wait(earliest_departure),
            })
        }).collect()
}

/// Parse the problem input
///
/// Returns:
/// - `Ok` - All of the bus candidates given the passenger's earliest departure time
/// - `Err(ParseError)` - The first parsing error encountered.
fn parse_input() -> Result<Vec<BusCandidate>, ParseError> {
    let mut lines = get_lines("/input/day-13-input.txt");
    let earliest_departure = lines.next();
    if earliest_departure.is_none() {
        return Err(EarliestDepartureNotSpecified);
    }
    let earliest_departure = earliest_departure.unwrap();
    let earliest_departure = earliest_departure.parse::<u32>();
    if earliest_departure.is_err() {
        return Err(InvalidDepartureTime(earliest_departure.unwrap_err()));
    }
    let earliest_departure = earliest_departure.unwrap();
    let buses = lines.next();
    if buses.is_none() {
        return Err(NoBusesSpecified);
    }
    let buses = buses.unwrap();
    let buses = parse_buses(earliest_departure, buses);
    let mut errors = buses.iter()
        .filter(|result| result.is_err())
        .map(move |result| result.as_ref().unwrap_err().to_owned());
    match errors.next() {
        None => {}
        Some(error) => return Err(error),
    }
    let buses = buses.iter()
        .map(move |result| result.as_ref().unwrap().to_owned())
        .collect::<Vec<BusCandidate>>();
    Ok(buses)
}

/// Find the modular multiplicative inverse
///
/// Parameters:
/// - `partial_product` - the number by which the result can be multiplied to yield a value
///    equivalent to `1 (mod mod_space)`
/// - `mod_space` - the modulus
///
/// Returns: The smallest number _x_ such that `mod_space` evenly divides
///          `( partial_product * x ) - 1`.
fn find_inverse(partial_product: u64, mod_space: u16) -> u64 {
    let mod_space = mod_space as u64;
    let mut multiplier = 1u64;
    loop {
        if (partial_product * multiplier) % mod_space == 1 {
            return multiplier;
        }
        multiplier += 1;
    }
}

mod tests {
    use crate::day13::{Bus, find_inverse, parse_input};

    /// Find the earliest bus you can take to the airport.
    #[test]
    fn part1() {
        let mut buses = parse_input().expect("Error parsing input");
        buses.sort();
        let first_bus = buses.first();
        let first_bus = first_bus.expect("No buses found.");
        let result = first_bus.bus.id as u32 * first_bus.time_to_wait;
        println!("Part 1: {}", result);
    }

    /// Find the earliest timestamp such that the first bus departs at that time and each subsequent
    /// listed bus departs at that subsequent minute.
    #[test]
    fn part2() {
        let buses = parse_input().expect("Error parsing input")
            .iter()
            .map(|candidate| candidate.bus)
            .collect::<Vec<Bus>>();

        // Apply the Chinese Remainder Theorem
        // Treat each bus as a modular expression of the form `x ≡ id - index (mod id)`.
        // Solve for _x_.

        // For each bus, multiply its placement (index) in the mod space (circuit time), the product
        // of the other mod spaces, and its inverse with regard to the other mod spaces, then sum
        // the result. This is _a_ timestamp at which each bus departs offset by its position in the
        // time table. However, initially, it will not contain the earliest departure time.
        let mut sum = 0u64;

        // Multiply all of the mod spaces (circuit times or bus IDs) to yield the spacing between
        // each possible solution.
        let mut product_of_mod_spaces = 1u64;
        for bus in buses.clone() {
            product_of_mod_spaces *= bus.id as u64;

            // Multiply by all of the other mod spaces (circuit times or bus IDs) in order to cancel
            // them out while applying `mod bus_id`.
            let product_of_other_mod_spaces = buses.clone()
                .iter()
                .filter(|other| **other != bus)
                .fold(1u64, |acc, other| acc * other.id as u64);

            let inverse =
                find_inverse(product_of_other_mod_spaces, bus.id);

            let index = bus.index as u16;
            if bus.id > bus.index as u16 {
                let remainder = ((bus.id - index) % bus.id) as u64;
                sum += remainder * product_of_other_mod_spaces * inverse;
            } else {
                let remainder = ((index - bus.id) % bus.id) as u64;
                sum -= remainder * product_of_other_mod_spaces * inverse;
            }
        }
        // Repeatedly subtract the product of all the mod spaces to find the earliest possible
        // departure time that satisfies the constraints.
        let iteration = sum / product_of_mod_spaces;
        sum -= product_of_mod_spaces * iteration;
        println!("Part 2: {}", sum);
    }
}