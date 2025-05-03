use crate::cards::{Card, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct Shoe {
    cards: Vec<Card>,
    force_shuffle: bool,
    pub number_of_cards: usize,
    pub number_of_shuffles: usize,
    pub out_of_cards: usize,
    cut_card: usize,
    burn_card: usize,
    next_card: usize,
    last_discard: usize,
}

impl Shoe {
    pub fn new(number_of_decks: usize, penetration: f64) -> Self {
        let mut cards = Vec::new();

        // Populate the shoe with cards based on the number of decks
        for _ in 0..number_of_decks {
            for suit in Suit::iter() {
                for rank in Rank::iter() {
                    cards.push(Card::new(rank, suit));
                }
            }
        }

        let number_of_cards = cards.len();
        let cut_card = (number_of_cards as f64 * penetration).round() as usize;

        let mut shoe = Shoe {
            cards,
            force_shuffle: false,
            number_of_cards,
            number_of_shuffles: 0,
            out_of_cards: 0,
            cut_card,
            burn_card: 1,
            next_card: number_of_cards,
            last_discard: number_of_cards,
        };

        shoe.shuffle();
        shoe
    }

    // Method to draw a card from the shoe
    pub fn draw_card(&mut self) -> Option<Card> {
        if self.next_card >= self.number_of_cards {
            self.force_shuffle = true;
            self.out_of_cards += 1;
            self.shuffle_random();
        }
        let card = self.cards.get(self.next_card).cloned();
        self.next_card += 1;
        card
    }

    // Method to shuffle the deck
    pub fn shuffle(&mut self) {
        self.last_discard = self.number_of_cards;
        self.force_shuffle = false;
        self.shuffle_random();
    }

    // Random shuffling method using the RNG
    fn shuffle_random(&mut self) {
        let mut rng = rand::rng();
        self.cards[..self.last_discard].shuffle(&mut rng);
        self.next_card = self.burn_card;
        self.number_of_shuffles += 1;
    }

    // Method to check if the deck should be shuffled
    pub fn should_shuffle(&mut self) -> bool {
        self.last_discard = self.next_card;
        (self.next_card >= self.cut_card) || self.force_shuffle
    }
}

#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn test_shoe_creation() {
        for decks in [1, 2, 6] {
            let mut shoe = Shoe::new(decks, 0.5);
            assert_eq!(shoe.number_of_cards, 52 * decks);
            assert_eq!(shoe.cut_card, 26 * decks);
            let card = shoe.draw_card();
            assert!(card.is_some());
        }
    }

    #[test]
    fn test_shuffle() {
        for decks in [1, 2, 6] {
            let mut shoe = Shoe::new(decks, 0.5);
            let original_cards = shoe.cards.clone();
            shoe.shuffle();
            assert_ne!(original_cards, shoe.cards);
            assert!(!shoe.should_shuffle());
            shoe.next_card = shoe.number_of_cards;
            assert!(shoe.should_shuffle());
        }
    }

    #[test]
    fn test_force_shuffle() {
        for decks in [1, 2, 6] {
            let mut shoe = Shoe::new(decks, 0.5);
            let shuffles = shoe.number_of_shuffles;
            for _ in 0..shoe.number_of_cards {
                shoe.draw_card();
                shoe.last_discard = shoe.next_card;
            }
            assert!(shoe.should_shuffle());
            assert!(shoe.number_of_shuffles == shuffles + 1);
            assert!(shoe.out_of_cards == 1);
        }
    }
}
