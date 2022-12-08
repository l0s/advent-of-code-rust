/// --- Day 8: Treetop Tree House ---
/// https://adventofcode.com/2022/day/8
use crate::get_lines;

/// A "peculiar patch of tall trees all planted carefully in a grid" as part of a reforestation
/// effort.
pub struct Forest {
    /// The heights of the trees
    heights: Vec<Vec<u8>>,
}

impl Forest {
    /// Determine how many trees in the forest are visible from outside the forest (from any side)
    pub fn count_visible_trees(&self) -> usize {
        let mut result = 0;
        for i in 0..self.heights.len() {
            for j in 0..self.heights[i].len() {
                if self.is_visible(i, j) {
                    result += 1;
                }
            }
        }
        result
    }

    /// Determine if a tree is visible from outside the forest, that is, not obstructed by other
    /// trees from any side
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.heights[x][y];
        let mut visible_from_north = true;
        for i in 0..x {
            if self.heights[i][y] >= tree_height {
                visible_from_north = false;
                break;
            }
        }
        if visible_from_north {
            return true;
        }
        let mut visible_from_south = true;
        for i in x + 1..self.heights.len() {
            if self.heights[i][y] >= tree_height {
                visible_from_south = false;
                break;
            }
        }
        if visible_from_south {
            return true;
        }
        let mut visible_from_west = true;
        for j in 0..y {
            if self.heights[x][j] >= tree_height {
                visible_from_west = false;
                break;
            }
        }
        if visible_from_west {
            return true;
        }
        for j in y + 1..self.heights[x].len() {
            if self.heights[x][j] >= tree_height {
                return false;
            }
        }
        true
    }

    /// Determine the best possible view from any treetop in the forest
    pub fn max_scenic_score(&self) -> usize {
        let mut result = 0;
        for i in 0..self.heights.len() {
            for j in 0..self.heights[i].len() {
                let score = self.scenic_score(i, j);
                if score > result {
                    result = score;
                }
            }
        }
        result
    }

    /// A score that rewards coördinates from which the most trees are visible
    ///
    /// Parameters:
    /// - `(x, y)` - the potential position in the forest to build a treetop tree house
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.heights[x][y];
        let mut north_score = 0;
        for i in (0..x).rev() {
            let height = self.heights[i][y];
            north_score += 1;
            if height >= tree_height {
                break;
            }
        }
        let mut south_score = 0;
        for i in x + 1..self.heights.len() {
            let height = self.heights[i][y];
            south_score += 1;
            if height >= tree_height {
                break;
            }
        }
        let mut west_score = 0;
        for j in (0..y).rev() {
            let height = self.heights[x][j];
            west_score += 1;
            if height >= tree_height {
                break;
            }
        }
        let mut east_score = 0;
        for j in y + 1..self.heights[x].len() {
            let height = self.heights[x][j];
            east_score += 1;
            if height >= tree_height {
                break;
            }
        }
        north_score * east_score * south_score * west_score
    }
}

pub fn get_input() -> Forest {
    let heights = get_lines("day-08.txt")
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).expect("Invalid tree height") as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Forest { heights }
}

#[cfg(test)]
mod tests {

    use crate::day08::get_input;

    #[test]
    fn part1() {
        let result = get_input().count_visible_trees();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let result = get_input().max_scenic_score();

        println!("Part 2: {}", result);
    }
}
