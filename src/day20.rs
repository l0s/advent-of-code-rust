// --- Day 20: Jurassic Jigsaw ---
// https://adventofcode.com/2020/day/20

use crate::get_lines;

type Id = u64;

/// A square portion of a satellite image. Because the satellite's camera array is malfunctioning,
/// it may have been rotated or flipped randomly. Each tile's image border matches with another
/// tile.
pub struct Tile {
    id: Id,

    /// The square grid of pixels. The length of the outer vector is the same as the length of each
    /// inner vector.
    pixels: Vec<Vec<char>>,
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            id: self.id,
            pixels: self.pixels.clone(),
        }
    }
}

impl Tile {
    fn permutations(&self) -> Vec<OrientedTile> {
        let original = OrientedTile {
            id: self.id,
            pixels: self.pixels.clone(),
        };
        let r90 = original.rotate90();
        vec![
            original.flip_horizontally(),
            original.flip_vertically(),
            original.rotate180(),
            original.rotate270(),
            r90.flip_horizontally(),
            r90.flip_vertically(),
            r90,
            /* these are redundant:
            original.rotate180().flip_horizontally(), original.rotate180().flip_vertically(),
            original.rotate270().flip_horizontally(), original.rotate270().flip_vertically(),
            */
            original,
        ]
    }
}

/// A satellite image tile that has been oriented in a specific way.
pub struct OrientedTile {
    id: Id,
    pixels: Vec<Vec<char>>, // TODO this is a lot of copying, can this just be a view into the underlying tile?
}

impl Clone for OrientedTile {
    fn clone(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: self.pixels
                .iter()
                .map(|row| row.to_vec())
                .collect(),
        }
    }
}

impl OrientedTile {

    /// The reference pattern of what a Sea Monster looks like
    const SEA_MONSTER: [[char; 20]; 3] = [
        [
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ',
            '#', '#', '#',
        ],
        [
            ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#',
            ' ', ' ', ' ',
        ],
    ];

    fn left_border(&self) -> Vec<char> {
        // TODO can we return Iterator instead of Vec?
        self.pixels.iter().map(|row| row[0]).collect()
    }

    fn right_border(&self) -> Vec<char> {
        let last_index = self.pixels.len() - 1;
        self.pixels.iter().map(|row| row[last_index]).collect()
    }

    fn top_border(&self) -> Vec<char> {
        self.pixels[0].clone()
    }

    fn bottom_border(&self) -> Vec<char> {
        let last_index = self.pixels.len() - 1;
        self.pixels[last_index].clone()
    }

    fn flip_horizontally(&self) -> Self {
        let pixels = self
            .pixels
            .iter()
            .map(|row| row.iter().cloned().rev().collect())
            .collect();
        OrientedTile {
            id: self.id,
            pixels,
        }
    }

    fn flip_vertically(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: self.pixels.iter().rev().cloned().collect(),
        }
    }

    fn rotate90(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len())
                .map(|original_column| {
                    (0..self.pixels.len())
                        .rev()
                        .map(|original_row| self.pixels[original_row][original_column])
                        .collect()
                })
                .collect(),
        }
    }

    fn rotate180(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len())
                .rev()
                .map(|original_row| {
                    (0..self.pixels.len())
                        .rev()
                        .map(|original_column| self.pixels[original_row][original_column])
                        .collect()
                })
                .collect(),
        }
    }

    fn rotate270(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len())
                .rev()
                .map(|original_column| {
                    (0..self.pixels.len())
                        .map(|original_row| self.pixels[original_row][original_column])
                        .collect()
                })
                .collect(),
        }
    }

    fn fits_to_left_of(&self, right_candidate: &OrientedTile) -> bool {
        OrientedTile::edges_match(&right_candidate.left_border(), &self.right_border())
    }

    fn fits_above(&self, bottom_candidate: &OrientedTile) -> bool {
        OrientedTile::edges_match(&bottom_candidate.top_border(), &self.bottom_border())
    }

    fn edges_match(left: &[char], right: &[char]) -> bool {
        assert_eq!(left.len(), right.len(), "Edge lengths do not match");
        for i in 0..left.len() {
            if left[i] != right[i] {
                return false;
            }
        }
        true
    }

    /// Highlights sea monsters with 'O'
    ///
    /// Returns: the number of sea monsters identified
    pub fn highlight_seamonsters(&mut self) -> usize {
        // TODO should I return boolean instead?
        let window_height = OrientedTile::SEA_MONSTER.len();
        let window_width = OrientedTile::SEA_MONSTER[0].len();
        let vertical_windows = self.pixels.len() - window_height;
        let horizontal_windows = self.pixels.len() - window_width;

        let mut sum = 0usize;
        for i in 0..vertical_windows {
            for j in 0..horizontal_windows {
                if self.contains_sea_monster(i, j) {
                    sum += 1;
                    self.highlight_seamonster(i, j);
                }
            }
        }
        sum
    }

    /// Highlights a single sea monster with 'O'
    ///
    /// Parameters:
    /// - `vertical_offset` - how far "down" from the origin that the image starts
    /// - `horizontal_offset` - how far "right" from the origin that the image starts
    fn highlight_seamonster(&mut self, vertical_offset: usize, horizontal_offset: usize) {
        for i in 0..OrientedTile::SEA_MONSTER.len() {
            let pattern_row = OrientedTile::SEA_MONSTER[i];
            let image_row = &mut self.pixels[i + vertical_offset];
            for j in 0..pattern_row.len() {
                let pattern = pattern_row[j];
                if pattern == '#' {
                    image_row[j + horizontal_offset] = 'O';
                }
            }
        }
    }

    /// Determine whether or not the window whose origin is at the specified coördinates contains a
    /// sea monster.
    ///
    /// Parameters:
    /// - `vertical_offset` - the vertical origin of the window in question
    /// - `horizontal_offset` - the horizontal origin of the window in question
    ///
    /// Returns: true if and only if the window contains a sea monster
    fn contains_sea_monster(&self, vertical_offset: usize, horizontal_offset: usize) -> bool {
        for i in 0..OrientedTile::SEA_MONSTER.len() {
            let pattern_row = OrientedTile::SEA_MONSTER[i];
            let image_row = &self.pixels[i + vertical_offset];
            for j in 0..pattern_row.len() {
                let pattern = pattern_row[j];
                // spaces can be anything
                if pattern == '#' && image_row[j + horizontal_offset] != '#' {
                    // only the '#' pixels need to match
                    return false;
                }
            }
        }
        true
    }

    /// Determines how rough the waters are in the sea monsters' habitat
    ///
    /// Note the sea monsters must first be highlighted by invoking
    /// `highlight_seamonster(&mut self, usize, usize)`.
    ///
    /// Returns: the number of '#' in the image that are _not_ part of a sea monster
    pub fn roughness(&self) -> usize {
        let mut result = 0usize;
        for row in &self.pixels {
            for cell in row {
                if cell == &'#' {
                    result += 1;
                }
            }
        }
        result
    }
}

/// Retrieve all of the image tiles from the Mythical Information Bureau's satellite's camera array.
/// Due to a malfunction in the array, the tiles arrive in a random order and rotated or flipped
/// randomly.
pub fn get_input() -> Vec<Tile> {
    // TODO can I return an Iterator?
    let mut result = vec![];
    let mut id: Id = 0;
    let mut rows = vec![];
    for line in get_lines("day-20-input.txt") {
        if line.starts_with("Tile") {
            let mut components = line.split(' ');
            components.next(); // "Tile"
            id = components
                .next()
                .and_then(|string| string.strip_suffix(':'))
                .map(|string| string.parse::<Id>().unwrap())
                .expect("Invalid tile ID");
            rows = vec![];
        } else if line.is_empty() {
            result.push(Tile {
                id,
                pixels: rows.to_owned(),
            });
        } else {
            rows.push(line.chars().collect::<Vec<char>>());
        }
    }
    result
}

/// Find all the valid permutations of tile orientations that yield an image.
///
/// Parameters:
/// - `partial_arrangement` - A valid arrangement prefix
/// - `remaining_tiles` - All of the tiles not in `partial_arrangement`
/// - `edge_length` - the number of _tiles_ on each edge of the final arrangement
/// Returns: All the permutations of all the arrangements of tiles whose borders match
pub fn get_valid_arrangements(
    partial_arrangement: Vec<OrientedTile>,
    remaining_tiles: Vec<Tile>,
    edge_length: usize,
) -> Vec<Vec<OrientedTile>> {
    // TODO can I return an Iterator instead?
    if remaining_tiles.is_empty() {
        return vec![partial_arrangement];
    } else if partial_arrangement.is_empty() {
        // Find candidates for the top-left tile
        for i in 0..remaining_tiles.len() {
            let candidate = remaining_tiles.get(i).unwrap();
            for orientation in candidate.permutations() {
                let partial = vec![orientation];
                let (left, _) = remaining_tiles.split_at(i);
                let (_, right) = remaining_tiles.split_at(i + 1);
                let mut remaining = vec![];
                remaining.extend(left.iter().cloned());
                remaining.extend(right.iter().cloned());

                let valid_arrangements = get_valid_arrangements(partial, remaining, edge_length);
                if !valid_arrangements.is_empty() {
                    // There are more valid arrangements, but we only need one
                    return valid_arrangements;
                }
            }
        }
        return vec![];
    }

    // Find all possible suffixes given the partial, valid, arrangement
    let mut prefixes = vec![];
    for i in 0..remaining_tiles.len() {
        let candidate = remaining_tiles.get(i).unwrap();
        if let Some(candidate) = fits(&partial_arrangement, candidate, edge_length) {
            let mut partial = partial_arrangement.clone();
            partial.push(candidate);

            let (left, _) = remaining_tiles.split_at(i);
            let (_, right) = remaining_tiles.split_at(i + 1);
            let mut remaining = vec![];
            remaining.extend(left.iter().cloned());
            remaining.extend(right.iter().cloned());

            let valid_arrangements = get_valid_arrangements(partial, remaining, edge_length);
            prefixes.extend(valid_arrangements.iter().cloned());
        }
    }
    prefixes
}

fn tile_above(
    arrangement: &[OrientedTile],
    index: usize,
    edge_length: usize,
) -> Option<&OrientedTile> {
    if index < edge_length {
        None
    } else {
        arrangement.get(index - edge_length)
    }
}

fn tile_to_left(
    arrangement: &[OrientedTile],
    index: usize,
    edge_length: usize,
) -> Option<&OrientedTile> {
    if index % edge_length == 0 {
        None
    } else {
        arrangement.get(index - 1)
    }
}

fn fits(
    arrangement: &[OrientedTile],
    candidate: &Tile,
    edge_length: usize,
) -> Option<OrientedTile> {
    let new_index = arrangement.len();
    let tile_above = tile_above(arrangement, new_index, edge_length);
    let tile_to_left = tile_to_left(arrangement, new_index, edge_length);
    for orientation in candidate.permutations() {
        let top_fits =
            tile_above.is_none() || tile_above.as_ref().unwrap().fits_above(&orientation);
        let left_fits =
            tile_to_left.is_none() || tile_to_left.as_ref().unwrap().fits_to_left_of(&orientation);
        if top_fits && left_fits {
            return Some(orientation);
        }
    }
    None
}

/// Combine a specific arrangement of tiles into one big tile
///
/// Parameters
/// - `arrangement` - a square arrangement of tiles. It's length must equal `tiles_on_edge^2`
/// - `tiles_on_edge` - the number of tiles on each border of the image
/// - `edge_size` - the total number of pixels in the final image
pub fn combine(arrangement: &[Tile], tiles_on_edge: usize, edge_size: usize) -> Tile {
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(edge_size);
    for (index, tile) in arrangement.iter().enumerate() {
        let tile_row = index / tiles_on_edge;
        let tile_column = index % tiles_on_edge;
        let row_offset = tile.pixels.len() * tile_row;
        let column_offset = tile.pixels.len() * tile_column;
        for original_row in 0..tile.pixels.len() {
            let row_id = original_row + row_offset;
            for (original_column, pixel) in tile.pixels[original_row].iter().enumerate() {
                let column_id = original_column + column_offset;
                if column_id == 0 {
                    grid.push(Vec::with_capacity(edge_size));
                }
                let row = &mut grid[row_id];
                row.push(*pixel);
            }
        }
    }
    Tile {
        id: 0,
        pixels: grid,
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::{combine, get_input, get_valid_arrangements, Tile};

    // TODO extract common code to pull arrangement once

    #[test]
    fn part1() {
        let tiles = get_input();
        let edge_length = (tiles.len() as f32).sqrt() as usize;
        let possible_arrangements = get_valid_arrangements(vec![], tiles, edge_length);
        assert!(!possible_arrangements.is_empty());
        let arrangement = possible_arrangements.get(0).unwrap();
        let top_left = arrangement.get(0).unwrap();
        let top_right = arrangement.get(edge_length - 1).unwrap();
        let bottom_left = arrangement.get(arrangement.len() - edge_length).unwrap();
        let bottom_right = arrangement.get(arrangement.len() - 1).unwrap();
        let result: u64 = vec![top_left, top_right, bottom_left, bottom_right]
            .iter()
            .map(|corner| corner.id)
            .product();
        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let tiles = get_input();
        let tiles_on_edge = (tiles.len() as f32).sqrt() as usize;
        let edge_size = tiles.get(0).unwrap().pixels.len();
        let possible_arrangements = get_valid_arrangements(vec![], tiles, tiles_on_edge);
        assert!(!possible_arrangements.is_empty());
        let arrangement = possible_arrangements.get(0).unwrap();

        let cropped_tiles = arrangement
            .iter()
            .map(|tile| -> Tile {
                // TODO add Tile method to remove border
                let mut wide_rows = tile.pixels.clone();
                wide_rows.remove(wide_rows.len() - 1);
                wide_rows.remove(0);

                Tile {
                    id: tile.id,
                    pixels: wide_rows
                        .iter()
                        .map(|wide_row| -> Vec<char> {
                            let mut row = wide_row.clone();
                            row.remove(row.len() - 1);
                            row.remove(0);
                            row
                        })
                        .collect(),
                }
            })
            .collect::<Vec<Tile>>();
        let combined_tile = combine(&cropped_tiles, tiles_on_edge, edge_size);
        for mut permutation in combined_tile.permutations() {
            let num_sea_monsters = permutation.highlight_seamonsters();
            if num_sea_monsters > 0 {
                println!("Part 2: {}", permutation.roughness());
                break;
            }
        }
    }
}
