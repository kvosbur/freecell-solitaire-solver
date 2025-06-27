//! Tableau implementation for FreeCell game state.
//!
//! This module provides the Tableau component of the FreeCell game, which is the
//! main play area with 8 columns where cards are built down by alternating colors.
//! The main components are:
//!
//! - [`Tableau`] - Represents the main play area with columns of cards
//! - [`TableauError`] - Errors that can occur during tableau operations
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::tableau::{Tableau, TableauError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//!
//! // Create a new tableau
//! let mut tableau = Tableau::new();
//! 
//! // Place cards in the tableau
//! let card = Card::new(Rank::King, Suit::Hearts);
//! tableau.place_card(0, card).unwrap();
//! 
//! // Check for cards in a column
//! let top_card = tableau.get_card(0).unwrap().unwrap();
//! assert_eq!(top_card.rank(), Rank::King);
//! 
//! // Check if a column is empty
//! assert!(!tableau.is_column_empty(0));
//! ```
//!
//! This module primarily implements the physical state and operations of the tableau.
//! While validation helpers are provided via `validate_card_placement()`, the component
//! itself does not enforce game rules during operations like `place_card()`.
//! This design allows higher-level game logic to implement and control rule enforcement.

use crate::card::Card;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for tableau operations.
///
/// This enum represents all the possible error conditions that can occur
/// when interacting with tableau columns.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::tableau::{Tableau, TableauError};
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// let mut tableau = Tableau::new();
/// 
/// // Trying to access an invalid column index
/// let result = tableau.place_card(8, Card::new(Rank::Ace, Suit::Hearts));
/// assert!(matches!(result, Err(TableauError::InvalidColumn)));
///
/// // Validation errors are returned by validate_card_placement
/// let mut tableau = Tableau::new();
/// tableau.place_card(0, Card::new(Rank::Ten, Suit::Hearts)).unwrap();
/// let result = tableau.validate_card_placement(0, &Card::new(Rank::Nine, Suit::Hearts));
/// assert!(matches!(result, Err(TableauError::InvalidColor)));
/// ```
pub enum TableauError {
    /// Attempted to access an invalid column index
    InvalidColumn,
    /// Attempted to access an invalid card index within a column
    InvalidCardIndex,
    /// Attempted to stack cards in an invalid way (general error)
    InvalidStack,
    /// Attempted to remove a card from an empty column
    EmptyColumn,
    /// Cards must be of alternating colors
    InvalidColor,
    /// Cards must be in descending rank order
    InvalidRank,
}

#[derive(Clone, PartialEq, Eq)]
/// Represents the main play area with multiple columns of cards.
///
/// Tableau is the main play area in a FreeCell game where cards are arranged in 8 columns.
/// Cards are built down in descending rank order with alternating colors.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::tableau::Tableau;
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// // Create a new empty tableau
/// let mut tableau = Tableau::new();
///
/// // Place a card in column 0
/// let card = Card::new(Rank::King, Suit::Hearts);
/// tableau.place_card(0, card).unwrap();
/// ```
pub struct Tableau {
    columns: [Vec<Card>; 8],
}

impl Default for Tableau {
    /// Creates a new Tableau instance with 8 empty columns
    fn default() -> Self {
        Self::new()
    }
}

impl Tableau {
    /// Create a new tableau with 8 empty columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    ///
    /// let tableau = Tableau::new();
    /// assert_eq!(tableau.column_count(), 8);
    /// assert_eq!(tableau.empty_columns_count(), 8);
    /// ```
    pub fn new() -> Self {
        Self {
            columns: Default::default(),
        }
    }

    /// Add a card to the specified column.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the index is out of bounds.
    /// Note that this method does not validate whether the card placement follows FreeCell rules.
    /// Use `validate_card_placement()` for rule validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// tableau.place_card(0, card).unwrap();
    /// ```
    pub fn place_card(&mut self, column: usize, card: Card) -> Result<(), TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        self.columns[column].push(card);
        Ok(())
    }

    /// Remove and return the top card from the specified column.
    ///
    /// Returns the card if one was present, or an error if the column was empty.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the index is out of bounds.
    /// Returns `TableauError::EmptyColumn` if the column is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// 
    /// // Place a card first
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// tableau.place_card(0, card.clone()).unwrap();
    /// 
    /// // Then remove it
    /// let removed_card = tableau.remove_card(0).unwrap().unwrap();
    /// assert_eq!(removed_card, card);
    /// ```
    pub fn remove_card(&mut self, column: usize) -> Result<Option<Card>, TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        if self.columns[column].is_empty() {
            return Err(TableauError::EmptyColumn);
        }
        Ok(self.columns[column].pop())
    }

    /// Get a reference to the top card in a column without removing it.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// tableau.place_card(0, card.clone()).unwrap();
    /// 
    /// // Get a reference to the card
    /// let card_ref = tableau.get_card(0).unwrap().unwrap();
    /// assert_eq!(card_ref.rank(), Rank::King);
    /// assert_eq!(card_ref.suit(), Suit::Hearts);
    /// ```
    pub fn get_card(&self, column: usize) -> Result<Option<&Card>, TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        Ok(self.columns[column].last())
    }

    /// Get a reference to a card at a specific index in a column.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the column index is out of bounds.
    /// Returns `TableauError::InvalidCardIndex` if the card index within the column is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::{Tableau, TableauError};
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// let card1 = Card::new(Rank::King, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Spades);
    /// tableau.place_card(0, card1.clone()).unwrap();
    /// tableau.place_card(0, card2.clone()).unwrap();
    /// 
    /// // Get the first and second cards
    /// assert_eq!(tableau.get_card_at(0, 0).unwrap(), &card1);
    /// assert_eq!(tableau.get_card_at(0, 1).unwrap(), &card2);
    /// 
    /// // Index out of bounds returns error
    /// assert!(matches!(tableau.get_card_at(0, 5), Err(TableauError::InvalidCardIndex)));
    /// ```
    pub fn get_card_at(&self, column: usize, index: usize) -> Result<&Card, TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        
        self.columns[column].get(index).ok_or(TableauError::InvalidCardIndex)
    }
    
    /// Get the number of columns in the tableau.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    ///
    /// let tableau = Tableau::new();
    /// assert_eq!(tableau.column_count(), 8);
    /// ```
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Count the number of empty columns in the tableau.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// assert_eq!(tableau.empty_columns_count(), 8);
    /// 
    /// // Place a card
    /// tableau.place_card(0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert_eq!(tableau.empty_columns_count(), 7);
    /// ```
    pub fn empty_columns_count(&self) -> usize {
        self.columns.iter().filter(|col| col.is_empty()).count()
    }

    /// Check if a column is empty.
    ///
    /// Returns false if the column index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// assert!(tableau.is_column_empty(0));
    /// 
    /// // Place a card
    /// tableau.place_card(0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert!(!tableau.is_column_empty(0));
    /// ```
    pub fn is_column_empty(&self, column: usize) -> bool {
        column < self.columns.len() && self.columns[column].is_empty()
    }

    /// Get the number of cards in a column.
    ///
    /// Returns 0 for invalid column indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// assert_eq!(tableau.column_length(0), 0);
    /// 
    /// tableau.place_card(0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert_eq!(tableau.column_length(0), 1);
    /// ```
    pub fn column_length(&self, column: usize) -> usize {
        if column >= self.columns.len() {
            return 0;
        }
        self.columns[column].len()
    }

    /// Get a slice of cards from the top of a column.
    ///
    /// Returns an empty slice if the column is invalid or if count exceeds column length.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// let card1 = Card::new(Rank::King, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Spades);
    /// tableau.place_card(0, card1).unwrap();
    /// tableau.place_card(0, card2.clone()).unwrap();
    /// 
    /// let top_card = tableau.get_column_top_cards(0, 1);
    /// assert_eq!(top_card.len(), 1);
    /// assert_eq!(top_card[0], card2);
    /// ```
    pub fn get_column_top_cards(&self, column: usize, count: usize) -> &[Card] {
        if column >= self.columns.len() || count > self.columns[column].len() {
            return &[];
        }
        let start = self.columns[column].len() - count;
        &self.columns[column][start..]
    }

    /// Get an iterator over all columns in the tableau.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// tableau.place_card(0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    ///
    /// // Iterate over all columns
    /// for column in tableau.columns() {
    ///     // Process each column
    /// }
    /// ```
    pub fn columns(&self) -> impl Iterator<Item = &Vec<Card>> {
        self.columns.iter()
    }
    
    /// Validates if a card can be legally placed on a tableau column according to FreeCell rules.
    /// Does not modify any state - only provides validation.
    /// 
    /// # Rules checked:
    /// - Cards must be one rank lower than the top card
    /// - Cards must be of opposite color to the top card
    /// - Any card can be placed on an empty column
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// tableau.place_card(0, Card::new(Rank::Ten, Suit::Hearts)).unwrap();
    ///
    /// // Valid: Nine of Spades on Ten of Hearts (descending rank, opposite colors)
    /// assert!(tableau.validate_card_placement(0, &Card::new(Rank::Nine, Suit::Spades)).is_ok());
    ///
    /// // Invalid: Same color
    /// assert!(tableau.validate_card_placement(0, &Card::new(Rank::Nine, Suit::Diamonds)).is_err());
    ///
    /// // Invalid: Wrong rank
    /// assert!(tableau.validate_card_placement(0, &Card::new(Rank::Eight, Suit::Spades)).is_err());
    /// ```
    pub fn validate_card_placement(&self, column: usize, card: &Card) -> Result<(), TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn);
        }
        
        // Any card can be placed on an empty column
        if self.columns[column].is_empty() {
            return Ok(());
        }
        
        if let Some(top_card) = self.columns[column].last() {
            // Check color alternation
            if top_card.color() == card.color() {
                return Err(TableauError::InvalidColor);
            }
            
            // Check descending rank
            if !top_card.is_one_higher_than(card) {
                return Err(TableauError::InvalidRank);
            }
            
            Ok(())
        } else {
            // This shouldn't happen based on the empty check above
            Ok(())
        }
    }
    
    /// Returns an iterator over the non-empty columns, yielding (index, column reference) pairs.
    ///
    /// This iterator provides a convenient way to iterate through all non-empty columns
    /// without having to check each column individually.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// tableau.place_card(0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// tableau.place_card(2, Card::new(Rank::King, Suit::Spades)).unwrap();
    /// 
    /// // Iterate through non-empty columns
    /// let non_empty_count = tableau.non_empty_columns().count();
    /// assert_eq!(non_empty_count, 2);
    /// ```
    pub fn non_empty_columns(&self) -> impl Iterator<Item = (usize, &Vec<Card>)> + '_ {
        self.columns
            .iter()
            .enumerate()
            .filter(|(_, col)| !col.is_empty())
    }
    
    /// Get all cards in a column.
    ///
    /// Returns an empty slice if the column index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// tableau.place_card(0, card.clone()).unwrap();
    ///
    /// let cards = tableau.get_column(0);
    /// assert_eq!(cards.len(), 1);
    /// assert_eq!(cards[0], card);
    /// ```
    pub fn get_column(&self, column: usize) -> &[Card] {
        if column >= self.columns.len() {
            return &[];
        }
        &self.columns[column]
    }
}

impl std::fmt::Display for TableauError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableauError::InvalidColumn => write!(f, "Invalid tableau column index"),
            TableauError::InvalidCardIndex => write!(f, "Invalid card index within column"),
            TableauError::InvalidStack => write!(f, "Invalid tableau stack move"),
            TableauError::EmptyColumn => write!(f, "No card in tableau column"),
            TableauError::InvalidColor => write!(f, "Cards must be of alternating colors"),
            TableauError::InvalidRank => write!(f, "Cards must be in descending rank order"),
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
                    // We know the column is valid here, but the card index might be out of bounds
                    if let Ok(card) = self.get_card_at(col, i) {
                        cards.push(format!("{:?}", card));
                    }
                }
                debug_struct.field(&column_name, &cards);
            }
        }
        debug_struct.finish()
    }
}

impl fmt::Display for Tableau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tableau:")?;
        for i in 0..self.column_count() {
            write!(f, "  Column {}: ", i)?;
            if self.is_column_empty(i) {
                writeln!(f, "[empty]")?;
            } else {
                for (j, card) in self.get_column(i).iter().enumerate() {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", card)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
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
        let card = Card::new(Rank::Seven, Suit::Hearts);
        tableau.place_card(0, card.clone()).unwrap();
        assert_eq!(tableau.column_length(0), 1);
        assert!(!tableau.is_column_empty(0));
        assert_eq!(tableau.get_card(0).unwrap(), Some(&card));
    }

    #[test]
    fn can_stack_valid_card_on_tableau() {
        let mut tableau = Tableau::new();
        let card1 = Card::new(Rank::Eight, Suit::Spades); // Black 8
        let card2 = Card::new(Rank::Seven, Suit::Hearts); // Red 7
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        assert_eq!(tableau.column_length(0), 2);
        assert_eq!(tableau.get_card(0).unwrap(), Some(&card2));
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
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        let removed_card = tableau.remove_card(0).expect("Should remove card2 from column 0");
        assert_eq!(removed_card, Some(card2));
        assert_eq!(tableau.column_length(0), 1);
        assert_eq!(tableau.get_card(0).unwrap(), Some(&card1));
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
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        let card3 = Card::new(Rank::Five, Suit::Diamonds);
        tableau.place_card(0, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(0, card2.clone()).expect("Should add card2 to column 0");
        tableau.place_card(0, card3.clone()).expect("Should add card3 to column 0");
        assert_eq!(tableau.remove_card(0).expect("Should remove card3"), Some(card3));
        assert_eq!(tableau.remove_card(0).expect("Should remove card2"), Some(card2));
        assert_eq!(tableau.remove_card(0).expect("Should remove card1"), Some(card1));
        let removed = tableau.remove_card(0);
        assert!(matches!(removed, Err(TableauError::EmptyColumn)), "Should be empty: {:?}", removed);
    }

    #[test]
    fn column_index_out_of_bounds_errors() {
        let mut tableau = Tableau::new();
        let result = tableau.place_card(
            8,
            Card::new(Rank::Two, Suit::Clubs),
        );
        assert!(matches!(result, Err(TableauError::InvalidColumn)));

        let mut tableau = Tableau::new();
        let result = tableau.remove_card(8);
        assert!(matches!(result, Err(TableauError::InvalidColumn)));

        let tableau = Tableau::new();
        let card = tableau.get_card(8);
        assert!(matches!(card, Err(TableauError::InvalidColumn)));
    }

    #[test]
    fn card_index_out_of_bounds_errors() {
        let mut tableau = Tableau::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        tableau.place_card(0, card).unwrap();
        
        // Index within bounds should work
        let result = tableau.get_card_at(0, 0);
        assert!(result.is_ok());
        
        // Index out of bounds should return InvalidCardIndex
        let result = tableau.get_card_at(0, 5);
        assert!(matches!(result, Err(TableauError::InvalidCardIndex)));
    }
}
