//! Tableau implementation for FreeCell game state.

use crate::card::Card;
use crate::rules;

#[derive(Debug)]
pub struct Tableau {
    columns: [Vec<Card>; 8],
}

impl Tableau {
    pub fn new() -> Self {
        Self {
            columns: Default::default(),
        }
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn column_length(&self, index: usize) -> usize {
        self.columns[index].len()
    }

    pub fn is_column_empty(&self, index: usize) -> bool {
        self.columns[index].is_empty()
    }

    pub fn add_card_to_column(&mut self, index: usize, card: Card) {
        if let Some(top) = self.get_top_card(index) {
            if !rules::can_stack_on_tableau(&card, top) {
                panic!("Invalid tableau move: must alternate color and descend in rank");
            }
        }
        self.columns[index].push(card);
    }

    pub fn initial_addition_of_card(&mut self, index: usize, card: Card) {
        self.columns[index].push(card);
    }

    pub fn get_top_card(&self, index: usize) -> Option<&Card> {
        self.columns[index].last()
    }

    pub fn remove_card_from_column(&mut self, index: usize) -> Option<Card> {
        self.columns[index].pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit};

    #[test]
    fn tableau_initializes_with_eight_empty_columns() {
        // This test checks that a new Tableau has exactly 8 columns, and each column is empty.
        let tableau = Tableau::new();
        assert_eq!(tableau.column_count(), 8, "Tableau should have 8 columns");
        for i in 0..tableau.column_count() {
            assert_eq!(
                tableau.column_length(i),
                0,
                "Column {} should be empty on initialization",
                i
            );
            assert!(
                tableau.is_column_empty(i),
                "is_column_empty({}) should be true on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_column() {
        let mut tableau = Tableau::new();
        let card = Card {
            rank: 7,
            suit: Suit::Hearts,
        };
        tableau.add_card_to_column(0, card.clone());
        assert_eq!(tableau.column_length(0), 1);
        assert!(!tableau.is_column_empty(0));
        assert_eq!(tableau.get_top_card(0), Some(&card));
    }

    #[test]
    fn can_stack_valid_card_on_tableau() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: 8,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: 7,
            suit: Suit::Hearts,
        }; // Red 7
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
        assert_eq!(tableau.column_length(0), 2);
        assert_eq!(tableau.get_top_card(0), Some(&card2));
    }

    #[test]
    #[should_panic(expected = "Invalid tableau move")]
    fn cannot_stack_invalid_card_on_tableau() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: 8,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: 7,
            suit: Suit::Clubs,
        }; // Black 7 (same color)
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
    }

    #[test]
    #[should_panic(expected = "Invalid tableau move")]
    fn cannot_stack_wrong_rank_on_tableau() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: 8,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: 6,
            suit: Suit::Hearts,
        }; // Red 6 (should be 7)
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
    }

    #[test]
    fn can_remove_card_from_column() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: 7,
            suit: Suit::Hearts,
        };
        let card2 = Card {
            rank: 6,
            suit: Suit::Spades,
        };
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
        let removed_card = tableau.remove_card_from_column(0);
        assert_eq!(removed_card, Some(card2));
        assert_eq!(tableau.column_length(0), 1);
        assert_eq!(tableau.get_top_card(0), Some(&card1));
    }

    #[test]
    fn removing_from_empty_column_returns_none() {
        let mut tableau = Tableau::new();
        let removed = tableau.remove_card_from_column(0);
        assert_eq!(removed, None);
    }

    #[test]
    fn multiple_adds_and_removes_are_lifo() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: 7,
            suit: Suit::Hearts,
        };
        let card2 = Card {
            rank: 6,
            suit: Suit::Spades,
        };
        let card3 = Card {
            rank: 5,
            suit: Suit::Diamonds,
        };
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
        tableau.add_card_to_column(0, card3.clone());
        assert_eq!(tableau.remove_card_from_column(0), Some(card3));
        assert_eq!(tableau.remove_card_from_column(0), Some(card2));
        assert_eq!(tableau.remove_card_from_column(0), Some(card1));
        assert_eq!(tableau.remove_card_from_column(0), None);
    }

    #[test]
    fn column_index_out_of_bounds_panics() {
        // Each closure must own its own Tableau to be UnwindSafe
        let result = std::panic::catch_unwind(|| {
            let mut tableau = Tableau::new();
            tableau.add_card_to_column(
                8,
                Card {
                    rank: 2,
                    suit: Suit::Clubs,
                },
            );
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let mut tableau = Tableau::new();
            tableau.remove_card_from_column(8);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let tableau = Tableau::new();
            tableau.get_top_card(8);
        });
        assert!(result.is_err());
    }
}
