// --- Day 22: Crab Combat ---
// https://adventofcode.com/2020/day/22

use crate::get_lines;

type Card = u8;

pub struct Deck {
    cards: Vec<Card>, // TODO try linked list
}

impl Deck {
    pub fn draw_top(&mut self) -> Card {
        let result = self.cards[0];
        self.cards.remove(0);
        result
    }

    pub fn insert(&mut self, winning_card: Card, losing_card: Card) {
        self.cards.push(winning_card);
        self.cards.push(losing_card);
    }

    pub fn score(&self) -> usize {
        let mut multiplier = self.cards.len();
        let mut score = 0usize;
        for card in &self.cards {
            score += multiplier * (*card as usize);
            multiplier -= 1;
        }
        score
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

pub fn get_input() -> (Deck, Deck) {
    let mut player_1 = Deck {
        cards: vec![],
    };
    let mut player_2 = Deck {
        cards: vec![],
    };
    let mut target = &mut player_1;
    for line in get_lines("day-22-input.txt") {
        if line.eq("Player 1:") {
            target = &mut player_1;
        } else if line.eq("Player 2:") {
            target = &mut player_2;
        } else if line.is_empty() {
            continue;
        } else {
            target.cards.push(line.parse::<Card>().unwrap())
        }
    }
    (player_1, player_2)
}

#[cfg(test)]
mod tests {
    use crate::day22::get_input;

    #[test]
    fn part1() {
        let (mut player_1, mut player_2) = get_input();
        while !player_1.is_empty() && !player_2.is_empty() {
            let x = player_1.draw_top();
            let y = player_2.draw_top();
            assert_ne!(x, y);
            if x > y {
                player_1.insert(x, y);
            } else {
                player_2.insert(y, x);
            }
        }
        let winner = if player_1.is_empty() { player_2 } else { player_1 };
        let score = winner.score();
        println!("Part 1: {}", score);
    }

    #[test]
    fn part2() {

    }
}