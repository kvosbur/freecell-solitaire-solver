//! Tableau implementation for FreeCell solitaire.
//!
//! # Overview
//!
//! In FreeCell solitaire, the tableau consists of 8 columns where cards are initially dealt.
//! Cards in the tableau must be built down in descending rank and alternating colors.
//!
//! This module provides:
//! - [`Tableau`] - The main struct representing all tableau columns
//! - [`TableauError`] - Errors that can occur during tableau operations
//!
//! # Tableau Rules
//!
//! The rules for building tableau columns in FreeCell are:
//!
//! 1. Cards must be built down in descending rank (King → Queen → Jack → ...)
//! 2. Cards must alternate colors (red cards must be placed on black cards and vice versa)
//! 3. Any card can be placed on an empty column
//! 4. Multiple cards can be moved at once if there are enough free cells and empty columns
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::tableau::{Tableau, TableauError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//! use freecell_game_engine::location::TableauLocation;
//!
//! // Create a new tableau
//! let mut tableau = Tableau::new();
//! 
//! // Place cards in the tableau
//! let card = Card::new(Rank::King, Suit::Hearts);
//! let location = TableauLocation::new(0).unwrap();
//! tableau.place_card(location, card).unwrap();
//! 
//! // Check for cards in a column
//! let top_card = tableau.get_card(location).unwrap().unwrap();
//! assert_eq!(top_card.rank(), Rank::King);
//! 
//! // Check if a column is empty
//! assert!(!tableau.is_column_empty(location).unwrap());
//! 
//! // Validate placement according to FreeCell rules
//! let black_queen = Card::new(Rank::Queen, Suit::Spades);
//! assert!(tableau.validate_card_placement(location, &black_queen).is_ok());
//! ```
//!
//! This module primarily implements the physical state and operations of the tableau.
//! While validation helpers are provided via `validate_card_placement()`, the component
//! itself does not enforce game rules during operations like `place_card()`.
//! This design allows higher-level game logic to implement and control rule enforcement.

use crate::card::Card;
use crate::location::TableauLocation;
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
/// use freecell_game_engine::location::TableauLocation;
///
/// let mut tableau = Tableau::new();
/// 
/// // Validation errors are returned by validate_card_placement
/// let mut tableau = Tableau::new();
/// let location = TableauLocation::new(0).unwrap();
/// tableau.place_card(location, Card::new(Rank::Ten, Suit::Hearts)).unwrap();
/// let result = tableau.validate_card_placement(0, &Card::new(Rank::Nine, Suit::Hearts));
/// assert!(matches!(result, Err(TableauError::InvalidColor { .. })));
/// ```
pub enum TableauError {
    /// Attempted to access an invalid column index.
    InvalidColumn(u8),
    /// Attempted to access an invalid card index within a column.
    InvalidCardIndex,
    /// Attempted to stack cards in an invalid way (general error).
    InvalidStack,
    /// Card placement failed due to a color mismatch.
    InvalidColor {
        top_card: Card,
        new_card: Card,
    },
    /// Card placement failed due to a rank mismatch.
    InvalidRank {
        top_card: Card,
        new_card: Card,
    },
    /// Attempted to access more cards than exist in the column.
    InsufficientCards {
        column: u8,
        requested: usize,
        available: usize,
    },
    /// Attempted to perform an operation on an empty column.
    EmptyColumn(u8),
    /// No valid placement found for the card.
    InvalidPlacement {
        card: Card,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
/// use freecell_game_engine::location::TableauLocation;
///
/// // Create a new empty tableau
/// let mut tableau = Tableau::new();
///
/// // Place a card in column 0
/// let card = Card::new(Rank::King, Suit::Hearts);
/// let location = TableauLocation::new(0).unwrap();
/// tableau.place_card(location, card).unwrap();
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// tableau.place_card(TableauLocation::new(0).unwrap(), card).unwrap();
    /// ```
    pub fn place_card(&mut self, location: TableauLocation, card: Card) -> Result<(), TableauError> {
        self.columns[location.index() as usize].push(card);
        Ok(())
    }

    /// Remove and return the top card from the specified column.
    ///
    /// Returns the card if one was present, or an error if the column was empty.
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// 
    /// // Place a card first
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, card.clone()).unwrap();
    /// 
    /// // Then remove it
    /// let removed_card = tableau.remove_card(location).unwrap().unwrap();
    /// assert_eq!(removed_card, card);
    /// ```
    pub fn remove_card(&mut self, location: TableauLocation) -> Result<Option<Card>, TableauError> {
        Ok(self.columns[location.index() as usize].pop())
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, card.clone()).unwrap();
    /// 
    /// // Get a reference to the card
    /// let card_ref = tableau.get_card(location).unwrap().unwrap();
    /// assert_eq!(card_ref.rank(), Rank::King);
    /// assert_eq!(card_ref.suit(), Suit::Hearts);
    /// ```
    pub fn get_card(&self, location: TableauLocation) -> Result<Option<&Card>, TableauError> {
        Ok(self.columns[location.index() as usize].last())
    }

    /// Get a reference to a card at a specific index in a column.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the location is out of bounds.
    /// Returns `TableauError::InvalidCardIndex` if the card index within the column is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::{Tableau, TableauError};
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let card1 = Card::new(Rank::King, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Spades);
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, card1.clone()).unwrap();
    /// tableau.place_card(location, card2.clone()).unwrap();
    /// 
    /// // Get the first and second cards
    /// assert_eq!(tableau.get_card_at(location, 0).unwrap(), &card1);
    /// assert_eq!(tableau.get_card_at(location, 1).unwrap(), &card2);
    /// 
    /// // Index out of bounds returns error
    /// assert!(matches!(tableau.get_card_at(location, 5), Err(TableauError::InvalidCardIndex)));
    /// ```
    pub fn get_card_at(&self, location: TableauLocation, index: usize) -> Result<&Card, TableauError> {
        let column = location.index() as usize;
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(location.index()));
        }
        
        self.columns[column].get(index).ok_or(TableauError::InvalidCardIndex)
    }
    
    /// Get a reference to a card at a specific index in a column using raw indices.
    ///
    /// This method is provided for backward compatibility with existing code.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the column index is out of bounds.
    /// Returns `TableauError::InvalidCardIndex` if the card index within the column is out of bounds.
    pub fn get_card_at_raw(&self, column: usize, index: usize) -> Result<&Card, TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(column as u8));
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// assert_eq!(tableau.empty_columns_count(), 8);
    /// 
    /// // Place a card
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert_eq!(tableau.empty_columns_count(), 7);
    /// ```
    pub fn empty_columns_count(&self) -> usize {
        self.columns.iter().filter(|col| col.is_empty()).count()
    }

    /// Check if a column is empty.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the location is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let location = TableauLocation::new(0).unwrap();
    /// assert!(tableau.is_column_empty(location).unwrap());
    /// 
    /// // Place a card
    /// tableau.place_card(location, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert!(!tableau.is_column_empty(location).unwrap());
    /// ```
    pub fn is_column_empty(&self, location: TableauLocation) -> Result<bool, TableauError> {
        let column = location.index() as usize;
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(location.index()));
        }
        Ok(self.columns[column].is_empty())
    }

    /// Get the number of cards in a column.
    ///
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the location is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let location = TableauLocation::new(0).unwrap();
    /// assert_eq!(tableau.column_length(location).unwrap(), 0);
    /// 
    /// tableau.place_card(location, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert_eq!(tableau.column_length(location).unwrap(), 1);
    /// ```
    pub fn column_length(&self, location: TableauLocation) -> Result<usize, TableauError> {
        let column = location.index() as usize;
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(location.index()));
        }
        Ok(self.columns[column].len())
    }
    
    /// Get the number of cards in a column using a raw index.
    ///
    /// Returns 0 for invalid column indices. This method is provided for backward compatibility
    /// and internal use.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// assert_eq!(tableau.column_length_raw(0), 0);
    /// 
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// assert_eq!(tableau.column_length_raw(0), 1);
    /// ```
    pub fn column_length_raw(&self, column: usize) -> usize {
        if column >= self.columns.len() {
            return 0;
        }
        self.columns[column].len()
    }

    /// Get a slice of cards from the top of a column.
    ///
    /// # Errors
    ///
    /// - Returns `TableauError::InvalidColumn` if the column index is out of bounds.
    /// - Returns `TableauError::InsufficientCards` if count exceeds column length.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let card1 = Card::new(Rank::King, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Spades);
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, card1).unwrap();
    /// tableau.place_card(location, card2.clone()).unwrap();
    /// 
    /// let top_card = tableau.get_column_top_cards(location, 1).unwrap();
    /// assert_eq!(top_card.len(), 1);
    /// assert_eq!(top_card[0], card2);
    /// ```
    pub fn get_column_top_cards(&self, location: TableauLocation, count: usize) -> Result<&[Card], TableauError> {
        let column = location.index() as usize;
        
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(location.index()));
        }
        
        if count > self.columns[column].len() {
            return Err(TableauError::InsufficientCards {
                column: location.index(),
                requested: count,
                available: self.columns[column].len(),
            });
        }
        
        let start = self.columns[column].len() - count;
        Ok(&self.columns[column][start..])
    }

    /// Get an iterator over all columns in the tableau.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, Card::new(Rank::King, Suit::Hearts)).unwrap();
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
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the location is out of bounds.
    /// Returns `TableauError::InvalidColor` if the card color is the same as the top card.
    /// Returns `TableauError::InvalidRank` if the card rank is not one lower than the top card.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::{Tableau, TableauError};
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, Card::new(Rank::Ten, Suit::Hearts)).unwrap();
    ///
    /// // Valid: Nine of Spades on Ten of Hearts (descending rank, opposite colors)
    /// assert!(tableau.validate_card_placement(location, &Card::new(Rank::Nine, Suit::Spades)).is_ok());
    ///
    /// // Invalid: Same color
    /// let result = tableau.validate_card_placement(location, &Card::new(Rank::Nine, Suit::Diamonds));
    /// assert!(matches!(result, Err(TableauError::InvalidColor { .. })));
    ///
    /// // Invalid: Wrong rank
    /// let result = tableau.validate_card_placement(location, &Card::new(Rank::Eight, Suit::Spades));
    /// assert!(matches!(result, Err(TableauError::InvalidRank { .. })));
    /// ```
    pub fn validate_card_placement(&self, location: TableauLocation, card: &Card) -> Result<(), TableauError> {
        let column = location.index() as usize;
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(location.index()));
        }
        
        // Any card can be placed on an empty column
        if self.columns[column].is_empty() {
            return Ok(());
        }
        
        if let Some(top_card) = self.columns[column].last() {
            // Check color alternation
            if top_card.color() == card.color() {
                return Err(TableauError::InvalidColor {
                    top_card: *top_card,
                    new_card: *card,
                });
            }
            
            // Check descending rank
            if !top_card.is_one_higher_than(card) {
                return Err(TableauError::InvalidRank {
                    top_card: *top_card,
                    new_card: *card,
                });
            }
            
            Ok(())
        } else {
            // This shouldn't happen based on the empty check above
            Ok(())
        }
    }
    
    /// Validates if a card can be legally placed on a tableau column using raw column index.
    /// This method is provided for backward compatibility with existing code.
    /// 
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the column index is out of bounds.
    /// Returns `TableauError::InvalidColor` if the card color is the same as the top card.
    /// Returns `TableauError::InvalidRank` if the card rank is not one lower than the top card.
    pub fn validate_card_placement_raw(&self, column: usize, card: &Card) -> Result<(), TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(column as u8));
        }
        
        // Any card can be placed on an empty column
        if self.columns[column].is_empty() {
            return Ok(());
        }
        
        if let Some(top_card) = self.columns[column].last() {
            // Check color alternation
            if top_card.color() == card.color() {
                return Err(TableauError::InvalidColor {
                    top_card: *top_card,
                    new_card: *card,
                });
            }
            
            // Check descending rank
            if !top_card.is_one_higher_than(card) {
                return Err(TableauError::InvalidRank {
                    top_card: *top_card,
                    new_card: *card,
                });
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let location0 = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location0, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// let location2 = TableauLocation::new(2).unwrap();
    /// tableau.place_card(location2, Card::new(Rank::King, Suit::Spades)).unwrap();
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
    /// # Errors
    ///
    /// Returns `TableauError::InvalidColumn` if the column index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::tableau::Tableau;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut tableau = Tableau::new();
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// let location = TableauLocation::new(0).unwrap();
    /// tableau.place_card(location, card.clone()).unwrap();
    ///
    /// let cards = tableau.get_column(0).unwrap();
    /// assert_eq!(cards.len(), 1);
    /// assert_eq!(cards[0], card);
    /// ```
    pub fn get_column(&self, column: usize) -> Result<&[Card], TableauError> {
        if column >= self.columns.len() {
            return Err(TableauError::InvalidColumn(column as u8));
        }
        Ok(&self.columns[column])
    }

}

impl std::fmt::Display for TableauError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableauError::InvalidColumn(index) => write!(f, "Invalid tableau column index: {}", index),
            TableauError::InvalidCardIndex => write!(f, "Invalid card index within column"),
            TableauError::InvalidStack => write!(f, "Invalid tableau stack move"),
            TableauError::InvalidColor { top_card, new_card } => write!(
                f,
                "Cannot place {} on {}: colors are not alternating",
                new_card, top_card
            ),
            TableauError::InvalidRank { top_card, new_card } => write!(
                f,
                "Cannot place {} on {}: rank is not one lower",
                new_card, top_card
            ),
            TableauError::InsufficientCards { column, requested, available } => write!(
                f,
                "Insufficient cards in column {}: requested {} but only {} available",
                column, requested, available
            ),
            TableauError::EmptyColumn(column) => write!(
                f,
                "Column {} is empty",
                column
            ),
            TableauError::InvalidPlacement { card } => write!(
                f,
                "No valid placement found for card {}",
                card
            ),
        }
    }
}

impl std::error::Error for TableauError {}

impl fmt::Debug for Tableau {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Tableau");
        for col in 0..self.column_count() {
            let column_name = format!("column_{}", col);
            if self.column_length_raw(col) == 0 {
                debug_struct.field(&column_name, &"[empty]");
            } else {
                let mut cards = Vec::new();
                for i in 0..self.column_length_raw(col) {
                    // We know the column is valid here, but the card index might be out of bounds
                    if let Ok(card) = self.get_card_at_raw(col, i) {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tableau:")?;
        for i in 0..self.column_count() {
            write!(f, "  Column {}: ", i)?;
            let is_empty = if let Ok(location) = TableauLocation::new(i as u8) {
                self.is_column_empty(location).unwrap_or(true)
            } else {
                true
            };
            
            if is_empty {
                writeln!(f, "[empty]")?;
            } else {
                if let Ok(cards) = self.get_column(i) {
                    for (j, card) in cards.iter().enumerate() {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", card)?;
                    }
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
    use crate::location::TableauLocation;

    #[test]
    fn tableau_initializes_with_eight_empty_columns() {
        // This test checks that a new Tableau has exactly 8 columns, and each column is empty.
        let tableau = Tableau::new();
        assert_eq!(tableau.column_count(), 8, "Tableau should have 8 columns");
        for i in 0..tableau.column_count() {
            assert_eq!(
                tableau.column_length_raw(i),
                0,
                "Column {} should be empty on initialization",
                i
            );
            let location = TableauLocation::new(i as u8).unwrap();
            assert!(
                tableau.is_column_empty(location).unwrap(),
                "is_column_empty({}) should be true on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_column() {
        let mut tableau = Tableau::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, card.clone()).unwrap();
        assert_eq!(tableau.column_length(location).unwrap(), 1);
        assert!(!tableau.is_column_empty(location).unwrap());
        assert_eq!(tableau.get_card(location).unwrap(), Some(&card));
    }

    #[test]
    fn can_stack_valid_card_on_tableau() {
        let mut tableau = Tableau::new();
        let card1 = Card::new(Rank::Eight, Suit::Spades); // Black 8
        let card2 = Card::new(Rank::Seven, Suit::Hearts); // Red 7
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(location, card2.clone()).expect("Should add card2 to column 0");
        assert_eq!(tableau.column_length(location).unwrap(), 2);
        assert_eq!(tableau.get_card(location).unwrap(), Some(&card2));
    }

    #[test]
    fn cannot_stack_invalid_card_on_tableau() {
        let mut tableau = Tableau::new();
        let top_card = Card::new(Rank::Ten, Suit::Hearts); // Red
        let new_card = Card::new(Rank::Nine, Suit::Diamonds); // Red (invalid)
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, top_card.clone()).unwrap();

        let result = tableau.validate_card_placement(location, &new_card);
        assert!(matches!(
            result,
            Err(TableauError::InvalidColor { .. })
        ));
    }

    #[test]
    fn cannot_stack_wrong_rank_on_tableau() {
        let mut tableau = Tableau::new();
        let top_card = Card::new(Rank::Ten, Suit::Hearts); // Red
        let new_card = Card::new(Rank::Eight, Suit::Spades); // Black (wrong rank)
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, top_card.clone()).unwrap();

        let result = tableau.validate_card_placement(location, &new_card);
        assert!(matches!(
            result,
            Err(TableauError::InvalidRank { .. })
        ));
    }

    #[test]
    fn can_remove_card_from_column() {
        let mut tableau = Tableau::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(location, card2.clone()).expect("Should add card2 to column 0");
        let removed_card = tableau.remove_card(location).expect("Should remove card2 from column 0");
        assert_eq!(removed_card, Some(card2));
        assert_eq!(tableau.column_length(location).unwrap(), 1);
        assert_eq!(tableau.get_card(location).unwrap(), Some(&card1));
    }

    #[test]
    fn removing_from_empty_column_returns_ok_none() {
        let mut tableau = Tableau::new();
        let location = TableauLocation::new(0).unwrap();
        let removed = tableau.remove_card(location).unwrap();
        assert_eq!(removed, None);
    }

    #[test]
    fn multiple_adds_and_removes_are_lifo() {
        let mut tableau = Tableau::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        let card3 = Card::new(Rank::Five, Suit::Diamonds);
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, card1.clone()).expect("Should add card1 to column 0");
        tableau.place_card(location, card2.clone()).expect("Should add card2 to column 0");
        tableau.place_card(location, card3.clone()).expect("Should add card3 to column 0");
        assert_eq!(tableau.remove_card(location).expect("Should remove card3"), Some(card3));
        assert_eq!(tableau.remove_card(location).expect("Should remove card2"), Some(card2));
        assert_eq!(tableau.remove_card(location).expect("Should remove card1"), Some(card1));
        let removed = tableau.remove_card(location).unwrap();
        assert_eq!(removed, None, "Should be empty: {:?}", removed);
    }

    // Note: With TableauLocation, out-of-bounds errors are prevented at compile time.
    #[test]
    fn tableau_location_prevents_out_of_bounds() {
        let mut tableau = Tableau::new();
        let card = Card::new(Rank::Two, Suit::Clubs);
        
        // This will fail to compile if using an invalid index with TableauLocation::new
        let valid_location = TableauLocation::new(7).unwrap();
        assert!(tableau.place_card(valid_location, card).is_ok());

        // The following would not compile:
        // let invalid_location = TableauLocation::new(8).unwrap();
    }

    #[test]
    fn card_index_out_of_bounds_errors() {
        let mut tableau = Tableau::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        let location = TableauLocation::new(0).unwrap();
        tableau.place_card(location, card).unwrap();
        
        // Index within bounds should work
        let result = tableau.get_card_at(location, 0);
        assert!(result.is_ok());
        
        // Index out of bounds should return InvalidCardIndex
        let result = tableau.get_card_at(location, 5);
        assert!(matches!(result, Err(TableauError::InvalidCardIndex)));
    }

    #[test]
    fn can_use_tableau_location() {
        let mut tableau = Tableau::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        let location = TableauLocation::new(0).unwrap();

        tableau.place_card(location, card.clone()).unwrap();
        assert_eq!(tableau.get_card(location).unwrap(), Some(&card));
        assert_eq!(tableau.remove_card(location).unwrap(), Some(card));
        assert_eq!(tableau.get_card(location).unwrap(), None);
    }
}
