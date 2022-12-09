use crate::day09::Direction::{Down, Left, Right, Up};
use crate::get_lines;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// --- Day 9: Rope Bridge ---
/// https://adventofcode.com/2022/day/9

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn distance(&self, other: &Self) -> usize {
        ((self.x as f64 - other.x as f64).powf(2f64) + (self.y as f64 - other.y as f64).powf(2f64))
            .sqrt() as usize
    }

    pub fn step(&mut self, distance: (i32, i32)) {
        self.x += distance.0;
        self.y += distance.1;
    }

    pub fn step_towards(&mut self, leader: &Self) {
        let x_distance = match leader.x.cmp(&self.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let y_distance = match leader.y.cmp(&self.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        self.step((x_distance, y_distance));
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(&self) -> (i32, i32) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

pub struct Instruction {
    direction: Direction,
    distance: u16,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split(' ');
        let direction = components
            .next()
            .expect("No direction specified")
            .parse::<Direction>()
            .expect("Unparseable direction");
        let distance = components
            .next()
            .expect("No distance specified")
            .parse::<u16>()
            .expect("Unparseable distance");
        Ok(Self {
            direction,
            distance,
        })
    }
}

pub struct Rope {
    knot_coordinates: Vec<Coordinate>,
    visited: HashMap<i32, HashSet<i32>>,
}

impl Rope {
    pub fn count_visited(&self) -> usize {
        let mut result = 0;
        for bucket in self.visited.values() {
            result += bucket.len();
        }
        result
    }

    pub fn process(&mut self, instruction: &Instruction) {
        let step = instruction.direction.step();
        for _ in 0..instruction.distance {
            self.knot_coordinates[0].step(step);
            for j in 1..self.knot_coordinates.len() {
                self.move_knot(j);
            }
        }
    }

    fn move_knot(&mut self, knot_index: usize) {
        let leader = self.knot_coordinates[knot_index - 1];
        let mut follower = self.knot_coordinates[knot_index];
        if leader == follower || leader.distance(&follower) <= 1 {
            return;
        }

        follower.step_towards(&leader);
        self.knot_coordinates[knot_index] = follower;

        if knot_index == &self.knot_coordinates.len() - 1 {
            match self.visited.entry(follower.x) {
                Entry::Occupied(o) => {
                    o.into_mut().insert(follower.y);
                }
                Entry::Vacant(v) => {
                    let mut bucket = HashSet::new();
                    bucket.insert(follower.y);
                    v.insert(bucket);
                }
            }
        }
    }
}

impl From<usize> for Rope {
    fn from(num_knots: usize) -> Self {
        let knot_coordinates = vec![Coordinate::default(); num_knots];
        let mut visited = HashMap::default();
        let mut bucket = HashSet::new();
        bucket.insert(0i32);
        visited.insert(0, bucket);
        Self {
            knot_coordinates,
            visited,
        }
    }
}

pub fn get_input() -> impl Iterator<Item = Instruction> {
    get_lines("day-09.txt")
        .map(|line| line.parse::<Instruction>())
        .map(Result::unwrap)
}

#[cfg(test)]
mod tests {

    use crate::day09::{get_input, Rope};

    #[test]
    fn part1() {
        let mut rope: Rope = 2usize.into();
        get_input().for_each(|instruction| rope.process(&instruction));
        let result = rope.count_visited();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let mut rope: Rope = 10usize.into();
        get_input().for_each(|instruction| rope.process(&instruction));
        let result = rope.count_visited();

        println!("Part 2: {}", result);
    }
}
