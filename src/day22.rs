// --- Day 22: Crab Combat ---
// https://adventofcode.com/2020/day/22

use std::collections::HashSet;

use crate::get_lines;

/// A space card
type Card = u8;

/// A small deck of space cards
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Deck {
    cards: Vec<Card>, // TODO try linked list
}

impl Deck {
    /// Remove the first card so it can be played in a Round
    pub fn draw_top(&mut self) -> Card {
        let result = self.cards[0];
        self.cards.remove(0);
        result
    }

    /// Call if the controlling player won a round
    /// Parameters:
    /// - `winning_card` - the first card to insert (will become the penultimate card in the deck)
    /// - `losing_card` - the second card to insert (will become the bottom card in the deck)
    pub fn insert(&mut self, winning_card: Card, losing_card: Card) {
        self.cards.push(winning_card);
        self.cards.push(losing_card);
    }

    /// Calculate the current score of the deck
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

    /// Create a new deck based on the first _count_ space cards
    pub fn clone_first(&self, count: usize) -> Deck {
        let (first, _) = self.cards.split_at(count);
        Deck {
            cards: first.to_vec(),
        }
    }
}

/// One of two players in a game of _Combat_
#[derive(Eq)]
pub struct Player {
    id: u8,
    deck: Deck,
}

impl Player {
    pub fn draw_top(&mut self) -> Card {
        self.deck.draw_top()
    }

    pub fn insert(&mut self, winning_card: Card, losing_card: Card) {
        self.deck.insert(winning_card, losing_card)
    }

    pub fn has_cards(&self) -> bool {
        !self.deck.is_empty()
    }

    pub fn has_at_least(&self, card_count: u8) -> bool {
        self.deck.cards.len() >= card_count as usize
    }

    pub fn clone_first(&self, count: usize) -> Player {
        Player {
            id: self.id,
            deck: self.deck.clone_first(count),
        }
    }

    pub fn score(&self) -> usize {
        self.deck.score()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn get_input() -> (Player, Player) {
    let mut player_1 = Deck { cards: vec![] };
    let mut player_2 = Deck { cards: vec![] };
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
    (
        Player {
            id: 1,
            deck: player_1,
        },
        Player {
            id: 2,
            deck: player_2,
        },
    )
}

/// Play a game of _Recursive Combat_
pub fn play(mut player1: Player, mut player2: Player) -> Player {
    let mut previous_rounds: HashSet<(Deck, Deck)> = HashSet::new();
    while player1.has_cards() && player2.has_cards() {
        if !previous_rounds.insert((player1.deck.clone(), player2.deck.clone())) {
            return player1;
        }
        let x = player1.draw_top();
        let y = player2.draw_top();
        if player1.has_at_least(x) && player2.has_at_least(y) {
            let winner = play(
                player1.clone_first(x as usize),
                player2.clone_first(y as usize),
            );
            if winner == player1 {
                player1.insert(x, y);
            } else {
                player2.insert(y, x);
            }
        } else if x > y {
            player1.insert(x, y);
        } else {
            player2.insert(y, x);
        }
    }

    if player1.has_cards() {
        player1
    } else {
        player2
    }
}

#[cfg(test)]
mod tests {
    use crate::day22::{get_input, play};

    #[test]
    fn part1() {
        let (mut player_1, mut player_2) = get_input();
        while player_1.has_cards() && player_2.has_cards() {
            let x = player_1.draw_top();
            let y = player_2.draw_top();
            assert_ne!(x, y);
            if x > y {
                player_1.insert(x, y);
            } else {
                player_2.insert(y, x);
            }
        }
        let winner = if player_1.has_cards() {
            player_1
        } else {
            player_2
        };
        let score = winner.score();
        println!("Part 1: {}", score);
    }

    #[test]
    fn part2() {
        let (player_1, player_2) = get_input();
        let winner = play(player_1, player_2);
        println!("Part 2: {}", winner.score());
    }
}
