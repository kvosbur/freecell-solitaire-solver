//! Move execution and undo logic for GameState.

use super::{GameState, GameError};
use crate::action::Action;

impl GameState {
    /// Executes a move, mutating the game state if valid.
    pub fn execute_move(&mut self, m: &Action) -> Result<(), GameError> {
        use Action::*;
        match m {
            TableauToFoundation { from_column, to_pile } => {
                self.execute_tableau_to_foundation(*from_column, *to_pile, m)
            }
            TableauToFreecell { from_column, to_cell } => {
                self.execute_tableau_to_freecell(*from_column, *to_cell, m)
            }
            FreecellToTableau { from_cell, to_column } => {
                self.execute_freecell_to_tableau(*from_cell, *to_column, m)
            }
            FreecellToFoundation { from_cell, to_pile } => {
                self.execute_freecell_to_foundation(*from_cell, *to_pile, m)
            }
            TableauToTableau { from_column, to_column, card_count } => {
                self.execute_tableau_to_tableau(*from_column, *to_column, *card_count, m)
            }
        }
    }

    fn execute_tableau_to_foundation(
        &mut self,
        from_column: usize,
        to_pile: usize,
        m: &Action,
    ) -> Result<(), GameError> {
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        self.tableau.remove_card_from_column(from_column);
        self.foundations.add_card(to_pile, card);
        Ok(())
    }

    fn execute_tableau_to_freecell(
        &mut self,
        from_column: usize,
        to_cell: usize,
        m: &Action,
    ) -> Result<(), GameError> {
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        self.tableau.remove_card_from_column(from_column);
        self.freecells.add_card(to_cell, card);
        Ok(())
    }

    fn execute_freecell_to_tableau(
        &mut self,
        from_cell: usize,
        to_column: usize,
        m: &Action,
    ) -> Result<(), GameError> {
        let card = self
            .freecells
            .get_card(from_cell)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        self.freecells.remove_card(from_cell);
        self.tableau.add_card_to_column(to_column, card);
        Ok(())
    }

    fn execute_freecell_to_foundation(
        &mut self,
        from_cell: usize,
        to_pile: usize,
        m: &Action,
    ) -> Result<(), GameError> {
        let card = self
            .freecells
            .get_card(from_cell)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        self.freecells.remove_card(from_cell);
        self.foundations.add_card(to_pile, card);
        Ok(())
    }

    fn execute_tableau_to_tableau(
        &mut self,
        from_column: usize,
        to_column: usize,
        card_count: usize,
        m: &Action,
    ) -> Result<(), GameError> {
        if card_count != 1 {
            return Err(GameError::InvalidMove(
                "Only single card moves supported".to_string(),
            ));
        }
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        self.tableau.remove_card_from_column(from_column);
        self.tableau.add_card_to_column(to_column, card);
        Ok(())
    }

    /// Undoes a move, reversing its effect on the game state.
    pub fn undo_move(&mut self, m: &Action) {
        use Action::*;
        match m {
            TableauToFoundation {
                from_column,
                to_pile,
            } => {
                let card = self
                    .foundations
                    .remove_top_card(*to_pile)
                    .expect("Undo: foundation not empty");
                self.tableau.initial_addition_of_card(*from_column, card);
            }
            TableauToFreecell {
                from_column,
                to_cell,
            } => {
                let card = self
                    .freecells
                    .remove_card(*to_cell)
                    .expect("Undo: freecell not empty");
                self.tableau.initial_addition_of_card(*from_column, card);
            }
            FreecellToTableau {
                from_cell,
                to_column,
            } => {
                let card = self
                    .tableau
                    .remove_top_card(*to_column)
                    .expect("Undo: tableau not empty");
                self.freecells.add_card(*from_cell, card);
            }
            FreecellToFoundation { from_cell, to_pile } => {
                let card = self
                    .foundations
                    .remove_top_card(*to_pile)
                    .expect("Undo: foundation not empty");
                self.freecells.add_card(*from_cell, card);
            }
            TableauToTableau {
                from_column,
                to_column,
                card_count,
            } => {
                assert_eq!(*card_count, 1, "Undo only supports single card moves");
                let card = self
                    .tableau
                    .remove_top_card(*to_column)
                    .expect("Undo: tableau not empty");
                self.tableau.initial_addition_of_card(*from_column, card);
            }
        }
    }
}
