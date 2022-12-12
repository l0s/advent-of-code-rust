use crate::get_block_strings;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use Operator::{Add, Multiply};
use ValueSupplier::{Literal, OldValue};

/// --- Day 11: Monkey in the Middle ---
/// https://adventofcode.com/2022/day/11

pub type Int = u64;

/// A description of how a specific monkey handles your belongings in response to your worry level.
#[derive(Clone)]
pub struct Monkey {
    /// Your worry level for each item the monkey has
    items: Vec<Int>,
    /// How your worry level changes when this monkey inspects each item
    operation: Operation,
    /// A factor in what the monkey chooses to do with each item
    divisor: Int,
    /// The recipient ID if the test passes
    target_if_true: usize,
    /// The recipient ID if the test fails
    target_if_false: usize,
    /// The total number of times this monkey has inspected any item
    items_inspected: usize,
}

impl Monkey {
    /// Inspect the items in possession and throw them to other monkeys as necessary.
    ///
    /// Note this method assumes that a monkey cannot throw an item to themself.
    pub fn inspect_items(&mut self, worry_updater: &dyn Fn(&Int) -> Int) -> Vec<Throw> {
        let result = self
            .items
            .iter()
            .map(|worry_level| -> Throw {
                let worry_level = self.operation.apply(worry_level);
                let worry_level = worry_updater(&worry_level);
                let destination_monkey_id = if worry_level % self.divisor == 0 {
                    self.target_if_true
                } else {
                    self.target_if_false
                };
                Throw {
                    destination_monkey_id,
                    worry_level,
                }
            })
            .collect::<Vec<Throw>>();
        self.items_inspected += result.len();
        self.items.clear();
        result
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(block: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NOT_A_NUMBER: Regex = Regex::new("[^0-9]").unwrap();
        }
        let mut lines = block.split('\n').skip(1);
        let items = lines
            .next()
            .expect("Starting items not specified")
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|item_string| item_string.parse::<Int>())
            .map(Result::unwrap)
            .collect::<Vec<Int>>();
        let operation = lines
            .next()
            .expect("Operation not specified")
            .parse::<Operation>()
            .expect("Unparseable operation");
        let divisor = NOT_A_NUMBER
            .replace_all(lines.next().expect("Divisor not specified"), "")
            .parse::<Int>()
            .expect("Unparseable divisor");
        let target_if_true = NOT_A_NUMBER
            .replace_all(lines.next().expect("Target if true not specified"), "")
            .parse::<usize>()
            .expect("Unparseable target if true");
        let target_if_false = NOT_A_NUMBER
            .replace_all(lines.next().expect("Target if false not specified"), "")
            .parse::<usize>()
            .expect("Unparseable target if true");
        Ok(Self {
            items,
            operation,
            divisor,
            target_if_true,
            target_if_false,
            items_inspected: 0,
        })
    }
}

/// The side effect of a monkey inspecting an item, a throw to another monkey.
pub struct Throw {
    #[allow(dead_code)]
    destination_monkey_id: usize,
    #[allow(dead_code)]
    worry_level: Int,
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn apply(&self, x: &Int, y: &Int) -> Int {
        match self {
            Add => x + y,
            Multiply => x * y,
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Multiply),
            "+" => Ok(Add),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
enum ValueSupplier {
    OldValue,
    Literal(Int),
}

impl ValueSupplier {
    fn get_value(&self, old_value: &Int) -> Int {
        match self {
            OldValue => *old_value,
            Literal(literal) => *literal,
        }
    }
}

#[derive(Copy, Clone)]
struct Operation {
    operator: Operator,
    x_value_supplier: ValueSupplier,
    y_value_supplier: ValueSupplier,
}

impl Operation {
    pub fn apply(&self, old_value: &Int) -> Int {
        let x = self.x_value_supplier.get_value(old_value);
        let y = self.y_value_supplier.get_value(old_value);
        self.operator.apply(&x, &y)
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let trimmed = line.trim();
        assert!(trimmed.starts_with("Operation:"));
        let mut components = trimmed.split(' ').skip(3);
        let x_value_supplier = match components.next().expect("First parameter is missing") {
            "old" => OldValue,
            numeric => {
                let literal = numeric.parse::<Int>().expect("Unparseable parameter");
                Literal(literal)
            }
        };
        let operator = components
            .next()
            .expect("Operator is missing")
            .parse::<Operator>()
            .expect("Unparseable operator");
        let y_value_supplier = match components.next().expect("Second parameter is missing") {
            "old" => OldValue,
            numeric => {
                let literal = numeric.parse::<Int>().expect("Unparseable parameter");
                Literal(literal)
            }
        };
        Ok(Self {
            operator,
            x_value_supplier,
            y_value_supplier,
        })
    }
}

/// Parse the descriptions of the monkeys' behaviour. The Monkey's ID is its index in the vector.
pub fn get_input() -> Vec<Monkey> {
    get_block_strings("day-11.txt")
        .map(|block| block.parse::<Monkey>())
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
pub mod tests {

    use crate::day11::{get_input, Int};

    #[test]
    pub fn part1() {
        let mut monkeys = get_input();
        fn update_worry_level(worry_level: &Int) -> Int {
            worry_level / 3
        }
        for _round in 0..20 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys[i].clone();
                let throws = monkey.inspect_items(&update_worry_level);
                monkeys[i] = monkey;
                for throw in throws {
                    let mut target = monkeys[throw.destination_monkey_id].clone();
                    target.items.push(throw.worry_level);
                    monkeys[throw.destination_monkey_id] = target;
                }
            }
        }
        let mut counts = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .collect::<Vec<usize>>();
        counts.sort_unstable_by(|x, y| y.cmp(x));
        let result = counts[0] * counts[1];
        println!("Part 1: {}", result);
    }

    #[test]
    pub fn part2() {
        let mut monkeys = get_input();
        let product_of_divisors = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .reduce(|x, y| x * y)
            .expect("No monkeys found");
        let update_worry_level =
            move |worry_level: &Int| -> Int { worry_level % product_of_divisors };
        for _round in 0..10_000 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys[i].clone();
                let throws = monkey.inspect_items(&update_worry_level);
                monkeys[i] = monkey;
                for throw in throws {
                    let mut target = monkeys[throw.destination_monkey_id].clone();
                    target.items.push(throw.worry_level);
                    monkeys[throw.destination_monkey_id] = target;
                }
            }
        }
        let mut counts = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .collect::<Vec<usize>>();
        counts.sort_unstable_by(|x, y| y.cmp(x));
        let result = counts[0] * counts[1];
        println!("Part 2: {}", result);
    }
}
