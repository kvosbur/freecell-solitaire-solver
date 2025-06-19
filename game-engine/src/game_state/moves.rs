//! Move generation logic for GameState.

use super::GameState;
use crate::action::Action;

impl GameState {
    /// Returns all valid single-card moves from the current state.
    pub fn get_available_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        moves.extend(self.get_tableau_to_foundation_moves());
        moves.extend(self.get_freecell_to_foundation_moves());
        moves.extend(self.get_freecell_to_tableau_moves());
        moves.extend(self.get_tableau_to_tableau_moves());
        moves.extend(self.get_tableau_to_freecell_moves());
        moves
    }

    fn get_tableau_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        for from_col in 0..self.tableau.column_count() {
            for to_pile in 0..self.foundations.pile_count() {
                let m = Action::TableauToFoundation {
                    from_column: from_col,
                    to_pile,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn get_freecell_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        for from_cell in 0..self.freecells.cell_count() {
            for to_pile in 0..self.foundations.pile_count() {
                let m = Action::FreecellToFoundation { from_cell, to_pile };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn get_freecell_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        for from_cell in 0..self.freecells.cell_count() {
            for to_col in 0..self.tableau.column_count() {
                let m = Action::FreecellToTableau {
                    from_cell,
                    to_column: to_col,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn get_tableau_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        for from_col in 0..self.tableau.column_count() {
            for to_col in 0..self.tableau.column_count() {
                if from_col == to_col {
                    continue;
                }
                let m = Action::TableauToTableau {
                    from_column: from_col,
                    to_column: to_col,
                    card_count: 1,
                };
                if self.is_move_valid(&m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn get_tableau_to_freecell_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        for from_col in 0..self.tableau.column_count() {
            for to_cell in 0..self.freecells.cell_count() {
                let m = Action::TableauToFreecell {
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
}
