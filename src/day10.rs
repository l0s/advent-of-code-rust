use crate::day10::Instruction::{AddX, NoOp};
use crate::get_lines;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// --- Day 10: Cathode-Ray Tube ---
/// https://adventofcode.com/2022/day/10

/// The state of the central processing unit in the Elves' handheld device at a given point in time
pub struct ProcessorState {
    /// The clock cycle indicating the point in time this state was in effect
    cycle: u16,
    /// The `X` register of the processor
    register: i32,
}

impl ProcessorState {
    /// A measurable aspect of the processor that is derived from the current clock cycle and the
    /// register's value
    pub fn signal_strength(&self) -> i32 {
        (self.cycle as i32) * self.register
    }
}

impl Default for ProcessorState {
    fn default() -> Self {
        Self {
            cycle: 1,
            register: 1,
        }
    }
}

/// A low-level instruction for the Elves' handheld device
pub enum Instruction {
    /// Do nothing
    NoOp,

    /// Increase the `X` register by the value specified
    AddX(i32),
}

impl Instruction {
    /// The number of clock cycles it takes this instruction to complete
    fn cycles(&self) -> usize {
        match self {
            AddX(_) => 2,
            _ => 1,
        }
    }

    /// Execute a single instruction. The instruction may take multiple cycles to complete and a
    /// separate processor state is emitted for each cycle elapsed.
    pub fn execute(&self, current_state: &ProcessorState) -> Vec<ProcessorState> {
        let mut result = vec![];
        let mut inc = 1;
        for _ in 0..self.cycles() - 1 {
            result.push(ProcessorState {
                cycle: current_state.cycle + inc,
                register: current_state.register,
            });
            inc += 1;
        }
        let last = match self {
            NoOp => ProcessorState {
                cycle: current_state.cycle + inc,
                register: current_state.register,
            },
            AddX(argument) => ProcessorState {
                cycle: current_state.cycle + inc,
                register: current_state.register + argument,
            },
        };
        result.push(last);
        result
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split(' ');
        let instruction = components.next().expect("Instruction not specified");
        if instruction == "noop" {
            return Ok(NoOp);
        } else if instruction == "addx" {
            let argument = components
                .next()
                .expect("Argument required")
                .parse::<i32>()
                .expect("Unparseable argument");
            return Ok(AddX(argument));
        }
        Err("Unrecognised instruction")
    }
}

/// The CRT display on the Elves' handheld device
pub struct HandheldDisplay {
    pixels: [[char; 40]; 6],
}

impl HandheldDisplay {
    /// Update the pixels based on the current processor state
    pub fn update(&mut self, state: &ProcessorState) {
        let pixel_index = (state.cycle - 1) as usize;

        // Part or all of the sprite might be off screen
        let mut sprite_positions = vec![];
        if state.register > 0 {
            sprite_positions.push((state.register - 1) as usize);
        }
        if state.register >= 0 {
            sprite_positions.push(state.register as usize);
        }
        if state.register + 1 >= 0 {
            sprite_positions.push((state.register + 1) as usize);
        }

        // determine which pixel is currently being drawn
        let row = pixel_index / 40;
        let column = pixel_index % 40;
        if row < self.pixels.len() {
            self.pixels[row][column] = if sprite_positions.contains(&column) {
                '#'
            } else {
                '.'
            };
        }
    }
}

impl Display for HandheldDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.pixels.len() {
            writeln!(f, "{}", self.pixels[i].iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Default for HandheldDisplay {
    fn default() -> Self {
        Self {
            pixels: [['.'; 40]; 6],
        }
    }
}

pub fn get_input() -> impl Iterator<Item = Instruction> {
    get_lines("day-10.txt")
        .map(|line| line.parse::<Instruction>())
        .map(Result::unwrap)
}

#[cfg(test)]
mod tests {

    use crate::day10::{get_input, HandheldDisplay, ProcessorState};

    #[test]
    fn part1() {
        let interesting_cycles = vec![20u16, 60, 100, 140, 180, 220];
        let mut state: ProcessorState = Default::default();
        let mut total_signal_strength = 0;
        for instruction in get_input() {
            for result in instruction.execute(&state) {
                if interesting_cycles.contains(&result.cycle) {
                    total_signal_strength += result.signal_strength();
                }
                state = result;
            }
        }

        println!("Part 1: {}", total_signal_strength);
    }

    #[test]
    fn part2() {
        let mut state: ProcessorState = Default::default();
        let mut display: HandheldDisplay = Default::default();
        display.update(&state);
        for instruction in get_input() {
            for result in instruction.execute(&state) {
                display.update(&result);
                state = result;
            }
        }

        println!("Part 2:\n{}", display);
    }
}
