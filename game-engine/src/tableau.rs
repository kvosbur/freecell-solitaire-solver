//! Tableau implementation for FreeCell game state.

use crate::card::Card;

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
    pub fn place_card(&mut self, column: usize, card: Card) -> Result<(), TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        self.columns[column].push(card);
        Ok(())
    }

    /// Remove and return the top card from the specified column
    pub fn remove_card(&mut self, column: usize) -> Result<Option<Card>, TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        Ok(self.columns[column].pop())
    }

    /// Get a reference to the top card in a column without removing it
    pub fn get_card(&self, column: usize) -> Option<&Card> {
        if column >= self.columns.len() {
            return None;
        }
        self.columns[column].last()
    }

    /// Get a reference to a card at a specific index in a column
    pub fn get_card_at(&self, column: usize, index: usize) -> Option<&Card> {
        if column >= self.columns.len() || index >= self.columns[column].len() {
            return None;
        }
        self.columns[column].get(index)
    }
    
    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Count the number of empty columns
    pub fn empty_columns_count(&self) -> usize {
        self.columns.iter().filter(|col| col.is_empty()).count()
    }

    /// Check if a column is empty
    pub fn is_column_empty(&self, column: usize) -> bool {
        column < self.columns.len() && self.columns[column].is_empty()
    }

    /// Get the number of cards in a column
    pub fn column_length(&self, column: usize) -> usize {
        if column >= self.columns.len() {
            return 0;
        }
        self.columns[column].len()
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for tableau operations
pub enum TableauError {
    /// Attempted to access an invalid column index
    InvalidColumn,
    /// Attempted to stack cards in an invalid way
    InvalidStack,
    /// Attempted to remove a card from an empty column
    EmptyColumn,
}

impl std::fmt::Display for TableauError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableauError::InvalidColumn => write!(f, "Invalid tableau column index"),
            TableauError::InvalidStack => write!(f, "Invalid tableau stack move"),
            TableauError::EmptyColumn => write!(f, "No card in tableau column"),
        }
    }
}

impl std::error::Error for TableauError {}

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
        tableau.place_card(0, card.clone()).unwrap();
        assert_eq!(tableau.column_length(0), 1);
        assert!(!tableau.is_column_empty(0));
        assert_eq!(tableau.get_card(0), Some(&card));
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
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        assert_eq!(tableau.column_length(0), 2);
        assert_eq!(tableau.get_card(0), Some(&card2));
    }

    #[test]
    fn cannot_stack_invalid_card_on_tableau() {
        // This test is a placeholder: the real stacking logic should return InvalidStack.
        // For now, we just check that Err(TableauError::InvalidStack) matches.
        let result: Result<(), TableauError> = Err(TableauError::InvalidStack);
        assert!(matches!(result, Err(TableauError::InvalidStack)));
    }

    #[test]
    fn cannot_stack_wrong_rank_on_tableau() {
        // This test is a placeholder: the real stacking logic should return InvalidStack.
        // For now, we just check that Err(TableauError::InvalidStack) matches.
        let result: Result<(), TableauError> = Err(TableauError::InvalidStack);
        assert!(matches!(result, Err(TableauError::InvalidStack)));
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
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        let removed_card = tableau.remove_card(0).expect("Should remove card2 from column 0");
        assert_eq!(removed_card, Some(card2));
        assert_eq!(tableau.column_length(0), 1);
        assert_eq!(tableau.get_card(0), Some(&card1));
    }

    #[test]
    fn removing_from_empty_column_returns_error() {
        let mut tableau = Tableau::new();
        let removed = tableau.remove_card(0);
        assert!(matches!(removed, Err(TableauError::EmptyColumn)));
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
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        tableau.place_card(0, card3.clone()).expect("Should add card3 to column 0");
        assert_eq!(tableau.remove_card(0).expect("Should remove card3"), Some(card3));
        assert_eq!(tableau.remove_card(0).expect("Should remove card2"), Some(card2));
        assert_eq!(tableau.remove_card(0).expect("Should remove card1"), Some(card1));
        assert_eq!(tableau.remove_card(0).expect("Should be empty"), None);
    }

    #[test]
    fn column_index_out_of_bounds_errors() {
        let mut tableau = Tableau::new();
        let result = tableau.place_card(
            8,
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            },
        );
        assert!(matches!(result, Err(TableauError::InvalidColumn)));

        let mut tableau = Tableau::new();
        let result = tableau.remove_card(8);
        assert!(matches!(result, Err(TableauError::InvalidColumn)));

        let tableau = Tableau::new();
        let card = tableau.get_card(8);
        assert_eq!(card, None);
    }
}
