// --- Day 20: Jurassic Jigsaw ---
// https://adventofcode.com/2020/day/20

use std::ops::{Index, IndexMut};

use Transformation::*;

use crate::get_lines;
use std::fmt::{Display, Formatter};

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

impl Index<usize> for Tile {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl IndexMut<usize> for Tile {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            id: self.id,
            pixels: self.pixels.clone(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let picture = self.pixels.iter().map(|row| -> String {
            let mut joined: String = row.iter().cloned().collect::<String>();
            joined.push('\n');
            joined
        }).collect::<String>();
        write!(f, "{}:\n{}\n", self.id, picture)
    }
}

impl Tile {
    fn permutations(&self) -> Vec<OrientedTile> {
        let original = OrientedTile {
            id: self.id,
            pixels: self.pixels.clone(),
            tile: self,
            transformations: vec![],
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

    /// Determines how rough the waters are in the sea monsters' habitat
    ///
    /// Returns: the number of '#' pixels in the image
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

    pub fn crop_borders(&self) -> Self {
        let mut wide_rows = self.pixels.clone();
        wide_rows.remove(wide_rows.len() - 1);
        wide_rows.remove(0);

        Tile {
            id: self.id,
            pixels: wide_rows
                .iter()
                .cloned()
                .map(|mut row| -> Vec<char> {
                    // let mut row = wide_row.clone();
                    row.remove(row.len() - 1);
                    row.remove(0);
                    row
                })
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Transformation {
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontally,
    FlipVertically,
}

impl Transformation {
    fn transform(&self, x: usize, y: usize, length: usize) -> (usize, usize) {
        let result = match self {
            Rotate90 => (y, length - x - 1),
            Rotate180 => (length - x - 1, length - y - 1),
            Rotate270 => (length - y - 1, length - x - 1),
            FlipHorizontally => (x, length - y - 1),
            FlipVertically => (length - x - 1, y),
        };
        // eprintln!("{:?}: ({}, {}) -> ({}, {})", self, x, y, result.0, result.1);
        result
    }
}

/// A satellite image tile that has been oriented in a specific way.
pub struct OrientedTile<'t> {
    #[deprecated]
    id: Id, // FIXME pass through to `tile`
    #[deprecated]
    pixels: Vec<Vec<char>>,
    // TODO this is a lot of copying, can this just be a view into the underlying tile?
    tile: &'t Tile,
    transformations: Vec<Transformation>,
}

impl<'t> Clone for OrientedTile<'t> {
    fn clone(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: self.pixels
                .iter()
                .map(|row| row.to_vec())
                .collect(),
            tile: self.tile,
            transformations: self.transformations.clone(),
        }
    }
}

impl<'t> Display for OrientedTile<'t> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let picture = self.pixels().iter().map(|row| -> String {
            let mut joined: String = row.iter().cloned().collect::<String>();
            joined.push('\n');
            joined
        }).collect::<String>();
        write!(f, "{}:\n{}\n", self.tile.id, picture)
    }
}

impl<'t> OrientedTile<'t> {
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

    pub fn tile(&self) -> Tile {
        Tile {
            id: self.tile.id,
            pixels: self.pixels(),
        }
    }

    pub fn pixels(&self) -> Vec<Vec<char>> {
        (0..self.edge_length()) // final row indices
            .map(|i| -> Vec<char> {
                (0..self.edge_length()) // final column indices
                    .map(|j| self.translate(i, j)) // original coördinates
                    .map(|(x, y)| self.tile[x][y])// original char
                    .collect()
            })
            .collect()
    }

    fn edge_length(&self) -> usize {
        self.tile.pixels.len()
    }

    fn translate(&self, x: usize, y: usize) -> (usize, usize) {
        self.transformations
            .iter()
            .fold((x, y),
                  |previous, transformation| transformation
                      .transform(previous.0, previous.1, self.edge_length()))
    }

    fn item_at(&self, x: usize, y: usize) -> char {
        let coordinates = self.translate(x, y);
        self.tile[coordinates.0][coordinates.1]
    }

    fn left_border(&'t self) -> impl Iterator<Item=char> + 't {
        (0..self.edge_length())
            .map(move |i| self.item_at(i, 0))
    }

    fn right_border(&'t self) -> impl Iterator<Item=char> + 't {
        let last_index = self.edge_length() - 1;
        (0..self.edge_length())
            .map(move |i| self.item_at(i, last_index))
    }

    fn top_border(&'t self) -> impl Iterator<Item=char> + 't {
        (0..self.edge_length())
            .map(move |j| self.item_at(0, j))
    }

    fn bottom_border(&'t self) -> impl Iterator<Item=char> + 't {
        let last_index = self.edge_length() - 1;
        // self.pixels[last_index].iter().copied()
        // FIXME this yields different results than the line above T_T
        (0..self.edge_length())
            .map(move |j| self.item_at(last_index, j))

        // (0..self.edge_length())
        //     .map(move |j| self.transformations.iter()
        //         .fold((last_index, j),
        //               |accumulator, transformation| transformation
        //                   .transform(accumulator.0, accumulator.1, self.edge_length())))
        //     .map(move |coordinates| self.tile[coordinates.0][coordinates.1])
    }

    fn flip_horizontally(&self) -> Self {
        let pixels = self
            .pixels
            .iter()
            .map(|row| row.iter().cloned().rev().collect())
            .collect();
        let mut transformations = self.transformations.clone();
        transformations.push(Transformation::FlipHorizontally);
        OrientedTile {
            id: self.id,
            pixels,
            tile: self.tile,
            transformations,
        }
    }

    fn flip_vertically(&self) -> Self {
        let mut transformations = self.transformations.clone();
        transformations.push(Transformation::FlipVertically);
        OrientedTile {
            id: self.id,
            pixels: self.pixels.iter().rev().cloned().collect(),
            tile: self.tile,
            transformations,
        }
    }

    fn rotate90(&self) -> Self {
        let mut transformations = self.transformations.clone();
        transformations.push(Transformation::Rotate90);
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
            tile: self.tile,
            transformations,
        }
    }

    fn rotate180(&self) -> Self {
        let mut transformations = self.transformations.clone();
        transformations.push(Transformation::Rotate180);
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
            tile: self.tile,
            transformations,
        }
    }

    fn rotate270(&self) -> Self {
        let mut transformations = self.transformations.clone();
        transformations.push(Transformation::Rotate270);
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
            tile: self.tile,
            transformations,
        }
    }

    fn fits_to_left_of(&self, right_candidate: &OrientedTile) -> bool {
        OrientedTile::edges_match(right_candidate.left_border(), self.right_border())
    }

    fn fits_above(&self, bottom_candidate: &OrientedTile) -> bool {
        OrientedTile::edges_match(bottom_candidate.top_border(), self.bottom_border())
    }

    fn edges_match(mut x: impl Iterator<Item=char>, mut y: impl Iterator<Item=char>) -> bool {
        while let (Some(x), Some(y)) = (x.next(), y.next()) {
            if x != y {
                return false;
            }
        }
        x.next().is_none() && y.next().is_none()
    }

    /// Highlights sea monsters with 'O'
    ///
    /// Returns: the number of sea monsters identified and a copy of the tile with the sea monsters
    /// highlighted
    pub fn highlight_seamonsters(&'t self) -> (usize, Tile) {
        // TODO should I return boolean instead?
        let window_height = OrientedTile::SEA_MONSTER.len();
        let window_width = OrientedTile::SEA_MONSTER[0].len();
        let vertical_windows = self.edge_length() - window_height;
        let horizontal_windows = self.edge_length() - window_width;

        let mut pixels = self.pixels();

        let mut sum = 0usize;
        for i in 0..vertical_windows {
            for j in 0..horizontal_windows {
                if self.contains_sea_monster(&pixels, i, j) {
                    sum += 1;
                    self.highlight_seamonster(&mut pixels,i, j);
                }
            }
        }
        let tile = Tile {
            id: self.tile.id,
            pixels,
        };
        (sum, tile)
    }

    /// Paints a sea monster using '0' in the given window, overwriting any existing pixels
    ///
    /// Parameters:
    /// - `vertical_offset` - how far "down" from the origin that the image starts
    /// - `horizontal_offset` - how far "right" from the origin that the image starts
    fn highlight_seamonster(&'t self, pixels: &mut Vec<Vec<char>>, vertical_offset: usize, horizontal_offset: usize) {
        for i in 0..OrientedTile::SEA_MONSTER.len() {
            let pattern_row = OrientedTile::SEA_MONSTER[i];
            for j in 0..pattern_row.len() {
                let pattern = pattern_row[j];
                let image_row = &mut pixels[ i + vertical_offset ];
                if pattern == '#' {
                    image_row[ j + horizontal_offset ] = '0';
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
    fn contains_sea_monster(&'t self, pixels: &Vec<Vec<char>>, vertical_offset: usize, horizontal_offset: usize) -> bool {
        for i in 0..OrientedTile::SEA_MONSTER.len() {
            let pattern_row = OrientedTile::SEA_MONSTER[i];
            let image_row = &pixels[ i + vertical_offset ];
            for j in 0..pattern_row.len() {
                let pattern = pattern_row[j];
                // spaces can be anything
                if pattern == '#' && image_row[ j + horizontal_offset ] != '#' {
                    // only the '#' pixels need to match
                    return false;
                }
            }
        }
        true
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
pub fn get_valid_arrangements<'t, 'a>(
    partial_arrangement: &'a [OrientedTile<'t>],
    remaining_tiles: Vec<&'t Tile>,
    edge_length: usize,
) -> Vec<Vec<OrientedTile<'t>>> {
    // TODO can I return an Iterator instead?
    if remaining_tiles.is_empty() {
        return vec![partial_arrangement.to_vec()];
    } else if partial_arrangement.is_empty() {
        // Find candidates for the top-left tile
        for i in 0..remaining_tiles.len() {
            let candidate = remaining_tiles[i]; // choose a candidate for the top left corner
            for orientation in candidate.permutations() { // choose an orientation for the candidate tile
                let partial = vec![orientation];
                let (left, _) = remaining_tiles.split_at(i);
                let (_, right) = remaining_tiles.split_at(i + 1);
                let mut remaining = vec![];
                remaining.extend(left.iter().cloned());
                remaining.extend(right.iter().cloned());

                // get all the possible arrangements with this orientation as the first tile
                let valid_arrangements =
                    get_valid_arrangements(&partial, remaining, edge_length);
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
        // choose a candidate for the next tile
        let candidate = remaining_tiles[i];
        // check if it fits in some orientation
        let mut partial = vec![];
        partial.extend_from_slice(partial_arrangement);
        if let Some(candidate) = fits(&partial, candidate, edge_length) {
            let mut partial = partial_arrangement.to_vec();
            partial.push(candidate);

            let (left, _) = remaining_tiles.split_at(i);
            let (_, right) = remaining_tiles.split_at(i + 1);
            let mut remaining = vec![];
            remaining.extend(left.iter().cloned());
            remaining.extend(right.iter().cloned());

            let valid_arrangements = get_valid_arrangements(&partial, remaining, edge_length);
            prefixes.extend(valid_arrangements.iter().cloned());
        }
    }
    prefixes
}

fn tile_above<'a>( // TODO move this into an "arrangement" struct
    arrangement: &'a [OrientedTile],
    index: usize,
    edge_length: usize,
) -> Option<&'a OrientedTile<'a>> {
    if index < edge_length {
        None
    } else {
        arrangement.get(index - edge_length)
    }
}

fn tile_to_left<'a>( // TODO move this into an "arrangement" struct
    arrangement: &'a [OrientedTile],
    index: usize,
    edge_length: usize,
) -> Option<&'a OrientedTile<'a>> {
    if index % edge_length == 0 {
        None
    } else {
        arrangement.get(index - 1)
    }
}

fn fits<'a, 't>(
    arrangement: &'a [OrientedTile],
    candidate: &'t Tile,
    edge_length: usize,
) -> Option<OrientedTile<'t>> {
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
    fn crop() {
        // given
        let original = Tile {
            id: 0,
            pixels: vec![
                vec![ 'X', 'X', 'X', 'X' ],
                vec![ 'X', 'O', 'O', 'X' ],
                vec![ 'X', 'O', 'O', 'X' ],
                vec![ 'X', 'X', 'X', 'X' ],
            ]
        };

        // when
        let cropped = original.crop_borders();

        // then
        let mut iter = cropped.pixels.iter();
        assert_eq!(iter.next(), Some(&vec!['O', 'O']));
        assert_eq!(iter.next(), Some(&vec!['O', 'O']));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn part1() {
        let tiles = get_input();
        let refs = tiles.iter().map(|tile| tile).collect();
        let edge_length = (tiles.len() as f32).sqrt() as usize;
        let empty = vec![];
        let possible_arrangements = get_valid_arrangements(&empty, refs, edge_length);
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
        let refs = tiles.iter().map(|tile| tile).collect();
        let tiles_on_edge = (tiles.len() as f32).sqrt() as usize;
        let edge_size = tiles.get(0).unwrap().pixels.len();
        let empty = vec![];
        let possible_arrangements = get_valid_arrangements(&empty, refs, tiles_on_edge);
        assert!(!possible_arrangements.is_empty());
        let arrangement = &possible_arrangements[0];

        let cropped_tiles = arrangement
            .iter()
            .map(|oriented| oriented.tile())
            .map(|tile| tile.crop_borders())
            .collect::<Vec<Tile>>();
        let combined_tile = combine(&cropped_tiles, tiles_on_edge, edge_size);
        eprintln!("-- combined tile:\n{}", combined_tile);
        for mut permutation in combined_tile.permutations() {
            // eprintln!("-- looking for sea monster in:\n{}", permutation);
            let (num_sea_monsters, highlighted) = permutation.highlight_seamonsters();
            eprintln!("-- found {} sea monsters.", num_sea_monsters);
            eprintln!("-- highlighted:\n{}", highlighted);
            if num_sea_monsters > 0 {
                println!("Part 2: {}", highlighted.roughness());
                return;
            }
        }
        assert!(false, "None of the permutations had sea monsters");
    }
}
