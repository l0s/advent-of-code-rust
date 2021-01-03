use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_lines(file: &str) -> impl Iterator<Item=u16> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = String::from(manifest_dir) + file;
    let path = Path::new(&file_name);
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|result| result.unwrap().parse::<u16>().unwrap())
        .filter(|candidate| *candidate <= 2020u16) // filter out anything that is obviously too big
}

fn get_input() -> Vec<u16> {
    let mut items: Vec<u16> = get_lines("/input/day-1-input.txt").collect();
    items.sort(); // sort to reduce the search space for the complement
    items
}

mod tests {
    #[test]
    fn part1() {
        let items = super::get_input();

        for i in 0..items.len() - 1 {
            let x: u32 = items[i].into();
            for j in i + 1..items.len() {
                let y: u32 = items[j].into();
                let result = x + y;
                if result == 2020 {
                    println!("Part 1: {}", (x * y));
                    return;
                } else if result > 2020 {
                    break;
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
                for k in j + 1..items.len() - 0 {
                    let z: u32 = items[k].into();
                    let result = x + y + z;
                    if result == 2020 {
                        println!("Part 2: {}", (x * y * z));
                    } else if result > 2020 {
                        break;
                    }
                }
            }
        }
    }
}