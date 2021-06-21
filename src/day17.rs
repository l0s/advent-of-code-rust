// --- Day 17: Conway Cubes ---
// https://adventofcode.com/2020/day/17

use std::collections::{BTreeMap, HashSet};
use std::fmt::{Display, Formatter};
use std::{cmp, fmt};

use crate::get_lines;

/// A signed integer for indexing into an infinite 3-dimensional space
///
/// This can be sized to accommodate the maximum-needed distance from the origin.
type Int = i8;

type Coordinate = (Int, Int, Int); // TODO should this be a struct?

/// An infinite, 3-dimensional grid of Conway Cubes. Each cube is either active or inactive as
/// represented by a `bool`.
pub struct Grid {
    x_bounds: (Int, Int),
    // TODO consider struct for bounds
    y_bounds: (Int, Int),
    z_bounds: (Int, Int),
    map: BTreeMap<Int, BTreeMap<Int, BTreeMap<Int, bool>>>,
}

impl Grid {
    pub fn new(known_cubes: HashSet<(Coordinate, bool)>) -> Grid {
        let mut map = BTreeMap::new();
        let mut x_min: Int = 0;
        let mut x_max: Int = 0;
        let mut y_min: Int = 0;
        let mut y_max: Int = 0;
        let mut z_min: Int = 0;
        let mut z_max: Int = 0;

        for (coordinate, active) in known_cubes.iter() {
            let x_dimension = map.entry(coordinate.0).or_insert_with(BTreeMap::new);
            let y_dimension = x_dimension
                .entry(coordinate.1)
                .or_insert_with(BTreeMap::new);
            y_dimension.insert(coordinate.2, *active);

            x_min = cmp::min(x_min, coordinate.0);
            x_max = cmp::max(x_max, coordinate.0);
            y_min = cmp::min(y_min, coordinate.1);
            y_max = cmp::max(y_max, coordinate.1);
            z_min = cmp::min(z_min, coordinate.2);
            z_max = cmp::max(z_max, coordinate.2);
        }

        Grid {
            x_bounds: (x_min, x_max),
            y_bounds: (y_min, y_max),
            z_bounds: (z_min, z_max),
            map,
        }
    }

    /// Returns: the total number of active Conway Cubes in the unbounded grid
    pub fn count_active(&self) -> usize {
        self.map
            .values()
            .map(|x_dimension| -> usize {
                x_dimension
                    .values()
                    .map(|y_dimension| -> usize {
                        y_dimension.values().filter(|state| **state).count()
                    })
                    .sum()
            })
            .sum()
    }

    /// Determine if the Conway Cube at the given three-dimensional coordinates is active or not.
    fn is_active(&self, coordinates: Coordinate) -> bool {
        if !self.map.contains_key(&coordinates.0) {
            return false;
        }
        let x_dimension = self.map.get(&coordinates.0).unwrap();
        if !x_dimension.contains_key(&coordinates.1) {
            return false;
        }
        let y_dimension = x_dimension.get(&coordinates.1).unwrap();
        *y_dimension.get(&coordinates.2).unwrap_or(&false)
    }

    /// Create a new generation from the current one.
    ///
    /// Returns: a new Grid based on the evaluation of the current state
    pub fn cycle(&self) -> Grid {
        let known_cubes = (self.x_bounds.0 - 1..=self.x_bounds.1 + 1)
            .flat_map(move |x| {
                (self.y_bounds.0 - 1..=self.y_bounds.1 + 1).flat_map(move |y| {
                    (self.z_bounds.0 - 1..=self.z_bounds.1 + 1)
                        .map(move |z| ((x, y, z), self.cycle_cube((x, y, z))))
                })
            })
            .collect::<HashSet<(Coordinate, bool)>>();
        Grid::new(known_cubes)
    }

    fn cycle_cube(&self, coordinates: Coordinate) -> bool {
        let active_neighbours = self
            .get_neighbouring_coordinates(coordinates)
            .filter(|neighbour| self.is_active(*neighbour))
            .count();
        if self.is_active(coordinates) {
            active_neighbours == 2 || active_neighbours == 3
        } else {
            active_neighbours == 3
        }
    }

    fn get_neighbouring_coordinates(
        &self,
        coordinates: Coordinate,
    ) -> impl Iterator<Item = Coordinate> {
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1).flat_map(move |y_offset| {
                (-1..=1).flat_map(move |z_offset| {
                    if x_offset == 0 && y_offset == 0 && z_offset == 0 {
                        None
                    } else {
                        Some((
                            coordinates.0 + x_offset,
                            coordinates.1 + y_offset,
                            coordinates.2 + z_offset,
                        ))
                    }
                })
            })
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Grid x: [{}, {}], y: [{}, {}], z: [{}, {}], active: {}",
            self.x_bounds.0,
            self.x_bounds.1,
            self.y_bounds.0,
            self.y_bounds.1,
            self.z_bounds.0,
            self.z_bounds.1,
            self.count_active()
        )
    }
}

/// Parse the problem input.
///
/// "In the initial state of the pocket dimension, almost all cubes start inactive. The only
/// exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
/// start in the specified active (#) or inactive (.) state."
pub fn get_input() -> Grid {
    let known_cubes = get_lines("day-17-input.txt")
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, state)| ((x as Int, y as Int, 0), state == '#'))
                .collect::<HashSet<(Coordinate, bool)>>()
        })
        .collect::<HashSet<(Coordinate, bool)>>();
    Grid::new(known_cubes)
}

#[cfg(test)]
mod tests {
    use crate::day17::get_input;

    #[test]
    fn part1() {
        let mut grid = get_input();
        for _ in 0..6 {
            grid = grid.cycle();
        }
        let num_active = grid.count_active();
        println!("Part 1: {}", num_active);
    }
}
