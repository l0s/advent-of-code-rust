use crate::get_lines;

pub fn get_input() -> Vec<u16> {
    let mut items: Vec<u16> = get_lines("/input/day-1-input.txt")
        .map(|line| line.parse::<u16>().unwrap())
        .filter(|candidate| *candidate <= 2020u16) // filter out anything that is obviously too big
        .collect();
    items.sort_unstable(); // sort to reduce the search space for the complement
    items
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    #[test]
    fn part1() {
        let items = super::get_input();

        for i in 0..items.len() - 1 {
            let x: u32 = items[i].into();
            for y in items.iter().skip(i + 1) {
                let y = *y as u32;
                let result = x + y;
                match result.cmp(&2020) {
                    Ordering::Equal => {
                        println!("Part 1: {}", (x * y));
                        return;
                    }
                    Ordering::Greater => {
                        break;
                    }
                    Ordering::Less => {}
                }
            }
        }
    }

    #[test]
    fn part2() {
        let items = super::get_input();
        for i in 0..items.len() - 2 {
            let x: u32 = items[i].into();
            for j in i + 1..items.len() - 1 {
                let y: u32 = items[j].into();
                if x + y > 2020 {
                    break;
                }
                for z in items.iter().skip(j + 1) {
                    let z = *z as u32;
                    let result = x + y + z;
                    match result.cmp(&2020u32) {
                        Ordering::Less => {}
                        Ordering::Equal => {
                            println!("Part 2: {}", (x * y * z));
                            return;
                        }
                        Ordering::Greater => {
                            break;
                        }
                    }
                }
            }
        }
    }
}
