//! FreeCells implementation for FreeCell game state.
//!
//! This module provides the FreeCells component of the FreeCell game, which are the
//! 4 temporary storage slots where single cards can be placed. The main components are:
//!
//! - [`FreeCells`] - Represents the set of free cells in a game
//! - [`FreeCellError`] - Errors that can occur during freecell operations
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::freecells::{FreeCells, FreeCellError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//!
//! // Create a new set of freecells
//! let mut freecells = FreeCells::new();
//! 
//! // Place a card in a freecell
//! let card = Card::new(Rank::Ace, Suit::Spades);
//! freecells.place_card(0, card).unwrap();
//! 
//! // Check if a cell is empty
//! assert!(!freecells.is_cell_empty(0).unwrap());
//! 
//! // Remove a card from a freecell
//! let removed_card = freecells.remove_card(0).unwrap().unwrap();
//! assert_eq!(removed_card, card);
//! ```

use crate::card::Card;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the free cells where individual cards can be stored.
///
/// FreeCells are the 4 temporary storage slots in a FreeCell game where any
/// single card can be placed. A card in a freecell can then be moved to a tableau
/// column or to a foundation pile according to the game rules.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::freecells::FreeCells;
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// // Create a new empty set of freecells
/// let mut freecells = FreeCells::new();
///
/// // Place a card in cell 0
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// freecells.place_card(0, card).unwrap();
/// ```
pub struct FreeCells {
    cells: [Option<Card>; 4],
}

impl Default for FreeCells {
    /// Creates a new FreeCells instance with 4 empty cells
    fn default() -> Self {
        Self::new()
    }
}

impl FreeCells {
    /// Create a new set of freecells with 4 empty cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    ///
    /// let freecells = FreeCells::new();
    /// assert_eq!(freecells.cell_count(), 4);
    /// assert_eq!(freecells.empty_cells_count(), 4);
    /// ```
    pub fn new() -> Self {
        Self { cells: Default::default() }
    }
    
    /// Place a card in a freecell at the specified index.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::InvalidCell` if the index is out of bounds.
    /// Returns `FreeCellError::CellOccupied` if the cell already contains a card.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// freecells.place_card(0, card).unwrap();
    /// ```
    pub fn place_card(&mut self, cell_index: usize, card: Card) -> Result<(), FreeCellError> {
        if cell_index >= self.cells.len() {
            return Err(FreeCellError::InvalidCell);
        }
        
        if self.cells[cell_index].is_some() {
            return Err(FreeCellError::CellOccupied);
        }
        
        self.cells[cell_index] = Some(card);
        Ok(())
    }
    
    /// Remove and return a card from a freecell at the specified index.
    ///
    /// Returns the card if one was present, or `None` if the cell was empty.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::InvalidCell` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// 
    /// // Place a card first
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// freecells.place_card(0, card).unwrap();
    /// 
    /// // Then remove it
    /// let removed_card = freecells.remove_card(0).unwrap();
    /// assert_eq!(removed_card, Some(card));
    /// ```
    pub fn remove_card(&mut self, cell_index: usize) -> Result<Option<Card>, FreeCellError> {
        if cell_index >= self.cells.len() {
            return Err(FreeCellError::InvalidCell);
        }
        Ok(self.cells[cell_index].take())
    }
    
    /// Get a reference to a card in a freecell without removing it.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::InvalidCell` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// freecells.place_card(0, card).unwrap();
    /// 
    /// // Get a reference to the card
    /// let card_ref = freecells.get_card(0).unwrap().unwrap();
    /// assert_eq!(card_ref, &card);
    /// ```
    pub fn get_card(&self, cell_index: usize) -> Result<Option<&Card>, FreeCellError> {
        if cell_index >= self.cells.len() {
            return Err(FreeCellError::InvalidCell);
        }
        Ok(self.cells[cell_index].as_ref())
    }
    
    /// Get the number of freecells.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    ///
    /// let freecells = FreeCells::new();
    /// assert_eq!(freecells.cell_count(), 4);
    /// ```
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
    
    /// Count the number of empty cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// assert_eq!(freecells.empty_cells_count(), 4);
    /// 
    /// // Place a card
    /// freecells.place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// assert_eq!(freecells.empty_cells_count(), 3);
    /// ```
    pub fn empty_cells_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_none()).count()
    }
    
    /// Check if a cell is empty.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::InvalidCell` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// assert!(freecells.is_cell_empty(0).unwrap());
    /// 
    /// // Place a card
    /// freecells.place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// assert!(!freecells.is_cell_empty(0).unwrap());
    /// ```
    pub fn is_cell_empty(&self, cell_index: usize) -> Result<bool, FreeCellError> {
        if cell_index >= self.cells.len() {
            return Err(FreeCellError::InvalidCell);
        }
        Ok(self.cells[cell_index].is_none())
    }
    
    /// Returns an iterator over the non-empty cells, yielding (index, card reference) pairs.
    ///
    /// This iterator provides a convenient way to iterate through all occupied freecells
    /// without having to check each cell individually.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// freecells.place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// freecells.place_card(2, Card::new(Rank::King, Suit::Hearts)).unwrap();
    /// 
    /// // Iterate through occupied cells
    /// let occupied: Vec<_> = freecells.occupied_cells().collect();
    /// assert_eq!(occupied.len(), 2);
    /// ```
    pub fn occupied_cells(&self) -> impl Iterator<Item = (usize, &Card)> + '_ {
        self.cells.iter()
            .enumerate()
            .filter_map(|(idx, cell)| cell.as_ref().map(|card| (idx, card)))
    }
    
    /// Place a card in the first available empty freecell.
    /// 
    /// Returns the index where the card was placed, or an error if all cells are full.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::NoEmptyCells` if all freecells are already occupied.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut freecells = FreeCells::new();
    /// 
    /// // Place a card in any empty cell
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// let index = freecells.place_in_any_empty_cell(card).unwrap();
    /// 
    /// // Verify the card was placed at the returned index
    /// assert_eq!(freecells.get_card(index).unwrap(), Some(&card));
    /// ```
    pub fn place_in_any_empty_cell(&mut self, card: Card) -> Result<usize, FreeCellError> {
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            if cell.is_none() {
                *cell = Some(card);
                return Ok(idx);
            }
        }
        Err(FreeCellError::NoEmptyCells)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for freecell operations.
///
/// This enum represents all the possible error conditions that can occur
/// when interacting with freecells.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::freecells::{FreeCells, FreeCellError};
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// let mut freecells = FreeCells::new();
/// 
/// // Trying to access an invalid cell index
/// let result = freecells.place_card(5, Card::new(Rank::Ace, Suit::Spades));
/// assert!(matches!(result, Err(FreeCellError::InvalidCell)));
///
/// // Trying to place a card in an occupied cell
/// freecells.place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
/// let result = freecells.place_card(0, Card::new(Rank::Two, Suit::Hearts));
/// assert!(matches!(result, Err(FreeCellError::CellOccupied)));
/// ```
pub enum FreeCellError {
    /// Attempted to access an invalid cell index
    InvalidCell,
    /// Attempted to place a card in an occupied cell
    CellOccupied,
    /// Attempted to place a card but all cells are full
    NoEmptyCells,
}

impl std::fmt::Display for FreeCellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreeCellError::InvalidCell => write!(f, "Invalid freecell index"),
            FreeCellError::CellOccupied => write!(f, "Freecell is already occupied"),
            FreeCellError::NoEmptyCells => write!(f, "No empty freecells available"),
        }
    }
}

impl fmt::Display for FreeCells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FreeCells:")?;
        for i in 0..self.cell_count() {
            match self.get_card(i) {
                Ok(Some(card)) => writeln!(f, "  Cell {}: {}", i, card)?,
                Ok(None) => writeln!(f, "  Cell {}: Empty", i)?,
                Err(_) => writeln!(f, "  Cell {}: Invalid", i)?,
            }
        }
        Ok(())
    }
}

impl std::error::Error for FreeCellError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{Card, Suit},
        Rank,
    };

    #[test]
    fn freecells_initialize_with_four_empty_cells() {
        let freecells = FreeCells::new();
        assert_eq!(freecells.cell_count(), 4, "FreeCells should have 4 cells");
        assert_eq!(
            freecells.empty_cells_count(),
            4,
            "All cells should be empty initially"
        );
        for i in 0..freecells.cell_count() {
            assert!(
                freecells.is_cell_empty(i).unwrap(),
                "Cell {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_freecell() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        freecells.place_card(0, card).unwrap();
        assert!(!freecells.is_cell_empty(0).unwrap());
        assert_eq!(freecells.empty_cells_count(), 3);
        assert_eq!(freecells.get_card(0).unwrap(), Some(&card));
    }

    #[test]
    fn can_remove_card_from_freecell() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        freecells.place_card(0, card).unwrap();
        let removed_card = freecells.remove_card(0).unwrap();
        assert_eq!(removed_card, Some(card));
        assert!(freecells.is_cell_empty(0).unwrap());
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    fn removing_from_empty_freecell_returns_none() {
        let mut freecells = FreeCells::new();
        let removed = freecells.remove_card(0).unwrap();
        assert_eq!(removed, None);
        assert!(freecells.is_cell_empty(0).unwrap());
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    fn adding_card_to_occupied_freecell_panics() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        freecells.place_card(0, card1).unwrap();
        let result = freecells.place_card(0, card2);
        assert!(matches!(result, Err(FreeCellError::CellOccupied)));
    }

    #[test]
    fn freecell_index_out_of_bounds_errors() {
        let mut freecells = FreeCells::new();
        let result = freecells.place_card(
            4,
            Card::new(Rank::Two, Suit::Clubs),
        );
        assert!(matches!(result, Err(FreeCellError::InvalidCell)));

        let mut freecells = FreeCells::new();
        let result = freecells.remove_card(4);
        assert!(matches!(result, Err(FreeCellError::InvalidCell)));

        let freecells = FreeCells::new();
        let result = freecells.get_card(4);
        assert!(matches!(result, Err(FreeCellError::InvalidCell)));
    }

    #[test]
    fn occupied_cells_iterator_yields_non_empty_cells() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        freecells.place_card(0, card1.clone()).unwrap();
        freecells.place_card(2, card2.clone()).unwrap();

        let occupied: Vec<_> = freecells.occupied_cells().collect();
        assert_eq!(occupied.len(), 2);
        assert_eq!(occupied[0], (0, &card1));
        assert_eq!(occupied[1], (2, &card2));
    }

    #[test]
    fn can_iterate_over_occupied_cells() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Ace, Suit::Diamonds);
        
        freecells.place_card(0, card1).unwrap();
        freecells.place_card(2, card2).unwrap();
        
        let occupied: Vec<(usize, &Card)> = freecells.occupied_cells().collect();
        
        assert_eq!(occupied.len(), 2);
        assert_eq!(occupied[0], (0, &Card::new(Rank::Seven, Suit::Hearts)));
        assert_eq!(occupied[1], (2, &Card::new(Rank::Ace, Suit::Diamonds)));
    }

    #[test]
    fn can_place_card_in_any_empty_cell() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        
        // Place card in specific cell
        freecells.place_card(0, card1.clone()).unwrap();
        // Place card in any empty cell
        let placed_index = freecells.place_in_any_empty_cell(card2.clone()).unwrap();
        
        assert_eq!(placed_index, 1);
        assert_eq!(freecells.get_card(1).unwrap(), Some(&card2));
    }

    #[test]
    fn placing_when_cells_full_returns_error() {
        let mut freecells = FreeCells::new();
        
        // Fill all cells
        freecells.place_card(0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        freecells.place_card(1, Card::new(Rank::Two, Suit::Hearts)).unwrap();
        freecells.place_card(2, Card::new(Rank::Three, Suit::Hearts)).unwrap();
        freecells.place_card(3, Card::new(Rank::Four, Suit::Hearts)).unwrap();
        
        // Try to place another
        let result = freecells.place_in_any_empty_cell(Card::new(Rank::Five, Suit::Hearts));
        assert!(matches!(result, Err(FreeCellError::NoEmptyCells)));
    }

    #[test]
    fn is_cell_empty_handles_invalid_index() {
        let freecells = FreeCells::new();
        let result = freecells.is_cell_empty(4);
        assert!(matches!(result, Err(FreeCellError::InvalidCell)));
    }

    #[test]
    fn error_implements_error_trait() {
        // Check that we can use FreeCellError with Box<dyn Error>
        fn returns_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
            let mut freecells = FreeCells::new();
            freecells.place_card(5, Card::new(Rank::Ace, Suit::Hearts))?;
            Ok(())
        }

        let result = returns_box_dyn_error();
        assert!(result.is_err());
        
        // Convert to string to check Display implementation works
        let error_string = result.unwrap_err().to_string();
        assert_eq!(error_string, "Invalid freecell index");
    }

    #[test]
    fn display_formatting_works() {
        let mut freecells = FreeCells::new();
        freecells.place_card(1, Card::new(Rank::King, Suit::Hearts)).unwrap();
        
        let display_output = format!("{}", freecells);
        assert!(display_output.contains("Cell 0: Empty"));
        assert!(display_output.contains("Cell 1:"));  // Assuming Card's Display includes the value
        assert!(display_output.contains("Cell 2: Empty"));
        assert!(display_output.contains("Cell 3: Empty"));
    }
}
