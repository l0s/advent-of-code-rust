use crate::get_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// --- Day 12: Hill Climbing Algorithm ---
/// https://adventofcode.com/2022/day/12

/// A map of the local area in a grid
pub struct HeightMap {
    grid: Vec<Vec<u8>>,
    #[allow(dead_code)]
    starting_point: (usize, usize),
    destination: (usize, usize),
}

impl HeightMap {
    pub fn length_of_shortest_path(&self, starting_point: &(usize, usize)) -> usize {
        let mut shortest_path_to_node: HashMap<(usize, usize), usize> = HashMap::new();
        shortest_path_to_node.insert(*starting_point, 0usize);
        let mut estimated_cost_to_destination: HashMap<(usize, usize), usize> = HashMap::new();
        estimated_cost_to_destination.insert(
            *starting_point,
            Self::estimate_distance(starting_point, &self.destination),
        );
        let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
        open_set.push(Node {
            coordinate: *starting_point,
            estimated_cost_to_destination: Self::estimate_distance(
                starting_point,
                &self.destination,
            ),
        });
        while let Some(current) = open_set.pop() {
            if current.coordinate == self.destination {
                return shortest_path_to_node[&current.coordinate];
            }
            for neighbour in self.neighbours(&current.coordinate) {
                let tentative_score = shortest_path_to_node[&current.coordinate] + 1;
                if tentative_score < *shortest_path_to_node.get(&neighbour).unwrap_or(&usize::MAX) {
                    shortest_path_to_node.insert(neighbour, tentative_score);
                    let estimate =
                        tentative_score + Self::estimate_distance(&neighbour, &self.destination);
                    estimated_cost_to_destination.insert(neighbour, estimate);
                    let node = Node {
                        coordinate: neighbour,
                        estimated_cost_to_destination: estimate,
                    };
                    open_set.push(node);
                }
            }
        }
        usize::MAX
    }

    pub fn potential_trail_heads(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for i in 0..self.grid.len() {
            let row = &self.grid[i];
            for (j, c) in row.iter().enumerate() {
                if *c == 0 {
                    result.push((i, j));
                }
            }
        }
        result
    }

    fn height(&self, coördinate: &(usize, usize)) -> u8 {
        self.grid[coördinate.0][coördinate.1]
    }

    fn neighbours(&self, coördinate: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        if coördinate.0 > 0 {
            let up = (coördinate.0 - 1, coördinate.1);
            if self.height(coördinate) + 1 >= self.height(&up) {
                result.push(up);
            }
        }
        if coördinate.0 < self.grid.len() - 1 {
            let down = (coördinate.0 + 1, coördinate.1);
            if self.height(coördinate) + 1 >= self.height(&down) {
                result.push(down);
            }
        }
        if coördinate.1 > 0 {
            let left = (coördinate.0, coördinate.1 - 1);
            if self.height(coördinate) + 1 >= self.height(&left) {
                result.push(left);
            }
        }
        let row = &self.grid[coördinate.0];
        if coördinate.1 < row.len() - 1 {
            let right = (coördinate.0, coördinate.1 + 1);
            if self.height(coördinate) + 1 >= self.height(&right) {
                result.push(right);
            }
        }
        result
    }

    fn estimate_distance(from: &(usize, usize), to: &(usize, usize)) -> usize {
        from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
    }
}

struct Node {
    coordinate: (usize, usize),
    estimated_cost_to_destination: usize,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate.eq(&other.coordinate)
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other
            .estimated_cost_to_destination
            .partial_cmp(&self.estimated_cost_to_destination)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_cost_to_destination
            .cmp(&self.estimated_cost_to_destination)
    }
}

pub fn get_input() -> HeightMap {
    let lines = get_lines("day-12.txt")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut grid: Vec<Vec<u8>> = vec![vec![]; lines.len()];
    let (mut start_x, mut start_y) = (0usize, 0usize);
    let (mut destination_x, mut destination_y) = (0usize, 0usize);
    for i in 0..lines.len() {
        let row = &lines[i];
        grid[i] = vec![0; row.len()];
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_x = i;
                start_y = j;
                grid[i][j] = 0;
            } else if *c == 'E' {
                destination_x = i;
                destination_y = j;
                grid[i][j] = b'z' - b'a';
            } else {
                grid[i][j] = *c as u8 - b'a';
            }
        }
    }
    HeightMap {
        grid,
        starting_point: (start_x, start_y),
        destination: (destination_x, destination_y),
    }
}

#[cfg(test)]
pub mod tests {

    use crate::day12::get_input;

    #[test]
    pub fn part1() {
        let map = get_input();
        let result = map.length_of_shortest_path(&map.starting_point);
        println!("Part 1: {}", result);
    }

    #[test]
    pub fn part2() {
        let map = get_input();
        let mut result = usize::MAX;
        for potential_trail_head in map.potential_trail_heads() {
            let distance = map.length_of_shortest_path(&potential_trail_head);
            if distance < result {
                result = distance;
            }
        }
        println!("Part 2: {}", result);
    }
}
