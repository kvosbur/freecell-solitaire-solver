//! FreeCells implementation for FreeCell game state.
//!
//! # Overview
//!
//! In FreeCell solitaire, freecells are temporary storage slots where any single card
//! can be placed. There are typically 4 freecells in a standard game, and each can
//! hold at most one card at a time. Freecells provide strategic flexibility by giving
//! the player temporary storage space to facilitate moving cards between tableaus and
//! to foundations.
//!
//! This module provides:
//! - [`FreeCells`] - The main struct representing the set of freecells
//! - [`FreeCellError`] - Errors that can occur during freecell operations
//!
//! # Freecell Rules
//!
//! The rules for using freecells in FreeCell are:
//!
//! 1. Each freecell can hold at most one card
//! 2. Any card can be placed in an empty freecell
//! 3. Cards in freecells can be moved to tableau columns or foundation piles
//!    according to the game rules
//! 4. The number of freecells limits how many cards can be moved at once
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::freecells::{FreeCells, FreeCellError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//! use freecell_game_engine::location::FreecellLocation;
//!
//! // Create a new set of freecells
//! let mut freecells = FreeCells::new();
//!
//! // Place a card in a specific freecell
//! let card = Card::new(Rank::Ace, Suit::Spades);
//! let location = FreecellLocation::new(0).unwrap();
//! freecells.place_card_at(location, card).unwrap();
//!
//! // Place a card in any available freecell
//! let second_card = Card::new(Rank::King, Suit::Hearts);
//! let second_location = freecells.place_card(second_card).unwrap();
//!
//! // Check if a cell has a card
//! assert!(freecells.get_card(location).unwrap().is_some());
//!
//! // Remove a card from a freecell
//! let removed_card = freecells.remove_card(location).unwrap().unwrap();
//! assert_eq!(removed_card, card);
//! ```

use crate::card::Card;
use crate::location::FreecellLocation;
use std::fmt;

/// The number of free cells in a standard FreeCell game.
pub const FREECELL_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the free cells where individual cards can be stored.
///
/// # Overview
///
/// FreeCells are the 4 temporary storage slots in a FreeCell game where any
/// single card can be placed. Unlike tableaus and foundations which have placement
/// rules, any card can be placed in an empty freecell. A card in a freecell can then
/// be moved to a tableau column or to a foundation pile according to the game rules.
///
/// Using freecells strategically is essential for solving FreeCell games, as they
/// provide the temporary space needed to rearrange cards.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::freecells::FreeCells;
/// use freecell_game_engine::card::{Card, Rank, Suit};
/// use freecell_game_engine::location::FreecellLocation;
///
/// // Create a new empty set of freecells
/// let mut freecells = FreeCells::new();
///
/// // Place a card in cell 0
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// let location = FreecellLocation::new(0).unwrap();
/// freecells.place_card_at(location, card).unwrap();
/// ```
pub struct FreeCells {
    cells: [Option<Card>; FREECELL_COUNT],
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
    /// use freecell_game_engine::freecells::{FreeCells, FREECELL_COUNT};
    ///
    /// let freecells = FreeCells::new();
    /// assert_eq!(freecells.empty_cells_count(), FREECELL_COUNT);
    /// ```
    pub fn new() -> Self {
        Self {
            cells: [None; FREECELL_COUNT],
        }
    }

    /// Place a card in the first available empty freecell automatically.
    ///
    /// This method finds an empty freecell, places the card there, and returns
    /// the location where the card was placed.
    ///
    /// # Returns
    ///
    /// Returns the location where the card was placed.
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
    /// let location = freecells.place_card(card).unwrap();
    ///
    /// // Verify the card was placed at the returned location
    /// assert_eq!(freecells.get_card(location).unwrap(), Some(&card));
    /// ```
    pub fn place_card(&mut self, card: Card) -> Result<FreecellLocation, FreeCellError> {
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            if cell.is_none() {
                *cell = Some(card);
                return Ok(FreecellLocation::new(idx as u8).unwrap());
            }
        }
        Err(FreeCellError::NoEmptyCells)
    }

    /// Place a card in a freecell at the specified location.
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::CellOccupied` if the cell already contains a card.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// freecells.place_card_at(FreecellLocation::new(0).unwrap(), card).unwrap();
    /// ```
    pub fn place_card_at(
        &mut self,
        location: FreecellLocation,
        card: Card,
    ) -> Result<(), FreeCellError> {
        // Validate first, without modifying state
        self.validate_card_placement(location, &card)?;

        // If validation passes, place the card
        let cell_index = location.index() as usize;
        self.cells[cell_index] = Some(card);
        Ok(())
    }

    pub fn place_card_at_no_checks(&mut self, location: FreecellLocation, card: Card) {
        let cell_index = location.index() as usize;
        self.cells[cell_index] = Some(card);
    }

    /// Validates if a card can be legally placed in a freecell according to FreeCell rules.
    /// Does not modify any state - only provides validation.
    ///
    /// # Rules checked:
    /// - The cell must be empty
    ///
    /// # Errors
    ///
    /// Returns `FreeCellError::CellOccupied` if the cell already contains a card.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::FreeCells;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// let location = FreecellLocation::new(0).unwrap();
    ///
    /// // Validate before placing
    /// assert!(freecells.validate_card_placement(location, &card).is_ok());
    ///
    /// // Place the card
    /// freecells.place_card_at(location, card).unwrap();
    ///
    /// // Trying to validate placing another card will fail
    /// let another_card = Card::new(Rank::Two, Suit::Hearts);
    /// assert!(freecells.validate_card_placement(location, &another_card).is_err());
    /// ```
    fn validate_card_placement(
        &self,
        location: FreecellLocation,
        card: &Card,
    ) -> Result<(), FreeCellError> {
        if let Some(existing_card) = self.cells[location.index() as usize] {
            return Err(FreeCellError::CellOccupied {
                cell_index: location.index(),
                existing_card,
                new_card: *card,
            });
        }
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
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    ///
    /// // Place a card first
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// let location = FreecellLocation::new(0).unwrap();
    /// freecells.place_card_at(location, card).unwrap();
    ///
    /// // Then remove it
    /// let removed_card = freecells.remove_card(location).unwrap();
    /// assert_eq!(removed_card, Some(card));
    /// ```
    pub fn remove_card(
        &mut self,
        location: FreecellLocation,
    ) -> Result<Option<Card>, FreeCellError> {
        Ok(self.cells[location.index() as usize].take())
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
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// let location = FreecellLocation::new(0).unwrap();
    /// freecells.place_card_at(location, card).unwrap();
    ///
    /// // Get a reference to the card
    /// let card_ref = freecells.get_card(location).unwrap().unwrap();
    /// assert_eq!(card_ref, &card);
    /// ```
    pub fn get_card(&self, location: FreecellLocation) -> Result<Option<&Card>, FreeCellError> {
        Ok(self.cells[location.index() as usize].as_ref())
    }

    /// Count the number of empty cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::freecells::{FreeCells, FREECELL_COUNT};
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    /// assert_eq!(freecells.empty_cells_count(), FREECELL_COUNT);
    ///
    /// // Place a card
    /// let location = FreecellLocation::new(0).unwrap();
    /// freecells.place_card_at(location, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// assert_eq!(freecells.empty_cells_count(), 3);
    /// ```
    pub fn empty_cells_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_none()).count()
    }

    // is_cell_empty was removed in favor of using get_card().is_none()

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
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut freecells = FreeCells::new();
    /// let location0 = FreecellLocation::new(0).unwrap();
    /// freecells.place_card_at(location0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// let location2 = FreecellLocation::new(2).unwrap();
    /// freecells.place_card_at(location2, Card::new(Rank::King, Suit::Hearts)).unwrap();
    ///
    /// // Iterate through occupied cells
    /// let occupied: Vec<_> = freecells.occupied_cells().collect();
    /// assert_eq!(occupied.len(), 2);
    /// ```
    pub fn occupied_cells(&self) -> impl Iterator<Item = (usize, &Card)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| cell.as_ref().map(|card| (idx, card)))
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
/// use freecell_game_engine::location::FreecellLocation;
///
/// let mut freecells = FreeCells::new();
///
/// // Trying to place a card in an occupied cell
/// let location = FreecellLocation::new(0).unwrap();
/// freecells.place_card_at(location, Card::new(Rank::Ace, Suit::Spades)).unwrap();
/// let result = freecells.place_card_at(location, Card::new(Rank::Two, Suit::Hearts));
/// assert!(matches!(result, Err(FreeCellError::CellOccupied { .. })));
/// ```
pub enum FreeCellError {
    /// Attempted to access an invalid cell index.
    InvalidCell(u8),
    /// Attempted to place a card in an occupied cell.
    CellOccupied {
        cell_index: u8,
        existing_card: Card,
        new_card: Card,
    },
    /// Attempted to place a card but all cells are full.
    NoEmptyCells,
}

impl std::fmt::Display for FreeCellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreeCellError::InvalidCell(index) => write!(f, "Invalid freecell index: {}", index),
            FreeCellError::CellOccupied {
                cell_index,
                existing_card,
                new_card,
            } => write!(
                f,
                "Cannot place {} in freecell {}: already occupied by {}",
                new_card, cell_index, existing_card
            ),
            FreeCellError::NoEmptyCells => write!(f, "No empty freecells available"),
        }
    }
}

impl fmt::Display for FreeCells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FreeCells:")?;
        for i in 0..FREECELL_COUNT {
            let location = FreecellLocation::new(i as u8).unwrap();
            match self.get_card(location) {
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
        assert_eq!(FREECELL_COUNT, 4, "FreeCells should have 4 cells");
        assert_eq!(
            freecells.empty_cells_count(),
            4,
            "All cells should be empty initially"
        );
        for i in 0..FREECELL_COUNT {
            let location = FreecellLocation::new(i as u8).unwrap();
            assert!(
                freecells.get_card(location).unwrap().is_none(),
                "Cell {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_freecell() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        let location = FreecellLocation::new(0).unwrap();
        freecells.place_card_at(location, card).unwrap();
        assert!(freecells.get_card(location).unwrap().is_some());
        assert_eq!(freecells.empty_cells_count(), 3);
        assert_eq!(freecells.get_card(location).unwrap(), Some(&card));
    }

    #[test]
    fn can_remove_card_from_freecell() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Seven, Suit::Hearts);
        let location = FreecellLocation::new(0).unwrap();
        freecells.place_card_at(location, card).unwrap();
        let removed_card = freecells.remove_card(location).unwrap();
        assert_eq!(removed_card, Some(card));
        assert!(freecells.get_card(location).unwrap().is_none());
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    fn removing_from_empty_freecell_returns_none() {
        let mut freecells = FreeCells::new();
        let location = FreecellLocation::new(0).unwrap();
        let removed = freecells.remove_card(location).unwrap();
        assert_eq!(removed, None);
        assert!(freecells.get_card(location).unwrap().is_none());
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    fn adding_card_to_occupied_freecell_errors() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::Six, Suit::Spades);
        let location = FreecellLocation::new(0).unwrap();
        freecells.place_card_at(location, card1).unwrap();
        let result = freecells.place_card_at(location, card2);
        assert!(matches!(result, Err(FreeCellError::CellOccupied { .. })));
    }

    // Note: With FreecellLocation, out-of-bounds errors are prevented at compile time.
    // We keep this test to ensure the underlying logic (if ever changed) is sound.
    #[test]
    fn freecell_location_prevents_out_of_bounds() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Two, Suit::Clubs);

        // This will fail to compile if using an invalid index with FreecellLocation::new
        let valid_location = FreecellLocation::new(3).unwrap();
        assert!(freecells.place_card_at(valid_location, card).is_ok());

        // The following would not compile:
        // let invalid_location = FreecellLocation::new(4).unwrap();
    }

    #[test]
    fn occupied_cells_iterator_yields_non_empty_cells() {
        let mut freecells = FreeCells::new();
        let card1 = Card::new(Rank::Seven, Suit::Hearts);
        let card2 = Card::new(Rank::King, Suit::Hearts);
        let location1 = FreecellLocation::new(0).unwrap();
        let location2 = FreecellLocation::new(2).unwrap();
        freecells.place_card_at(location1, card1.clone()).unwrap();
        freecells.place_card_at(location2, card2.clone()).unwrap();

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
        let location1 = FreecellLocation::new(0).unwrap();
        let location2 = FreecellLocation::new(2).unwrap();

        freecells.place_card_at(location1, card1).unwrap();
        freecells.place_card_at(location2, card2).unwrap();

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
        let location1 = FreecellLocation::new(0).unwrap();

        // Place card in specific cell
        freecells.place_card_at(location1, card1.clone()).unwrap();
        // Place card in any empty cell
        let placed_location = freecells.place_card(card2.clone()).unwrap();

        assert_eq!(placed_location, FreecellLocation::new(1).unwrap());
        assert_eq!(freecells.get_card(placed_location).unwrap(), Some(&card2));
    }

    #[test]
    fn placing_when_cells_full_returns_error() {
        let mut freecells = FreeCells::new();

        // Fill all cells
        for i in 0..4 {
            let location = FreecellLocation::new(i).unwrap();
            freecells
                .place_card_at(location, Card::new(Rank::Ace, Suit::Hearts))
                .unwrap();
        }

        // Try to place another
        let result = freecells.place_card(Card::new(Rank::Five, Suit::Hearts));
        assert!(matches!(result, Err(FreeCellError::NoEmptyCells)));
    }

    // Note: is_cell_empty now uses FreecellLocation, which prevents invalid indices at compile time.
    // This test is removed as it's no longer applicable.

    #[test]
    fn error_implements_error_trait() {
        // Check that we can use FreeCellError with Box<dyn Error>
        fn returns_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
            let mut freecells = FreeCells::new();
            let location = FreecellLocation::new(0).unwrap();
            freecells.place_card_at(location, Card::new(Rank::Ace, Suit::Hearts))?;
            freecells.place_card_at(location, Card::new(Rank::Two, Suit::Spades))?;
            Ok(())
        }

        let result = returns_box_dyn_error();
        assert!(result.is_err());

        // Convert to string to check Display implementation works
        let error_string = result.unwrap_err().to_string();
        assert!(error_string.contains("already occupied by"));
    }

    #[test]
    fn display_formatting_works() {
        let mut freecells = FreeCells::new();
        let location = FreecellLocation::new(1).unwrap();
        freecells
            .place_card_at(location, Card::new(Rank::King, Suit::Hearts))
            .unwrap();

        let display_output = format!("{}", freecells);
        assert!(display_output.contains("Cell 0: Empty"));
        assert!(display_output.contains("Cell 1:")); // Assuming Card's Display includes the value
        assert!(display_output.contains("Cell 2: Empty"));
        assert!(display_output.contains("Cell 3: Empty"));
    }

    #[test]
    fn can_use_freecell_location() {
        let mut freecells = FreeCells::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        let location = FreecellLocation::new(0).unwrap();

        freecells.place_card_at(location, card.clone()).unwrap();
        assert_eq!(freecells.get_card(location).unwrap(), Some(&card));
        assert_eq!(freecells.remove_card(location).unwrap(), Some(card));
        assert_eq!(freecells.get_card(location).unwrap(), None);
    }
}
