use crate::get_lines;

fn get_input() -> impl Iterator<Item=String> {
    get_lines("/input/day-5-input.txt")
}

fn get_sorted_seat_ids() -> Vec<u16> {
    let mut result = get_input()
        .map(|id| -> u16 {
            let mut chars = id.chars();

            let mut max_row_exclusive = 128u8;
            let mut row = 0;
            for i in 0..7_usize {
                let partition = chars.next().expect(&("Invalid id: ".to_owned() + &*id + " at index " + &i.to_string()));
                if partition == 'B' {
                    row = ((max_row_exclusive - row) / 2u8) + row;
                } else if partition == 'F' {
                    max_row_exclusive = max_row_exclusive - ((max_row_exclusive - row) / 2);
                } else {
                    panic!("Invalid character '{}' at index {} for id {}.", partition, i, id);
                }
            }

            let mut column = 0u8;
            let mut max_column_exclusive = 8u8;
            for i in 7..10_usize {
                let partition = chars.next().expect(&("Invalid id: ".to_owned() + &*id + " at index " + &i.to_string()));
                if partition == 'R' {
                    column = ((max_column_exclusive - column) / 2u8) + column;
                } else if partition == 'L' {
                    max_column_exclusive = max_column_exclusive - ((max_column_exclusive - column) / 2);
                } else {
                    panic!("Invalid character '{}' at index {} for id {}.", partition, i, id);
                }
            }

            row as u16 * 8u16 + column as u16
        }).collect::<Vec<u16>>();
    result.sort();
    result
}

mod tests {
    use crate::day05::get_sorted_seat_ids;

    #[test]
    fn part1() {
        let seat_ids = get_sorted_seat_ids();
        let last_seat_id = seat_ids.get(seat_ids.len() - 1).unwrap();
        println!("Part 1: {}", last_seat_id);
    }

    #[test]
    fn part2() {
        let seat_ids = get_sorted_seat_ids();
        for i in 1..seat_ids.len() {
            let x = seat_ids.get(i - 1).unwrap();
            let y = seat_ids.get(i).unwrap();
            if y - x > 1 {
                println!("Part 2: {}", (x + 1));
                return;
            }
        }
    }
}