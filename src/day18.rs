// --- Day 18: Operation Order ---
// https://adventofcode.com/2020/day/18

use crate::get_lines;

type Int = u64;

/// All of the supported operators
///
/// All are binary, left-associative operators.
#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

/// All of the possible values in a single math problem
#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    /// A numerical literal
    Number(Int),

    /// An opening parenthesis that indicates the start of an expression that must be evaluated
    /// before it can be used by the surrounding expression. A corresponding `EndGroup` token must
    /// follow eventually.
    StartGroup,

    /// A closing parenthesis indicating the end of an expression that must be evaluated before it
    /// can be used by the surrounding expression. A corresponding `StartGroup` token must precede
    /// this.
    EndGroup,

    /// An infix operator that sums the adjacent expressions
    Plus,

    /// An infix operator that produces the product of the adjacent expressions
    Times,
}

/// An element in the postfix representation of a mathematical expression
pub enum Instruction {
    /// An instruction to push a number onto the stack
    Number(Int),

    /// An instruction to add the top two elements on the stack and push the result
    /// This cannot be interpreted as a unary operator.
    Add,

    /// An instruction to multiply the top two elements on the stack and push the result
    Multiply,
}

/// Tokenise the problem input
///
/// Returns: an `Iterator` in which each entry is a mathematical expression using infix notation
pub fn get_input() -> impl Iterator<Item = Vec<Token>> {
    get_lines("day-18-input.txt").map(|line| -> Vec<Token> {
        line.chars()
            .flat_map(|c| -> Option<Token> {
                match c {
                    '0'..='9' => Some(Token::Number(c.to_digit(10).unwrap() as Int)),
                    '+' => Some(Token::Plus),
                    '*' => Some(Token::Times),
                    '(' => Some(Token::StartGroup),
                    ')' => Some(Token::EndGroup),
                    ' ' => None,
                    _ => panic!("Invalid char: {}", c),
                }
            })
            .collect()
    })
}

/// Convert an expression from infix notation to postfix notation.
///
/// This uses the Shunting-yard algorithm.
///
/// Parameters:
/// - `tokens` - a mathematical expression that uses infix notation
/// - `precedences` - a function that returns larger numbers for higher precedence operators
pub fn convert_to_postfix<F>(tokens: Vec<Token>, precedence: F) -> Vec<Instruction>
where
    F: Fn(&Operator) -> u8,
{
    /// A subset of the tokens stored as operators in the Shunting-yard algorithm
    enum Op {
        /// An add operation with a precedence value
        Add(u8),

        /// A multiplication operation with a precedence value
        Multiply(u8),

        /// An indicator of the start of a group
        Group,
    }

    let mut operators = vec![];
    let mut result = vec![];

    let process_operator = |current_operator: Op,
                            current_operator_precedence: u8,
                            operators: &mut Vec<Op>|
     -> Vec<Instruction> {
        let mut result = vec![];
        while let Some(previous_operator) = operators.last() {
            match previous_operator {
                Op::Group => break,
                Op::Add(previous_operator_precedence) => {
                    if *previous_operator_precedence >= current_operator_precedence {
                        operators.pop();
                        result.push(Instruction::Add);
                    } else {
                        break;
                    }
                }
                Op::Multiply(previous_operator_precedence) => {
                    if *previous_operator_precedence >= current_operator_precedence {
                        operators.pop();
                        result.push(Instruction::Multiply);
                    } else {
                        break;
                    }
                }
            }
        }
        operators.push(current_operator);
        result
    };

    for token in tokens {
        match token {
            Token::Number(i) => result.push(Instruction::Number(i)),
            Token::StartGroup => operators.push(Op::Group),
            Token::EndGroup => loop {
                let operator = operators.pop().expect("Unexpected group ending");
                match operator {
                    Op::Group => break,
                    Op::Add(_) => {
                        assert!(!operators.is_empty(), "Missing opening parenthesis");
                        result.push(Instruction::Add);
                    }
                    Op::Multiply(_) => {
                        assert!(!operators.is_empty(), "Missing opening parenthesis");
                        result.push(Instruction::Multiply);
                    }
                }
            },
            Token::Plus => {
                let current_operator_precedence = precedence(&Operator::Add);
                result.append(&mut process_operator(
                    Op::Add(current_operator_precedence),
                    current_operator_precedence,
                    &mut operators,
                ))
            }
            Token::Times => {
                let current_operator_precedence = precedence(&Operator::Multiply);
                result.append(&mut process_operator(
                    Op::Multiply(current_operator_precedence),
                    current_operator_precedence,
                    &mut operators,
                ))
            }
        }
    }
    while !operators.is_empty() {
        let operator = operators.pop().unwrap();
        match operator {
            Op::Add(_) => result.push(Instruction::Add),
            Op::Multiply(_) => result.push(Instruction::Multiply),
            Op::Group => panic!("All groups should have been removed by now"),
        }
    }
    result
}

/// Evaluate a mathematical expression in postfix notation
///
/// Parameters:
/// - `elements` - The mathematical instructions in postfix order
///
/// Returns: the result of evaluating the expression
pub fn evaluate(elements: Vec<Instruction>) -> Int {
    let mut stack = vec![];
    for element in elements {
        match element {
            Instruction::Number(i) => stack.push(i),
            Instruction::Add => {
                let right_value = stack.pop().unwrap();
                let left_value = stack.pop().unwrap();
                stack.push(left_value + right_value);
            }
            Instruction::Multiply => {
                let right_value = stack.pop().unwrap();
                let left_value = stack.pop().unwrap();
                stack.push(left_value * right_value);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day18::Operator::{Add, Multiply};
    use crate::day18::{convert_to_postfix, evaluate, get_input, Int};

    #[test]
    fn part1() {
        let sum = get_input()
            .map(|tokens| {
                convert_to_postfix(tokens, |operator| match operator {
                    // "However, the rules of operator precedence have changed. Rather than
                    // evaluating multiplication before addition, the operators have the same
                    // precedence, and are evaluated left-to-right regardless of the order in which
                    // they appear."
                    Add => 1,
                    Multiply => 1,
                })
            })
            .map(evaluate)
            .sum::<Int>();
        println!("Part 1: {}", sum);
    }

    #[test]
    fn part2() {
        let sum = get_input()
            .map(|tokens| {
                convert_to_postfix(tokens, |operator| match operator {
                    // "Now, addition and multiplication have different precedence levels, but
                    // they're not the ones you're familiar with. Instead, addition is evaluated
                    // before multiplication."
                    Add => 2,
                    Multiply => 1,
                })
            })
            .map(evaluate)
            .sum::<Int>();
        println!("Part 2: {}", sum);
    }
}
