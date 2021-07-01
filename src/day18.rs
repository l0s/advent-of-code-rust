// --- Day 18: Operation Order ---
// https://adventofcode.com/2020/day/18

use crate::get_lines;

type Int = u64;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

pub enum Token {
    Number(Int),
    StartGroup,
    EndGroup,
    Plus,
    Times,
}

pub fn get_input() -> impl Iterator<Item=Vec<Token>> {
    get_lines("day-18-input.txt")
        .map(|line| -> Vec<Token> {
            line.chars().flat_map(|c| -> Option<Token> {
                match c {
                    '0'..='9' => Some(Token::Number(c.to_digit(10).unwrap() as Int)),
                    '+' => Some(Token::Plus),
                    '*' => Some(Token::Times),
                    '(' => Some(Token::StartGroup),
                    ')' => Some(Token::EndGroup),
                    ' ' => None,
                    _ => panic!("Invalid char: {}", c),
                }
            }).collect()
        })
}

pub struct Frame {
    pub left_value: Int,
    pub operator: Operator,
}

#[cfg(test)]
mod tests {
    use crate::day18::{get_input, Int, Token, Frame, Operator};
    use crate::day18::Operator::{Add, Multiply};

    #[test]
    fn part1() {
        let sum = get_input().map(|tokens| -> Int {
            let mut stack: Vec<Frame> = vec![];

            let mut total: Option<Int> = None;
            let mut operator: Option<Operator> = None;

            tokens.iter().for_each(|token| {
                match token {
                    Token::Number(i) => {
                        match (total, operator.as_ref()) {
                            (None, None) => {
                                total = Some(*i);
                            },
                            (Some(left_value), Some(op)) => {
                                total = Some(match op {
                                    Operator::Add => left_value + i,
                                    Operator::Multiply => left_value * i,
                                });
                                operator = None;
                            },
                            (left_value, operator) => {
                                panic!("Invalid state: {:?} {:?}", left_value, operator);
                            }
                        }
                    }
                    Token::StartGroup => {
                        let frame = match (total, operator) {
                            (None, None) => Frame { left_value: 0, operator: Add },
                            (Some(left_value), Some(operator)) => Frame{ left_value, operator },
                            (_, _) => panic!("Cannot start group given {:?} {:?}", total, operator),
                        };
                        stack.push(frame);
                        total = None;
                        operator = None;
                    }
                    Token::EndGroup => {
                        assert!(total.is_some(), "Group has no value: {:?} {:?}", total, operator);
                        assert!(operator.is_none(), "Group cannot be closed while {:?} operator is unresolved.", operator);

                        let partial = total.unwrap();
                        let frame = stack.pop().expect("No frame on the stack");
                        total = Some(match frame.operator {
                            Add => frame.left_value + partial,
                            Multiply => frame.left_value * partial,
                        });
                    }
                    Token::Plus => {
                        assert!(total.is_some(), "No left_value for '+'");
                        assert!(operator.is_none(), "'+' cannot follow {:?}", operator.as_ref().unwrap());
                        operator = Some(Add);
                    }
                    Token::Times => {
                        assert!(total.is_some(), "No left_value for '*'");
                        assert!(operator.is_none(), "'*' cannot follow {:?}", operator.as_ref().unwrap());
                        operator = Some(Multiply);
                    }
                }
            });
            total.expect("Could not evaluate expression")
        }).sum::<Int>();

        println!("Part 1: {}", sum);
    }

    // #[test]
    fn part2() {
        assert!(false)
    }
}