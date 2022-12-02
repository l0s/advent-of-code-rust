/// --- Day 2: Rock Paper Scissors ---
/// https://adventofcode.com/2022/day/2
use std::str::FromStr;

use crate::day02::ResponseStrategy::{Draw, Lose, Win};
use crate::day02::Shape::{Paper, Rock, Scissors};

use crate::get_lines;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn beats(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn beaten_by(&self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    pub fn value(&self) -> u16 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

pub enum ResponseStrategy {
    Lose,
    Draw,
    Win,
}

impl ResponseStrategy {
    pub fn respond_to(&self, opponent: &Shape) -> Shape {
        match self {
            Lose => opponent.beats(),
            Draw => *opponent,
            Win => opponent.beaten_by(),
        }
    }
}

impl FromStr for ResponseStrategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

pub struct Round {
    opponent_shape: Shape,
    potential_response: Shape,
    response_strategy: ResponseStrategy,
}

impl Round {
    pub fn naïve_score(&self) -> u16 {
        outcome(&self.opponent_shape, &self.potential_response) + self.potential_response.value()
    }

    pub fn score(&self) -> u16 {
        let response = self.response_strategy.respond_to(&self.opponent_shape);
        outcome(&self.opponent_shape, &response) + response.value()
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split_whitespace();
        let opponent_shape = components
            .next()
            .ok_or("Opponent's shape code not found")?
            .parse::<Shape>()
            .ok()
            .ok_or("Could not decipher opponent's shape")?;
        let strategy_code = components.next().ok_or("Strategy code not found")?;
        let potential_response = strategy_code
            .parse::<Shape>()
            .ok()
            .ok_or("Could not decipher potential response")?;
        let response_strategy = strategy_code
            .parse::<ResponseStrategy>()
            .ok()
            .ok_or("Could not decipher response strategy")?;
        Ok(Round {
            opponent_shape,
            potential_response,
            response_strategy,
        })
    }
}

fn outcome(opponent: &Shape, player: &Shape) -> u16 {
    if opponent == player {
        3
    } else if opponent.beaten_by() == *player {
        6
    } else {
        0
    }
}

pub fn get_input() -> impl Iterator<Item = Round> {
    get_lines("day-02.txt").map(|line| line.parse::<Round>().expect("Unable to parse round"))
}

#[cfg(test)]
mod tests {
    use crate::day02::get_input;

    #[test]
    fn part1() {
        let result: u16 = get_input().map(|round| round.naïve_score()).sum();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let result: u16 = get_input().map(|round| round.score()).sum();

        println!("Part 2: {}", result);
    }
}
