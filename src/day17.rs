// --- Day 17: Conway Cubes ---
// https://adventofcode.com/2020/day/17

use std::cmp;
use std::collections::{BTreeMap, HashSet};

use crate::get_lines;

/// A signed integer for indexing into an infinite 3-dimensional space
///
/// This can be sized to accommodate the maximum-needed distance from the origin.
type Int = i8;

/// The location of a Conway Cube in three-dimensional space
///
/// Each location has 26 adjacent neighbours.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct SpatialCoordinate {
    x: Int,
    y: Int,
    z: Int,
}

impl SpatialCoordinate {
    /// Find the coordinate at the specified offset
    ///
    /// Returns:
    /// - `None` - if all the offsets are zero
    /// - `Some(SpatialCoordinate)` - The coordinate at the specified offset
    pub fn offset(&self, x_offset: Int, y_offset: Int, z_offset: Int) -> Option<SpatialCoordinate> {
        if x_offset == 0 && y_offset == 0 && z_offset == 0 {
            None
        } else {
            Some(
                SpatialCoordinate {
                    x: &self.x + x_offset,
                    y: &self.y + y_offset,
                    z: &self.z + z_offset,
                }
            )
        }
    }
}

/// The location of a Hyper Conway Cube in four-dimensional space
///
/// Each location has 80 adjacent neighbours.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct SpaceTimeCoordinate {
    x: Int,
    y: Int,
    z: Int,
    w: Int,
}

impl SpaceTimeCoordinate {
    /// Find the coordinate at the specified offset
    ///
    /// Returns:
    /// - `None` - if all the offsets are zero
    /// - `Some(SpatialCoordinate)` - The coordinate at the specified offset
    pub fn offset(&self, x_offset: Int, y_offset: Int, z_offset: Int, w_offset: Int) -> Option<SpaceTimeCoordinate> {
        if x_offset == 0 && y_offset == 0 && z_offset == 0 && w_offset == 0 {
            None
        } else {
            Some(
                SpaceTimeCoordinate {
                    x: &self.x + x_offset,
                    y: &self.y + y_offset,
                    z: &self.z + z_offset,
                    w: &self.w + w_offset,
                }
            )
        }
    }
}

/// Identifies the boundaries of the known space for a single dimension
///
/// Note that during a cycle, the cubes one unit beyond the bounds *may* update.
#[derive(Debug)]
pub struct Bounds {
    /// The lower bound, inclusive and strictly less than or equal to `upper`
    lower: Int,
    /// The upper bound, inclusive and strictly greater than or equal to `lower`
    upper: Int,
}

/// An infinite, 3-dimensional grid of Conway Cubes. Each cube is either active or inactive as
/// represented by a `bool`.
pub struct SpatialGrid {
    x_bounds: Bounds,
    y_bounds: Bounds,
    z_bounds: Bounds,
    map: BTreeMap<Int, BTreeMap<Int, BTreeMap<Int, bool>>>,
}

impl SpatialGrid {
    pub fn new(known_cubes: HashSet<(SpatialCoordinate, bool)>) -> SpatialGrid {
        let mut map = BTreeMap::new();
        let mut x_min: Int = 0;
        let mut x_max: Int = 0;
        let mut y_min: Int = 0;
        let mut y_max: Int = 0;
        let mut z_min: Int = 0;
        let mut z_max: Int = 0;

        for (coordinate, active) in known_cubes.iter() {
            let x_dimension = map.entry(coordinate.x).or_insert_with(BTreeMap::new);
            let y_dimension = x_dimension
                .entry(coordinate.y)
                .or_insert_with(BTreeMap::new);
            y_dimension.insert(coordinate.z, *active);

            x_min = cmp::min(x_min, coordinate.x);
            x_max = cmp::max(x_max, coordinate.x);
            y_min = cmp::min(y_min, coordinate.y);
            y_max = cmp::max(y_max, coordinate.y);
            z_min = cmp::min(z_min, coordinate.z);
            z_max = cmp::max(z_max, coordinate.z);
        }

        SpatialGrid {
            x_bounds: Bounds { lower: x_min, upper: x_max },
            y_bounds: Bounds { lower: y_min, upper: y_max },
            z_bounds: Bounds { lower: z_min, upper: z_max },
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
    fn is_active(&self, coordinates: &SpatialCoordinate) -> bool {
        // TODO should I use `match`?
        if !self.map.contains_key(&coordinates.x) {
            return false;
        }
        let x_dimension = self.map.get(&coordinates.x).unwrap();
        if !x_dimension.contains_key(&coordinates.y) {
            return false;
        }
        let y_dimension = x_dimension.get(&coordinates.y).unwrap();
        *y_dimension.get(&coordinates.z).unwrap_or(&false)
    }

    /// Create a new generation from the current one.
    ///
    /// Returns: a new Grid based on the evaluation of the current state
    pub fn cycle(&self) -> SpatialGrid {
        let known_cubes = (&self.x_bounds.lower - 1..=&self.x_bounds.upper + 1)
            .flat_map(move |x| {
                (&self.y_bounds.lower - 1..=&self.y_bounds.upper + 1).flat_map(move |y| {
                    (&self.z_bounds.lower - 1..=&self.z_bounds.upper + 1)
                        .map(move |z| {
                            let coordinate = SpatialCoordinate { x, y, z };
                            (coordinate, self.cycle_cube(&coordinate))
                        })
                })
            })
            .collect::<HashSet<(SpatialCoordinate, bool)>>();
        SpatialGrid::new(known_cubes)
    }

    fn cycle_cube(&self, coordinates: &SpatialCoordinate) -> bool {
        let active_neighbours = self
            .get_neighbouring_coordinates(coordinates)
            .filter(|neighbour| self.is_active(neighbour))
            .count();
        if self.is_active(coordinates) {
            active_neighbours == 2 || active_neighbours == 3
        } else {
            active_neighbours == 3
        }
    }

    fn get_neighbouring_coordinates<'a>(
        &self,
        coordinates: &'a SpatialCoordinate,
    ) -> impl Iterator<Item=SpatialCoordinate> + 'a {
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1).flat_map(move |y_offset| {
                (-1..=1).flat_map(move |z_offset| coordinates.offset(x_offset, y_offset, z_offset))
            })
        })
    }
}

pub struct SpaceTimeGrid {
    x_bounds: Bounds,
    y_bounds: Bounds,
    z_bounds: Bounds,
    w_bounds: Bounds,

    // TODO after 1 cycle, this isn't sparse anymore
    map: BTreeMap<Int, BTreeMap<Int, BTreeMap<Int, BTreeMap<Int, bool>>>>,
}

impl SpaceTimeGrid {
    pub fn new(known_cubes: HashSet<(SpaceTimeCoordinate, bool)>) -> SpaceTimeGrid {
        let mut map = BTreeMap::new();
        let mut x_min: Int = 0;
        let mut x_max: Int = 0;
        let mut y_min: Int = 0;
        let mut y_max: Int = 0;
        let mut z_min: Int = 0;
        let mut z_max: Int = 0;
        let mut w_min: Int = 0;
        let mut w_max: Int = 0;

        for (coordinate, active) in known_cubes.iter() {
            let x_dimension = map
                .entry(coordinate.x)
                .or_insert_with(BTreeMap::new);
            let y_dimension = x_dimension
                .entry(coordinate.y)
                .or_insert_with(BTreeMap::new);
            let z_dimension = y_dimension
                .entry(coordinate.z)
                .or_insert_with(BTreeMap::new);
            z_dimension.insert(coordinate.w, *active);

            x_min = cmp::min(x_min, coordinate.x);
            x_max = cmp::max(x_max, coordinate.x);
            y_min = cmp::min(y_min, coordinate.y);
            y_max = cmp::max(y_max, coordinate.y);
            z_min = cmp::min(z_min, coordinate.z);
            z_max = cmp::max(z_max, coordinate.z);
            w_min = cmp::min(w_min, coordinate.w);
            w_max = cmp::max(w_max, coordinate.w);
        }

        SpaceTimeGrid {
            x_bounds: Bounds { lower: x_min, upper: x_max },
            y_bounds: Bounds { lower: y_min, upper: y_max },
            z_bounds: Bounds { lower: z_min, upper: z_max },
            w_bounds: Bounds { lower: w_min, upper: w_max },
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
                        y_dimension
                            .values()
                            .map(|z_dimension| -> usize {
                                z_dimension
                                    .values()
                                    .filter(|state| **state)
                                    .count()
                            })
                            .sum()
                    })
                    .sum()
            })
            .sum()
    }

    /// Determine if the Hyper Conway Cube at the given three-dimensional coordinates is active or not.
    fn is_active(&self, coordinates: &SpaceTimeCoordinate) -> bool {
        if let Some(x_dimension) = self.map.get(&coordinates.x) {
            if let Some(y_dimension) = x_dimension.get(&coordinates.y) {
                if let Some(z_dimension) = y_dimension.get(&coordinates.z) {
                    return *z_dimension.get(&coordinates.w)
                        .unwrap_or(&false);
                }
            }
        }
        false
    }

    /// Create a new generation from the current one.
    ///
    /// Returns: a new Grid based on the evaluation of the current state
    pub fn cycle(&self) -> SpaceTimeGrid {
        let known_cubes = (&self.x_bounds.lower - 1..=&self.x_bounds.upper + 1).flat_map(move |x| {
            (&self.y_bounds.lower - 1..=&self.y_bounds.upper + 1).flat_map(move |y| {
                (&self.z_bounds.lower - 1..=&self.z_bounds.upper + 1).flat_map(move |z| {
                    (&self.w_bounds.lower - 1..=&self.w_bounds.upper + 1).map(move |w| {
                        let coordinate = SpaceTimeCoordinate { x, y, z, w };
                        // TODO can save space if cube is not active
                        let active = self.cycle_cube(&coordinate);
                        (coordinate, active)
                    })
                })
            })
        }).collect::<HashSet<(SpaceTimeCoordinate, bool)>>();
        SpaceTimeGrid::new(known_cubes)
    }

    fn cycle_cube(&self, coordinates: &SpaceTimeCoordinate) -> bool {
        let active_neighbours = self
            .get_neighbouring_coordinates(coordinates)
            .filter(|neighbour| self.is_active(neighbour))
            .count();
        if self.is_active(coordinates) {
            active_neighbours == 2 || active_neighbours == 3
        } else {
            active_neighbours == 3
        }
    }

    fn get_neighbouring_coordinates<'a>(
        &self,
        coordinates: &'a SpaceTimeCoordinate,
    ) -> impl Iterator<Item=SpaceTimeCoordinate> + 'a {
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1).flat_map(move |y_offset| {
                (-1..=1).flat_map(move |z_offset| {
                    (-1..=1).flat_map(move |w_offset| coordinates.offset(x_offset, y_offset, z_offset, w_offset))
                })
            })
        })
    }
}

/// Parse the problem input.
///
/// "In the initial state of the pocket dimension, almost all cubes start inactive. The only
/// exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
/// start in the specified active (#) or inactive (.) state."
pub fn parse_3d_grid() -> SpatialGrid {
    let known_cubes = get_lines("day-17-input.txt")
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, state)| (SpatialCoordinate { x: x as Int, y: y as Int, z: 0 }, state == '#'))
                .collect::<HashSet<(SpatialCoordinate, bool)>>() // TODO do I need this line?
        })
        .collect::<HashSet<(SpatialCoordinate, bool)>>();
    SpatialGrid::new(known_cubes)
}

/// Parse the problem input.
///
/// "In the initial state of the pocket dimension, almost all cubes start inactive. The only
/// exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
/// start in the specified active (#) or inactive (.) state."
pub fn parse_4d_grid() -> SpaceTimeGrid {
    let known_cubes = get_lines("day-17-input.txt")
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, state)| (SpaceTimeCoordinate { x: x as Int, y: y as Int, z: 0, w: 0 }, state == '#'))
                .collect::<HashSet<(SpaceTimeCoordinate, bool)>>() // TODO do I need this line?
        })
        .collect::<HashSet<(SpaceTimeCoordinate, bool)>>();
    SpaceTimeGrid::new(known_cubes)
}

#[cfg(test)]
mod tests {
    use crate::day17::{parse_3d_grid, parse_4d_grid};

    #[test]
    fn part1() {
        let mut grid = parse_3d_grid();
        for _ in 0..6 {
            grid = grid.cycle();
        }
        let num_active = grid.count_active();
        println!("Part 1: {}", num_active);
    }

    #[test]
    fn part2() {
        let mut grid = parse_4d_grid();
        for _ in 0..6 {
            grid = grid.cycle();
        }
        let num_active = grid.count_active();
        println!("Part 2: {}", num_active);
    }
}
