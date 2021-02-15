// https://adventofcode.com/2020/day/11

use std::iter::repeat;
use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

use crate::get_lines;

use crate::day11::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use crate::day11::Position::{EmptySeat, Floor, OccupiedSeat};

/// A position in the seating layout
#[derive(Clone, Copy, Eq, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Floor),
            "L" => Ok(EmptySeat),
            "#" => Ok(OccupiedSeat),
            _ => Err(()),
        }
    }
}

/// "The seat layout fits neatly on a grid."
fn read_seat_layout() -> Vec<Vec<Position>> {
    get_lines("/input/day-11-input.txt")
        .map(|line| -> Vec<Position> {
            line.graphemes(true)
                .map(|grapheme| Position::from_str(grapheme))
                .flatten()
                .collect()
        })
        .collect()
}

/// "Now, you just need to model the people who will be arriving shortly. Fortunately, people are
/// entirely predictable and always follow a simple set of rules. All decisions are based on the
/// number of occupied seats adjacent to a given seat (one of the eight positions immediately up,
/// down, left, right, or diagonal from the seat)."
fn update_seat_based_on_neighbours(
    row_index: usize,
    column_index: usize,
    source: &Vec<Vec<Position>>,
) -> Position {
    let row = &source[row_index];
    let has_preceding_row = row_index > 0;
    let has_subsequent_row = row_index < source.len() - 1;
    let has_preceding_column = column_index > 0;
    let has_subsequent_column = column_index < row.len() - 1;
    match source[row_index][column_index] {
        Floor => Floor, // "Floor (.) never changes; seats don't move, and nobody sits on the floor"
        EmptySeat => {
            // "If a seat is empty and there are no occupied seats adjacent to it, the seat becomes
            // occupied."
            let mut has_occupied_neighbour = false;

            // check current row
            if has_preceding_column {
                has_occupied_neighbour |= row[column_index - 1] == OccupiedSeat;
            }
            if has_subsequent_column {
                has_occupied_neighbour |= row[column_index + 1] == OccupiedSeat;
            }
            if has_preceding_row && !has_occupied_neighbour {
                // check preceding row
                let preceding_row = &source[row_index - 1];
                has_occupied_neighbour |= preceding_row[column_index] == OccupiedSeat;
                if has_preceding_column {
                    has_occupied_neighbour |= preceding_row[column_index - 1] == OccupiedSeat;
                }
                if has_subsequent_column {
                    has_occupied_neighbour |= preceding_row[column_index + 1] == OccupiedSeat;
                }
            }
            if has_subsequent_row && !has_occupied_neighbour {
                // check subsequent row
                let subsequent_row = &source[row_index + 1];
                has_occupied_neighbour |= subsequent_row[column_index] == OccupiedSeat;
                if has_preceding_column {
                    has_occupied_neighbour |= subsequent_row[column_index - 1] == OccupiedSeat;
                }
                if has_subsequent_column {
                    has_occupied_neighbour |= subsequent_row[column_index + 1] == OccupiedSeat;
                }
            }

            if has_occupied_neighbour {
                EmptySeat
            } else {
                OccupiedSeat
            }
        }
        OccupiedSeat => {
            // "If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
            // the seat becomes empty."

            let mut occupied_neighbours = 0u8;

            // check current row
            if has_preceding_column && row[column_index - 1] == OccupiedSeat {
                occupied_neighbours += 1;
            }
            if has_subsequent_column && row[column_index + 1] == OccupiedSeat {
                occupied_neighbours += 1;
            }
            if has_preceding_row {
                // check preceding row
                let preceding_row = &source[row_index - 1];

                if preceding_row[column_index] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
                if has_preceding_column && preceding_row[column_index - 1] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
                if has_subsequent_column && preceding_row[column_index + 1] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
            }
            if has_subsequent_row {
                // check subsequent row
                let subsequent_row = &source[row_index + 1];

                if subsequent_row[column_index] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
                if has_preceding_column && subsequent_row[column_index - 1] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
                if has_subsequent_column && subsequent_row[column_index + 1] == OccupiedSeat {
                    occupied_neighbours += 1;
                }
            }
            if occupied_neighbours >= 4 {
                EmptySeat
            } else {
                OccupiedSeat
            }
        }
    }
}

/// Update each seat based on its adjacent seats. Perform all updates simultaneously.
fn update_seats_based_on_neighbours(seat_layout: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
    let mut new_layout = seat_layout.clone();
    for i in 0..seat_layout.len() {
        let row = &seat_layout[i];
        for j in 0..row.len() {
            new_layout[i][j] = update_seat_based_on_neighbours(i, j, seat_layout);
        }
    }
    new_layout
}

/// Count the number of occupied seats in the given layout.
fn count_occupied(seat_layout: &Vec<Vec<Position>>) -> u16 {
    let mut count = 0u16;
    for row in seat_layout {
        for cell in row {
            count += match cell {
                OccupiedSeat => 1,
                _ => 0,
            }
        }
    }
    count
}

enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    fn has_occupied_seat_in_sight(
        &self,
        vantage_row: usize,
        vantage_column: usize,
        source: &Vec<Vec<Position>>,
        max_columns: usize,
    ) -> bool {
        for (i, j) in
            self.get_visible_seat_indices(vantage_row, vantage_column, source.len(), max_columns)
        {
            match source[i][j] {
                EmptySeat => return false,
                OccupiedSeat => return true,
                _ => continue,
            }
        }
        false
    }
    fn get_visible_seat_indices(
        &self,
        vantage_row: usize,
        vantage_column: usize,
        max_rows: usize,
        max_columns: usize,
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        match self {
            Direction::North => {
                let row_indices = (0..vantage_row).rev();
                let col_indices = repeat(vantage_column);
                Box::new(row_indices.zip(col_indices))
            }
            Direction::NorthWest => {
                let row_indices = (0..vantage_row).rev();
                let col_indices = (0..vantage_column).rev();
                Box::new(row_indices.zip(col_indices))
            }
            Direction::West => {
                let row_indices = repeat(vantage_row);
                let col_indices = (0..vantage_column).rev();
                Box::new(row_indices.zip(col_indices))
            }
            Direction::SouthWest => {
                let row_indices = vantage_row + 1..max_rows;
                let col_indices = (0..vantage_column).rev();
                Box::new(row_indices.zip(col_indices))
            }
            Direction::South => {
                let row_indices = vantage_row + 1..max_rows;
                let col_indices = repeat(vantage_column);
                Box::new(row_indices.zip(col_indices))
            }
            Direction::SouthEast => {
                let row_indices = vantage_row + 1..max_rows;
                let col_indices = vantage_column + 1..max_columns;
                Box::new(row_indices.zip(col_indices))
            }
            Direction::East => {
                let row_indices = repeat(vantage_row);
                let col_indices = vantage_column + 1..max_columns;
                Box::new(row_indices.zip(col_indices))
            }
            Direction::NorthEast => {
                let row_indices = (0..vantage_row).rev();
                let col_indices = vantage_column + 1..max_columns;
                Box::new(row_indices.zip(col_indices))
            }
        }
    }
}

/// Update a seat based on what is visible from that seat's vantage point.
///
/// "People don't just care about adjacent seats - they care about the first seat they can see in
/// each of those eight directions!
///
/// Now, instead of considering just the eight immediately adjacent seats, consider the first seat
/// in each of those eight directions."
///
/// Parameters:
/// * `row_index` - the row of the seat to update
/// * `column_index` - the column of the seat to update
/// * `source` - the full seating layout
///
/// Returns: the new seating status
fn update_seat_based_on_visibility(
    row_index: usize,
    column_index: usize,
    source: &Vec<Vec<Position>>,
) -> Position {
    match source[row_index][column_index] {
        Floor => Floor,
        EmptySeat => {
            // "empty seats that see no occupied seats become occupied"
            let row = &source[row_index];
            let mut has_occupied_seat_in_sight = false;
            for direction in [
                North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast,
            ]
            .iter()
            {
                has_occupied_seat_in_sight |= direction.has_occupied_seat_in_sight(
                    row_index,
                    column_index,
                    source,
                    row.len(),
                );
                if has_occupied_seat_in_sight {
                    break;
                }
            }
            if has_occupied_seat_in_sight {
                EmptySeat
            } else {
                OccupiedSeat
            }
        }
        OccupiedSeat => {
            // "it now takes five or more visible occupied seats for an occupied seat to become
            // empty"
            let row = &source[row_index];
            let mut visible_occupied_seats = 0u8;
            for direction in [
                North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast,
            ]
            .iter()
            {
                let has_occupied_seat_in_sight = direction.has_occupied_seat_in_sight(
                    row_index,
                    column_index,
                    source,
                    row.len(),
                );
                if has_occupied_seat_in_sight {
                    visible_occupied_seats += 1;
                }
                if visible_occupied_seats >= 5 {
                    break;
                }
            }
            if visible_occupied_seats >= 5 {
                EmptySeat
            } else {
                OccupiedSeat
            }
        }
    }
}

/// Update all of the seats simultaneously based on what is visible from each individual seat.
fn update_seats_based_on_visibility(seat_layout: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
    let mut new_layout = seat_layout.clone();
    for i in 0..seat_layout.len() {
        let row = &seat_layout[i];
        for j in 0..row.len() {
            new_layout[i][j] = update_seat_based_on_visibility(i, j, seat_layout);
        }
    }
    new_layout
}

mod tests {
    use crate::day11::{
        count_occupied, read_seat_layout, update_seats_based_on_neighbours,
        update_seats_based_on_visibility,
    };

    #[test]
    fn part1() {
        let mut layout = read_seat_layout();
        // "Simulate your seating area by applying the seating rules repeatedly until no seats
        // change state."
        loop {
            let new_layout = update_seats_based_on_neighbours(&layout);
            if new_layout == layout {
                break;
            }
            layout = new_layout;
        }
        println!("Part 1: {}", count_occupied(&layout));
    }

    #[test]
    fn part2() {
        let mut layout = read_seat_layout();
        loop {
            let new_layout = update_seats_based_on_visibility(&layout);
            if new_layout == layout {
                break;
            }
            layout = new_layout;
        }
        println!("Part 2: {}", count_occupied(&layout));
    }
}
