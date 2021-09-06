// --- Day 20: Jurassic Jigsaw ---
// https://adventofcode.com/2020/day/20

use std::fmt::{Display, Formatter};

use crate::get_lines;

type Id=u64;

pub struct Tile {
    id: Id,
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
        vec![original.flip_horizontally(), original.flip_vertically(),
             original.rotate180(), original.rotate270(),
             r90.flip_horizontally(), r90.flip_vertically(), r90,
             /* these are redundant:
             original.rotate180().flip_horizontally(), original.rotate180().flip_vertically(),
             original.rotate270().flip_horizontally(), original.rotate270().flip_vertically(),
              */
             original,
        ]
    }
}

pub struct OrientedTile {
    id: Id,
    pixels: Vec<Vec<char>>, // TODO this is a lot of copying, can this just be a view into the underlying tile?
}

impl Display for OrientedTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.id, self.pixels
            .iter()
            .map(|row| row.iter()
                .cloned()
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"))
    }
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
    fn left_border(&self) -> Vec<char> { // TODO can we return Iterator instead of Vec?
        self.pixels
            .iter()
            .map(|row| row[0])
            .collect()
    }
    fn right_border(&self) -> Vec<char> {
        let last_index = self.pixels.len() - 1;
        self.pixels
            .iter()
            .map(|row| row[last_index])
            .collect()
    }
    fn top_border(&self) -> Vec<char> {
        self.pixels[0].clone()
    }
    fn bottom_border(&self) -> Vec<char> {
        let last_index = self.pixels.len() - 1;
        self.pixels[last_index].clone()
    }
    fn flip_horizontally(&self) -> Self {
        let pixels = self.pixels
            .iter()
            .map(|row| row.iter()
                .cloned()
                .rev()
                .collect())
            .collect();
        OrientedTile {
            id: self.id,
            pixels,
        }
    }

    fn flip_vertically(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: self.pixels
                .iter()
                .rev()
                .cloned()
                .collect(),
        }
    }

    fn rotate90(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len())
                .map(|original_column| (0..self.pixels.len())
                    .rev()
                    .map(|original_row| self.pixels[original_row][original_column])
                    .collect())
                .collect(),
        }
    }

    fn rotate180(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len()).rev()
                .map(|original_row| (0..self.pixels.len()).rev()
                    .map(|original_column| self.pixels[original_row][original_column])
                    .collect())
                .collect(),
        }
    }

    fn rotate270(&self) -> Self {
        OrientedTile {
            id: self.id,
            pixels: (0..self.pixels.len()).rev()
                .map(|original_column| (0..self.pixels.len())
                    .map(|original_row| self.pixels[original_row][original_column])
                    .collect())
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
}

pub fn get_input() -> Vec<Tile> { // TODO can I return an Iterator?
    let mut result = vec![];
    let mut id: Id = 0;
    let mut rows = vec![];
    for line in get_lines("day-20-input.txt") {
        if line.starts_with("Tile") {
            let mut components = line.split(' ');
            components.next(); // "Tile"
            id = components.next()
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

pub fn get_valid_arrangements(partial_arrangement: Vec<OrientedTile>,
                          remaining_tiles: Vec<Tile>,
                          edge_length: usize) -> Vec<Vec<OrientedTile>> { // TODO can I return an Iterator instead?
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

                let valid_arrangements =
                    get_valid_arrangements(partial, remaining, edge_length);
                if !valid_arrangements.is_empty() {
                    // There are more valid arrangements, but we only need one
                    return valid_arrangements;
                }
            }
        }
        panic!("No valid arrangements found");
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

            let valid_arrangements =
                get_valid_arrangements(partial, remaining, edge_length);
            prefixes.extend(valid_arrangements.iter().cloned());
        }
    }
    prefixes
}

fn tile_above(arrangement: &[OrientedTile], index: usize, edge_length: usize) -> Option<&OrientedTile> {
    if index < edge_length {
        None
    } else {
        arrangement.get(index - edge_length)
    }
}

fn tile_to_left(arrangement: &[OrientedTile], index: usize, edge_length: usize) -> Option<&OrientedTile> {
    if index % edge_length == 0 {
        None
    } else {
        arrangement.get(index - 1)
    }
}

fn fits(arrangement: &[OrientedTile], candidate: &Tile, edge_length: usize) -> Option<OrientedTile> {
    let new_index = arrangement.len();
    let tile_above = tile_above(arrangement, new_index, edge_length);
    let tile_to_left = tile_to_left(arrangement, new_index, edge_length);
    for orientation in candidate.permutations() {
        let top_fits = tile_above.is_none() || tile_above.as_ref().unwrap().fits_above(&orientation);
        let left_fits = tile_to_left.is_none() || tile_to_left.as_ref().unwrap().fits_to_left_of(&orientation);
        if top_fits && left_fits {
            return Some(orientation);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::day20::{get_input, get_valid_arrangements};

    #[test]
    fn part1() {
        let tiles = get_input();
        let edge_length = (tiles.len() as f32).sqrt() as usize;
        let possible_arrangements = get_valid_arrangements(vec![], tiles, edge_length);
        let arrangement = possible_arrangements.get(0).unwrap();
        let top_left = arrangement.get(0).unwrap();
        let top_right = arrangement.get(edge_length - 1).unwrap();
        let bottom_left = arrangement.get(arrangement.len() - edge_length).unwrap();
        let bottom_right = arrangement.get(arrangement.len() - 1).unwrap();
        let result: u64 = vec![top_left, top_right, bottom_left, bottom_right].iter()
            .map(|corner| corner.id)
            .product();
        println!("Part 1: {}", result);
    }

}