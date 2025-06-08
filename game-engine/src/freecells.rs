//! FreeCells implementation for FreeCell game state.
//! 
//! FreeCells are the 4 temporary storage slots where single cards can be placed.

use crate::card::Card;
use crate::rules;

pub struct FreeCells {
    cells: [Option<Card>; 4],
}

impl FreeCells {
    pub fn new() -> Self {
        Self {
            cells: [None, None, None, None],
        }
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn is_cell_empty(&self, index: usize) -> bool {
        self.cells[index].is_none()
    }

    pub fn empty_cell_count(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_none()).count()
    }

    pub fn add_card(&mut self, index: usize, card: Card) {
        if let Err(msg) = rules::can_move_to_freecell(&card, self.cells[index].as_ref()) {
            panic!("{}", msg);
        }
        self.cells[index] = Some(card);
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cells[index].as_ref()
    }

    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        self.cells[index].take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit};

    #[test]
    fn freecells_initialize_with_four_empty_cells() {
        let freecells = FreeCells::new();
        assert_eq!(freecells.cell_count(), 4, "FreeCells should have 4 cells");
        assert_eq!(freecells.empty_cell_count(), 4, "All cells should be empty initially");
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
        let mut freecells = FreeCells::new();
        let card = Card { rank: 7, suit: Suit::Hearts };
        freecells.add_card(0, card.clone());
        assert!(!freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cell_count(), 3);
        assert_eq!(freecells.get_card(0), Some(&card));
    }

    #[test]
    fn can_remove_card_from_freecell() {
        let mut freecells = FreeCells::new();
        let card = Card { rank: 7, suit: Suit::Hearts };
        freecells.add_card(0, card.clone());
        let removed_card = freecells.remove_card(0);
        assert_eq!(removed_card, Some(card));
        assert!(freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cell_count(), 4);
    }

    #[test]
    fn removing_from_empty_freecell_returns_none() {
        let mut freecells = FreeCells::new();
        let removed = freecells.remove_card(0);
        assert_eq!(removed, None);
        assert!(freecells.is_cell_empty(0));
        assert_eq!(freecells.empty_cell_count(), 4);
    }

    #[test]
    #[should_panic(expected = "Cell is already occupied")]
    fn adding_card_to_occupied_freecell_panics() {
        let mut freecells = FreeCells::new();
        let card1 = Card { rank: 7, suit: Suit::Hearts };
        let card2 = Card { rank: 6, suit: Suit::Spades };
        freecells.add_card(0, card1);
        freecells.add_card(0, card2); // Should panic
    }

    #[test]
    fn freecell_index_out_of_bounds_panics() {
        // Each closure must own its own FreeCells to be UnwindSafe
        let result = std::panic::catch_unwind(|| {
            let mut freecells = FreeCells::new();
            freecells.add_card(4, Card { rank: 2, suit: Suit::Clubs });
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let mut freecells = FreeCells::new();
            freecells.remove_card(4);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let freecells = FreeCells::new();
            freecells.get_card(4);
        });
        assert!(result.is_err());
    }
}
