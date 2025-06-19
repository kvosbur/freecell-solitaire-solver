//! FreeCells implementation for FreeCell game state.
//!
//! FreeCells are the 4 temporary storage slots where single cards can be placed.

use crate::card::Card;
use crate::rules;

#[derive(Clone, PartialEq, Eq, Hash)]
/// Represents the free cells where individual cards can be stored
pub struct FreeCells {
    cells: Vec<Option<Card>>,
}

impl FreeCells {
    /// Create a new freecells with the specified number of cells
    pub fn new(cell_count: usize) -> Self {
        Self { cells: vec![None; cell_count] }
    }
    
    /// Place a card in a freecell
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
    
    /// Remove and return a card from a freecell
    pub fn remove_card(&mut self, cell_index: usize) -> Option<Card> {
        if cell_index >= self.cells.len() {
            return None;
        }
        self.cells[cell_index].take()
    }
    
    /// Get a reference to a card in a freecell without removing it
    pub fn get_card(&self, cell_index: usize) -> Option<&Card> {
        if cell_index >= self.cells.len() {
            return None;
        }
        self.cells[cell_index].as_ref()
    }
    
    /// Get the number of freecells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
    
    /// Count the number of empty cells
    pub fn empty_cells_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_none()).count()
    }
    
    /// Check if a cell is empty
    pub fn is_cell_empty(&self, cell_index: usize) -> bool {
        cell_index < self.cells.len() && self.cells[cell_index].is_none()
    }
}

use std::fmt;

impl fmt::Debug for FreeCells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("FreeCells");
        for cell in 0..self.cell_count() {
            let cell_name = format!("cell_{}", cell);
            match self.get_card(cell) {
                Some(card) => debug_struct.field(&cell_name, &format!("{:?}", card)),
                None => debug_struct.field(&cell_name, &"[empty]"),
            };
        }
        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{Card, Suit},
        Rank,
    };

    #[test]
    fn freecells_initialize_with_four_empty_cells() {
        let freecells = FreeCells::new(4);
        assert_eq!(freecells.cell_count(), 4, "FreeCells should have 4 cells");
        assert_eq!(
            freecells.empty_cells_count(),
            4,
            "All cells should be empty initially"
        );
        for i in 0..freecells.cell_count() {
            assert!(
                freecells.is_cell_empty(i),
                "Cell {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_freecell() {
        let mut freecells = FreeCells::new(4);
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Hearts,
        };
        freecells.place_card(0, card.clone()).unwrap();
        assert!(!freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cells_count(), 3);
        assert_eq!(freecells.get_card(0), Some(&card));
    }

    #[test]
    fn can_remove_card_from_freecell() {
        let mut freecells = FreeCells::new(4);
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Hearts,
        };
        freecells.place_card(0, card.clone()).unwrap();
        let removed_card = freecells.remove_card(0);
        assert_eq!(removed_card, Some(card));
        assert!(freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    fn removing_from_empty_freecell_returns_none() {
        let mut freecells = FreeCells::new(4);
        let removed = freecells.remove_card(0);
        assert_eq!(removed, None);
        assert!(freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cells_count(), 4);
    }

    #[test]
    #[should_panic(expected = "CellOccupied")]
    fn adding_card_to_occupied_freecell_panics() {
        let mut freecells = FreeCells::new(4);
        let card1 = Card {
            rank: Rank::Seven,
            suit: Suit::Hearts,
        };
        let card2 = Card {
            rank: Rank::Six,
            suit: Suit::Spades,
        };
        freecells.place_card(0, card1).unwrap();
        freecells.place_card(0, card2); // Should panic
    }

    #[test]
    fn freecell_index_out_of_bounds_panics() {
        // Each closure must own its own FreeCells to be UnwindSafe
        let result = std::panic::catch_unwind(|| {
            let mut freecells = FreeCells::new(4);
            freecells.place_card(
                4,
                Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
            );
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let mut freecells = FreeCells::new(4);
            freecells.remove_card(4);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let freecells = FreeCells::new(4);
            freecells.get_card(4);
        });
        assert!(result.is_err());
    }
}
