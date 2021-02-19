use crate::get_lines;

fn get_input() -> impl Iterator<Item = String> {
    get_lines("/input/day-3-input.txt")
}

/// A map of the trees in the area
pub fn get_map() -> Vec<Vec<bool>> {
    get_input()
        .map(|line| {
            line.chars()
                .map(|square| square == '#')
                .collect::<Vec<bool>>()
        })
        .collect()
}

/// Returns the number of trees a toboggan will encounter when traveling from the top-left of a map
/// to the bottom.
///
/// Parameters
/// * `forest` - a map of the area with `true` squares denoting the presence of a tree
/// * `slope` - the angle at which the toboggan travels
pub fn count_trees(forest: &[Vec<bool>], slope: &Slope) -> u8 {
    let mut num_trees: u8 = 0;
    let mut row_index: usize = 0;
    let mut column_index: usize = 0;

    loop {
        let row = &forest[row_index];
        let cell = &row[column_index];
        if *cell {
            num_trees += 1;
        }
        // waiting for destructuring assignment: https://github.com/rust-lang/rfcs/issues/372
        let (new_row, new_column) = slope.move_toboggan(row_index, column_index, row.len());
        row_index = new_row;
        column_index = new_column;

        if row_index >= forest.len() {
            break num_trees;
        }
    }
}

pub struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    /// Returns the new coordinates of the toboggan on the map
    pub fn move_toboggan(
        &self,
        current_row: usize,
        current_column: usize,
        num_columns: usize,
    ) -> (usize, usize) {
        let row = current_row + self.down;
        let column = current_column + self.right;
        // These aren't the only trees, though; due to something you read about once involving
        // arboreal genetics and biome stability, the same pattern repeats to the right many times
        let column = column % num_columns;
        (row, column)
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::{count_trees, get_map, Slope};

    #[test]
    fn part1() {
        let slope = Slope { right: 3, down: 1 };

        let num_trees = count_trees(&get_map(), &slope);

        println!("Part 1: {}", num_trees);
    }

    #[test]
    fn part2() {
        let slopes = vec![
            Slope { right: 1, down: 1 },
            Slope { right: 3, down: 1 },
            Slope { right: 5, down: 1 },
            Slope { right: 7, down: 1 },
            Slope { right: 1, down: 2 },
        ];
        let forest = get_map();

        let num_trees = slopes
            .iter()
            .map(|slope| count_trees(&forest, slope))
            .fold(1u64, |accumulator, count| accumulator * (count as u64));

        println!("Part 2: {}", num_trees);
    }
}
