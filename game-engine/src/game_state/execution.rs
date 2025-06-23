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
            .get_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.foundations.place_card(to_pile, removed_card)?;
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
            .get_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.freecells.place_card(to_cell, removed_card)?;
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
        let removed = self.freecells.remove_card(from_cell)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.tableau.place_card(to_column, removed_card)?;
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
        let removed = self.freecells.remove_card(from_cell)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.foundations.place_card(to_pile, removed_card)?;
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
            .get_card(from_column)
            .ok_or(GameError::EmptySource)?
            .clone();
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.tableau.place_card(to_column, removed_card)?;
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
                let removed = self.foundations.remove_card(*to_pile).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                self.tableau.place_card(*from_column, card).expect("Undo: tableau error");
            }
            TableauToFreecell {
                from_column,
                to_cell,
            } => {
                let removed = self.freecells.remove_card(*to_cell).expect("Undo: freecell error");
                let card = removed.expect("Undo: freecell not empty");
                self.tableau.place_card(*from_column, card).expect("Undo: tableau error");
            }
            FreecellToTableau {
                from_cell,
                to_column,
            } => {
                let removed = self.tableau.remove_card(*to_column).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                self.freecells.place_card(*from_cell, card).expect("Undo: freecell error");
            }
            FreecellToFoundation { from_cell, to_pile } => {
                let removed = self.foundations.remove_card(*to_pile).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                self.freecells.place_card(*from_cell, card).expect("Undo: freecell error");
            }
            TableauToTableau {
                from_column,
                to_column,
                card_count,
            } => {
                assert_eq!(*card_count, 1, "Undo only supports single card moves");
                let removed = self.tableau.remove_card(*to_column).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                self.tableau.place_card(*from_column, card).expect("Undo: tableau error");
            }
        }
    }
}
