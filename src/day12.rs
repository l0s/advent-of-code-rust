// https://adventofcode.com/2020/day/12

use core::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::day12::Action::{East, Forward, Left, North, Right, South, West};
use crate::day12::ParseError::{
    InstructionTooShort, InvalidAction, InvalidValue, ValueInappropriateForAction,
};
use crate::get_lines;

#[derive(Debug)]
pub enum ParseError {
    InvalidAction(String),
    InstructionTooShort(String),
    InvalidValue(ParseIntError),
    ValueInappropriateForAction(Action, i16),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidAction(action) => write!(f, "Invalid action: {}", action),
            ParseError::InstructionTooShort(instruction) => {
                write!(f, "Instruction too short: {}", instruction)
            }
            ParseError::InvalidValue(error) => write!(f, "Could not parse value: {}", error),
            ParseError::ValueInappropriateForAction(action, value) => {
                write!(
                    f,
                    "Value {} is not appropriate for action {:?}",
                    value, action
                )
            }
        }
    }
}

/// An action that the ferry's navigation computer can produce.
#[derive(Debug)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "N" => Ok(North),
            "S" => Ok(South),
            "E" => Ok(East),
            "W" => Ok(West),
            "L" => Ok(Left),
            "R" => Ok(Right),
            "F" => Ok(Forward),
            _ => Err(InvalidAction(code.to_owned())),
        }
    }
}

/// An instruction produced by the ferry's navigation computer. A series of NavigationInstructions
/// will allow the ferry/ship to evade the storm and make it to the island.
///
/// Each instruction consists of an Action and an integer value indicating the magnitude of that
/// action.
pub struct NavigationInstruction {
    action: Action,
    value: i16,
}

impl FromStr for NavigationInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(InstructionTooShort(s.to_owned()));
        }
        let (action, value) = s.split_at(1);
        action
            .parse::<Action>()
            .and_then(|action| -> Result<Self, Self::Err> {
                value.parse::<i16>().map_err(InvalidValue).and_then(
                    |value| -> Result<Self, Self::Err> {
                        match action {
                            Left | Right => {
                                if value % 90 != 0 {
                                    return Err(ValueInappropriateForAction(action, value));
                                }
                            }
                            _ => {}
                        }
                        Ok(NavigationInstruction { action, value })
                    },
                )
            })
    }
}

pub struct Point {
    /// longitudinal position,
    /// positive numbers are to the East and negative numbers are to the West
    x: i16,
    /// latitudinal position,
    /// positive numbers are to the North and negative numbers are to the South
    y: i16,
}

impl Point {
    /// The sum of the absolute longitudinal and latitudinal distances from the origin
    pub fn get_manhattan_distance(&self) -> u16 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl NavigationInstruction {
    /// Interpret the instruction to move the ferry
    ///
    /// Parameters
    /// - `bearing` - (degrees) the direction the ferry is currently facing.
    ///     0° means it's facing North,
    ///     90° means it's facing East,
    ///     180° means it's facing South,
    ///     270° means it's facing West,
    /// - `position` - the current position of the ferry
    ///
    /// Returns
    /// - `Ok` - the ferry's new bearing and new position
    /// - `Err` - if the instruction is to move "forward" but the provided `bearing` is not
    ///           supported
    pub fn move_ship(&self, bearing: u16, position: Point) -> Result<(u16, Point), String> {
        match self.action {
            North => Ok((
                bearing,
                Point {
                    x: position.x,
                    y: position.y + self.value,
                },
            )),
            South => Ok((
                bearing,
                Point {
                    x: position.x,
                    y: position.y - self.value,
                },
            )),
            East => Ok((
                bearing,
                Point {
                    x: position.x + self.value,
                    y: position.y,
                },
            )),
            West => Ok((
                bearing,
                Point {
                    x: position.x - self.value,
                    y: position.y,
                },
            )),
            Left => {
                let relative_bearing = bearing as i16 - self.value;
                let bearing = if relative_bearing < 0 {
                    (relative_bearing + 360) as u16
                } else {
                    relative_bearing as u16
                };
                Ok((bearing, position))
            }
            Right => Ok(((bearing + self.value as u16) % 360, position)),
            Forward => match bearing {
                0 => Ok((
                    bearing,
                    Point {
                        x: position.x,
                        y: position.y + self.value,
                    },
                )),
                90 => Ok((
                    bearing,
                    Point {
                        x: position.x + self.value,
                        y: position.y,
                    },
                )),
                180 => Ok((
                    bearing,
                    Point {
                        x: position.x,
                        y: position.y - self.value,
                    },
                )),
                270 => Ok((
                    bearing,
                    Point {
                        x: position.x - self.value,
                        y: position.y,
                    },
                )),
                _ => Err("Unsupported bearing: ".to_owned() + &bearing.to_string()),
            },
        }
    }

    /// Interpret the instruction to move the ferry's waypoint and possibly also the ferry itself
    ///
    /// Parameters
    /// - `waypoint` - the current position of the ferry's waypoint
    /// - `ship` - the current position of the ferry
    ///
    /// Returns - The new positions of the waypoint and the ferry
    pub fn move_ship_using_waypoint(&self, waypoint: Point, ship: Point) -> (Point, Point) {
        match self.action {
            North => (
                Point {
                    x: waypoint.x,
                    y: waypoint.y + self.value,
                },
                ship,
            ),
            South => (
                Point {
                    x: waypoint.x,
                    y: waypoint.y - self.value,
                },
                ship,
            ),
            East => (
                Point {
                    x: waypoint.x + self.value,
                    y: waypoint.y,
                },
                ship,
            ),
            West => (
                Point {
                    x: waypoint.x - self.value,
                    y: waypoint.y,
                },
                ship,
            ),
            Left => {
                // rotate the waypoint around the ship, anti-clockwise (when viewed from above),
                // by the number of degrees specified in the instruction

                // capture the vector between the waypoint and the ship
                let mut longitudinal_difference = waypoint.x - ship.x;
                let mut latitudinal_difference = waypoint.y - ship.y;

                // relocate the waypoint relative to the ship
                let mut waypoint = Point {
                    x: waypoint.x,
                    y: waypoint.y,
                };
                for _ in 0..self.value / 90 {
                    waypoint.x = ship.x - latitudinal_difference;
                    waypoint.y = ship.y + longitudinal_difference;
                    longitudinal_difference = waypoint.x - ship.x;
                    latitudinal_difference = waypoint.y - ship.y;
                }
                (waypoint, ship)
            }
            Right => {
                // rotate the waypoint around the ship clockwise (when viewed from above),
                // by the number of degrees specified in the instruction

                // capture the vector between the waypoint and the ship
                let mut longitudinal_difference = waypoint.x - ship.x;
                let mut latitudinal_difference = waypoint.y - ship.y;

                // relocate the waypoint relative to the ship
                let mut waypoint = Point {
                    x: waypoint.x,
                    y: waypoint.y,
                };
                for _ in 0..self.value / 90 {
                    waypoint.x = ship.x + latitudinal_difference;
                    waypoint.y = ship.y - longitudinal_difference;
                    longitudinal_difference = waypoint.x - ship.x;
                    latitudinal_difference = waypoint.y - ship.y;
                }
                (waypoint, ship)
            }
            Forward => {
                // capture the vector between the waypoint and the ship
                let longitudinal_difference = waypoint.x - ship.x;
                let latitudinal_difference = waypoint.y - ship.y;

                let mut waypoint = Point {
                    x: waypoint.x,
                    y: waypoint.y,
                };
                let mut ship = Point {
                    x: ship.x,
                    y: ship.y,
                };

                ship.x += longitudinal_difference * self.value;
                ship.y += latitudinal_difference * self.value;

                // "The waypoint is relative to the ship; that is, if the ship moves, the waypoint
                // moves with it."
                waypoint.x = ship.x + longitudinal_difference;
                waypoint.y = ship.y + latitudinal_difference;

                (waypoint, ship)
            }
        }
    }
}

pub fn get_instructions() -> impl Iterator<Item = Result<NavigationInstruction, String>> {
    get_lines("day-12-input.txt")
        .map(|line| line.parse::<NavigationInstruction>())
        .map(|parse_result| parse_result.map_err(|error| error.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::day12::{get_instructions, Point};

    #[test]
    fn part1() {
        let (_, position) = get_instructions()
            .try_fold(
                (90u16, Point { x: 0, y: 0 }), // "The ship starts by facing east."
                |coordinates, result| {
                    result
                        .and_then(|instruction| instruction.move_ship(coordinates.0, coordinates.1))
                },
            )
            .expect("Invalid instruction found");

        println!("Part 1: {}", position.get_manhattan_distance());
    }

    #[test]
    fn part2() {
        let (_, ship) = get_instructions()
            // "The waypoint starts 10 units east and 1 unit north relative to the ship."
            .try_fold(
                (Point { x: 10, y: 1 }, Point { x: 0, y: 0 }),
                |positions, result| {
                    result.map(|instruction| {
                        instruction.move_ship_using_waypoint(positions.0, positions.1)
                    })
                },
            )
            .expect("Invalid instruction found");
        println!("Part 2: {}", ship.get_manhattan_distance());
    }
}
