use advent_of_code::get_lines;

// https://adventofcode.com/2020/day/9

/// "Though the port is non-standard, you manage to connect it to your computer through the clever
/// use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle
/// input).
///
/// The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which,
/// conveniently for you, is an old cypher with an important weakness."
fn get_data() -> impl Iterator<Item=u64> {
    get_lines("/input/day-9-input.txt")
        .map(|line| line.parse())
        .map(|result| result.unwrap()) // panic on invalid XMAS code
}

mod tests {
    use std::collections::LinkedList;

    use crate::day09::get_data;

    #[test]
    fn test() {
        let preamble_size = 25_usize;
        let mut buffer: LinkedList<u64> = LinkedList::new();
        let mut data = get_data();
        // "XMAS starts by transmitting a preamble of 25 numbers."
        for _ in 0..preamble_size {
            buffer.push_back(data.next().unwrap());
        }
        let invalid = 'candidates: loop {
            match data.next() {
                None => break None,
                Some(number) => {
                    // "each number you receive should be the sum of any two of the 25 immediately
                    // previous numbers"
                    let mut addends = buffer.iter()
                        // no negative numbers in the input, so filter out anything larger than our
                        // target
                        .filter(|previous| **previous <= number)
                        .collect::<Vec<&u64>>();
                    addends.sort(); // sort so we can quit search early
                    for i in 0..addends.len() - 1 {
                        let x = addends[i];
                        for j in i + 1..addends.len() {
                            let y = addends[j];
                            let sum = x + y;
                            if sum == number {
                                buffer.push_back(number);
                                buffer.pop_front();
                                continue 'candidates;
                            } else if sum > number {
                                continue;
                            }
                        }
                    }
                    break Some(number);
                }
            }
        };
        let invalid = invalid.unwrap();
        println!("Part 1: {}", invalid);

        // I avoided putting all the input into memory until this point, but for the next part, it
        // seems to be the most elegant approach. Alternatively, I could use two iterators over the
        // input file, one to iterate through the potential left bounds and one to find the right
        // bound for each potential left bound.
        let data = get_data().collect::<Vec<u64>>();
        'left_bounds: for i in 0..data.len() - 1 {
            let mut total = data[i];
            let mut min = total;
            let mut max = total;
            for j in i + 1..data.len() {
                let current = data[j];
                if current < min { min = current }
                if current > max { max = current }
                total += current;
                if total == invalid {
                    let result = min + max;
                    println!("Part 2: {}", result);
                    break 'left_bounds;
                } else if total > invalid {
                    // Since there are no negative numbers, stop the search when the numbers become
                    // too big.
                    continue 'left_bounds;
                }
            }
        }
    }
}