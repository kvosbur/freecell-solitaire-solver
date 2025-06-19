//! Tableau implementation for FreeCell game state.

use crate::card::Card;
use crate::rules;

#[derive(Clone, PartialEq, Eq, Hash)]
/// Represents the main play area with multiple columns of cards
pub struct Tableau {
    columns: Vec<Vec<Card>>,
}

impl Tableau {
    /// Create a new tableau with the specified number of columns
    pub fn new() -> Self {
        Self { columns: vec![Vec::new(); 8] }
    }
    
    /// Add a card to the specified column
    pub fn push_card(&mut self, column: usize, card: Card) -> Result<(), TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        self.columns[column].push(card);
        Ok(())
    }
    
    /// Remove and return the top card from the specified column
    pub fn pop_card(&mut self, column: usize) -> Option<Card> {
        if column >= self.columns.len() || self.columns[column].is_empty() {
            return None;
        }
        self.columns[column].pop()
    }
    
    /// Get a reference to the top card in a column without removing it
    pub fn get_top_card(&self, column: usize) -> Option<&Card> {
        if column >= self.columns.len() {
            return None;
        }
        self.columns[column].last()
    }
    
    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
    
    /// Get the number of cards in a column
    pub fn column_length(&self, column: usize) -> usize {
        if column >= self.columns.len() {
            return 0;
        }
        self.columns[column].len()
    }
    
    /// Check if a column is empty
    pub fn is_column_empty(&self, column: usize) -> bool {
        column < self.columns.len() && self.columns[column].is_empty()
    }
    
    /// Count the number of empty columns
    pub fn empty_columns_count(&self) -> usize {
        self.columns.iter().filter(|col| col.is_empty()).count()
    }
    
    /// Get a slice of cards from the top of a column
    pub fn get_column_top_cards(&self, column: usize, count: usize) -> &[Card] {
        if column >= self.columns.len() || count > self.columns[column].len() {
            return &[];
        }
        let start = self.columns[column].len() - count;
        &self.columns[column][start..]
    }
    
    /// Get an iterator over all columns
    pub fn columns(&self) -> impl Iterator<Item = &Vec<Card>> {
        self.columns.iter()
    }
}

use std::fmt;

impl fmt::Debug for Tableau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Tableau");
        for col in 0..self.column_count() {
            let column_name = format!("column_{}", col);
            if self.column_length(col) == 0 {
                debug_struct.field(&column_name, &"[empty]");
            } else {
                let mut cards = Vec::new();
                for i in 0..self.column_length(col) {
                    if let Some(card) = self.get_card_at(col, i) {
                        cards.push(format!("{:?}", card));
                    }
                }
                debug_struct.field(&column_name, &cards);
            }
        }
        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

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
            rank: Rank::Seven,
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
            rank: Rank::Eight,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: Rank::Seven,
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
            rank: Rank::Eight,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: Rank::Seven,
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
            rank: Rank::Eight,
            suit: Suit::Spades,
        }; // Black 8
        let card2 = Card {
            rank: Rank::Six,
            suit: Suit::Hearts,
        }; // Red 6 (should be 7)
        tableau.add_card_to_column(0, card1.clone());
        tableau.add_card_to_column(0, card2.clone());
    }

    #[test]
    fn can_remove_card_from_column() {
        let mut tableau = Tableau::new();
        let card1 = Card {
            rank: Rank::Seven,
            suit: Suit::Hearts,
        };
        let card2 = Card {
            rank: Rank::Six,
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
            rank: Rank::Seven,
            suit: Suit::Hearts,
        };
        let card2 = Card {
            rank: Rank::Six,
            suit: Suit::Spades,
        };
        let card3 = Card {
            rank: Rank::Five,
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
                    rank: Rank::Two,
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
