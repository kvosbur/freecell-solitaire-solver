//! GameState struct for FreeCell, combining tableau, freecells, and foundations.

use crate::foundations::Foundations;
use crate::freecells::FreeCells;
use crate::tableau::Tableau;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    TableauToFoundation {
        from_column: usize,
        to_pile: usize,
    },
    TableauToFreecell {
        from_column: usize,
        to_cell: usize,
    },
    FreecellToTableau {
        from_cell: usize,
        to_column: usize,
    },
    FreecellToFoundation {
        from_cell: usize,
        to_pile: usize,
    },
    TableauToTableau {
        from_column: usize,
        to_column: usize,
        card_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    InvalidMove(String),
    IndexOutOfBounds {
        component: &'static str,
        index: usize,
    },
    EmptySource,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    pub tableau: Tableau,
    pub freecells: FreeCells,
    pub foundations: Foundations,
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GameState:")?;
        writeln!(f, "  Tableau:")?;
        for col in 0..self.tableau.column_count() {
            write!(f, "    Column {}: ", col)?;
            let len = self.tableau.column_length(col);
            if len == 0 {
                writeln!(f, "[empty]")?;
            } else {
                for i in 0..len {
                    if let Some(card) = self.tableau.get_card_at(col, i) {
                        write!(f, "{:?} ", card)?;
                    }
                }
                writeln!(f)?;
            }
        }
        writeln!(f, "  FreeCells:")?;
        for cell in 0..self.freecells.cell_count() {
            write!(f, "    Cell {}: ", cell)?;
            match self.freecells.get_card(cell) {
                Some(card) => writeln!(f, "{:?}", card)?,
                None => writeln!(f, "[empty]")?,
            }
        }
        writeln!(f, "  Foundations:")?;
        for pile in 0..self.foundations.pile_count() {
            write!(f, "    Pile {}: ", pile)?;
            match self.foundations.get_top_card(pile) {
                Some(card) => writeln!(f, "top: {:?}", card)?,
                None => writeln!(f, "[empty]")?,
            }
        }
        Ok(())
    }
}

impl GameState {
    /// Returns true if all foundation piles are complete (i.e., game is won).
    pub fn is_game_won(&self) -> bool {
        (0..self.foundations.pile_count()).all(|i| self.foundations.is_pile_complete(i))
    }

    /// Returns all valid single-card moves from the current state.
    pub fn get_available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        // Tableau to Foundation
        for from_col in 0..self.tableau.column_count() {
            for to_pile in 0..self.foundations.pile_count() {
                let m = Move::TableauToFoundation {
                    from_column: from_col,
                    to_pile,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        // Freecell to Foundation
        for from_cell in 0..self.freecells.cell_count() {
            for to_pile in 0..self.foundations.pile_count() {
                let m = Move::FreecellToFoundation { from_cell, to_pile };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        // Freecell to Tableau
        for from_cell in 0..self.freecells.cell_count() {
            for to_col in 0..self.tableau.column_count() {
                let m = Move::FreecellToTableau {
                    from_cell,
                    to_column: to_col,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        // Tableau to Tableau (single card)
        for from_col in 0..self.tableau.column_count() {
            for to_col in 0..self.tableau.column_count() {
                if from_col == to_col {
                    continue;
                }
                let m = Move::TableauToTableau {
                    from_column: from_col,
                    to_column: to_col,
                    card_count: 1,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        // Tableau to Freecell
        for from_col in 0..self.tableau.column_count() {
            for to_cell in 0..self.freecells.cell_count() {
                let m = Move::TableauToFreecell {
                    from_column: from_col,
                    to_cell,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    pub fn new() -> Self {
        Self {
            tableau: Tableau::new(),
            freecells: FreeCells::new(),
            foundations: Foundations::new(),
        }
    }

    /// Executes a move, mutating the game state if valid.
    pub fn execute_move(&mut self, m: &Move) -> Result<(), GameError> {
        use Move::*;
        match m {
            TableauToFoundation {
                from_column,
                to_pile,
            } => {
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or(GameError::EmptySource)?
                    .clone();
                self.is_move_valid(m)
                    .map_err(|e| GameError::InvalidMove(e.to_string()))?;
                self.tableau.remove_card_from_column(*from_column);
                self.foundations.add_card(*to_pile, card);
                Ok(())
            }
            TableauToFreecell {
                from_column,
                to_cell,
            } => {
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or(GameError::EmptySource)?
                    .clone();
                self.is_move_valid(m)
                    .map_err(|e| GameError::InvalidMove(e.to_string()))?;
                self.tableau.remove_card_from_column(*from_column);
                self.freecells.add_card(*to_cell, card);
                Ok(())
            }
            FreecellToTableau {
                from_cell,
                to_column,
            } => {
                let card = self
                    .freecells
                    .get_card(*from_cell)
                    .ok_or(GameError::EmptySource)?
                    .clone();
                self.is_move_valid(m)
                    .map_err(|e| GameError::InvalidMove(e.to_string()))?;
                self.freecells.remove_card(*from_cell);
                self.tableau.add_card_to_column(*to_column, card);
                Ok(())
            }
            FreecellToFoundation { from_cell, to_pile } => {
                let card = self
                    .freecells
                    .get_card(*from_cell)
                    .ok_or(GameError::EmptySource)?
                    .clone();
                self.is_move_valid(m)
                    .map_err(|e| GameError::InvalidMove(e.to_string()))?;
                self.freecells.remove_card(*from_cell);
                self.foundations.add_card(*to_pile, card);
                Ok(())
            }
            TableauToTableau {
                from_column,
                to_column,
                card_count,
            } => {
                if *card_count != 1 {
                    return Err(GameError::InvalidMove(
                        "Only single card moves supported".to_string(),
                    ));
                }
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or(GameError::EmptySource)?
                    .clone();
                self.is_move_valid(m)
                    .map_err(|e| GameError::InvalidMove(e.to_string()))?;
                self.tableau.remove_card_from_column(*from_column);
                self.tableau.add_card_to_column(*to_column, card);
                Ok(())
            }
        }
    }

    /// Validates a move without executing it.
    pub fn is_move_valid(&self, m: &Move) -> Result<(), &'static str> {
        use Move::*;
        match m {
            TableauToFoundation {
                from_column,
                to_pile,
            } => {
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or("No card in tableau column")?;
                let foundation_top = self.foundations.get_top_card(*to_pile);
                if crate::rules::can_move_to_foundation(card, foundation_top) {
                    Ok(())
                } else {
                    Err("Invalid move: cannot move card to foundation")
                }
            }
            TableauToFreecell {
                from_column,
                to_cell,
            } => {
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or("No card in tableau column")?;
                let cell = self.freecells.get_card(*to_cell);
                crate::rules::can_move_to_freecell(card, cell)
            }
            FreecellToTableau {
                from_cell,
                to_column,
            } => {
                let card = self
                    .freecells
                    .get_card(*from_cell)
                    .ok_or("No card in freecell")?;
                let tableau_top = self.tableau.get_top_card(*to_column);
                if let Some(top) = tableau_top {
                    if crate::rules::can_stack_on_tableau(card, top) {
                        Ok(())
                    } else {
                        Err("Invalid move: cannot stack card on tableau")
                    }
                } else {
                    Ok(())
                }
            }
            FreecellToFoundation { from_cell, to_pile } => {
                let card = self
                    .freecells
                    .get_card(*from_cell)
                    .ok_or("No card in freecell")?;
                let foundation_top = self.foundations.get_top_card(*to_pile);
                if crate::rules::can_move_to_foundation(card, foundation_top) {
                    Ok(())
                } else {
                    Err("Invalid move: cannot move card to foundation")
                }
            }
            TableauToTableau {
                from_column,
                to_column,
                card_count,
            } => {
                // Only allow single card moves for now
                if *card_count != 1 {
                    return Err("Only single card moves supported");
                }
                let card = self
                    .tableau
                    .get_top_card(*from_column)
                    .ok_or("No card in tableau column")?;
                let dest_top = self.tableau.get_top_card(*to_column);
                if let Some(top) = dest_top {
                    if crate::rules::can_stack_on_tableau(card, top) {
                        Ok(())
                    } else {
                        Err("Invalid move: cannot stack card on tableau")
                    }
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn game_state_initializes_with_empty_components() {
        let state = GameState::new();
        // Tableau: 8 empty columns
        assert_eq!(state.tableau.column_count(), 8);
        for i in 0..8 {
            assert!(state.tableau.is_column_empty(i));
        }
        // FreeCells: 4 empty cells
        assert_eq!(state.freecells.cell_count(), 4);
        assert_eq!(state.freecells.empty_cell_count(), 4);
        // Foundations: 4 empty piles
        assert_eq!(state.foundations.pile_count(), 4);
        for i in 0..4 {
            assert!(state.foundations.is_pile_empty(i));
        }
    }

    #[test]
    fn can_validate_tableau_to_foundation_move() {
        use crate::card::{Card, Suit};
        let mut state = GameState::new();
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        state.tableau.add_card_to_column(0, card.clone());
        let m = Move::TableauToFoundation {
            from_column: 0,
            to_pile: 0,
        };
        assert!(state.is_move_valid(&m).is_ok());
    }

    #[test]
    fn can_execute_tableau_to_foundation_move() {
        use crate::card::{Card, Suit};
        let mut state = GameState::new();
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        state.tableau.add_card_to_column(0, card.clone());
        let m = Move::TableauToFoundation {
            from_column: 0,
            to_pile: 0,
        };
        assert!(state.execute_move(&m).is_ok());
        assert!(state.tableau.is_column_empty(0));
        assert_eq!(state.foundations.get_top_card(0), Some(&card));
    }

    #[test]
    fn is_game_won_returns_true_when_all_foundations_complete() {
        use crate::card::{Card, Suit};
        let mut state = GameState::new();
        // Fill all foundations with 13 cards of the same suit
        for pile in 0..4 {
            for rank in 1..=13 {
                state.foundations.add_card(
                    pile,
                    Card {
                        rank: Rank::try_from(rank).unwrap(),
                        suit: Suit::Hearts,
                    },
                );
            }
        }
        assert!(state.is_game_won());
    }

    #[test]
    fn get_available_moves_returns_valid_moves() {
        use crate::card::{Card, Suit};
        let mut state = GameState::new();
        // Place an Ace in tableau column 0
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        state.tableau.add_card_to_column(0, card.clone());
        let moves = state.get_available_moves();
        // Should include TableauToFoundation move for column 0, pile 0
        assert!(moves.iter().any(|m| matches!(
            m,
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 0
            }
        )));
    }
}
