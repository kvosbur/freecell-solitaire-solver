//! Move validation logic for GameState.

use super::{GameState, GameError};
use crate::action::Action;

impl GameState {
    /// Validates a move without executing it.
    pub fn is_move_valid(&self, m: &Action) -> Result<(), GameError> {
        use Action::*;
        match m {
            TableauToFoundation { from_column, to_pile } => {
                self.validate_tableau_to_foundation(*from_column, *to_pile)
            }
            TableauToFreecell { from_column, to_cell } => {
                self.validate_tableau_to_freecell(*from_column, *to_cell)
            }
            FreecellToTableau { from_cell, to_column } => {
                self.validate_freecell_to_tableau(*from_cell, *to_column)
            }
            FreecellToFoundation { from_cell, to_pile } => {
                self.validate_freecell_to_foundation(*from_cell, *to_pile)
            }
            TableauToTableau { from_column, to_column, card_count } => {
                self.validate_tableau_to_tableau(*from_column, *to_column, *card_count)
            }
        }
    }

    /// Validates a Tableau-to-Foundation move.
    /// Returns Ok if the move is legal, or an appropriate GameError otherwise.
    fn validate_tableau_to_foundation(&self, from_column: usize, to_pile: usize) -> Result<(), GameError> {
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::NoCardInTableauColumn)?;
        let foundation_top = self.foundations.get_top_card(to_pile);
        if crate::rules::can_move_to_foundation(card, foundation_top) {
            Ok(())
        } else {
            Err(GameError::CannotMoveToFoundation)
        }
    }

    /// Validates a Tableau-to-Freecell move.
    /// Returns Ok if the move is legal, or an appropriate GameError otherwise.
    fn validate_tableau_to_freecell(&self, from_column: usize, to_cell: usize) -> Result<(), GameError> {
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::NoCardInTableauColumn)?;
        let cell = self.freecells.get_card(to_cell);
        crate::rules::can_move_to_freecell(card, cell)
            .map_err(|_| GameError::CannotMoveToFreecell)
    }

    /// Validates a Freecell-to-Tableau move.
    /// Returns Ok if the move is legal, or an appropriate GameError otherwise.
    fn validate_freecell_to_tableau(&self, from_cell: usize, to_column: usize) -> Result<(), GameError> {
        let card = self
            .freecells
            .get_card(from_cell)
            .ok_or(GameError::NoCardInFreecell)?;
        let tableau_top = self.tableau.get_top_card(to_column);
        if let Some(top) = tableau_top {
            if crate::rules::can_stack_on_tableau(card, top) {
                Ok(())
            } else {
                Err(GameError::CannotStackOnTableau)
            }
        } else {
            Ok(())
        }
    }

    /// Validates a Freecell-to-Foundation move.
    /// Returns Ok if the move is legal, or an appropriate GameError otherwise.
    fn validate_freecell_to_foundation(&self, from_cell: usize, to_pile: usize) -> Result<(), GameError> {
        let card = self
            .freecells
            .get_card(from_cell)
            .ok_or(GameError::NoCardInFreecell)?;
        let foundation_top = self.foundations.get_top_card(to_pile);
        if crate::rules::can_move_to_foundation(card, foundation_top) {
            Ok(())
        } else {
            Err(GameError::CannotMoveToFoundation)
        }
    }

    /// Validates a Tableau-to-Tableau move.
    /// Returns Ok if the move is legal, or an appropriate GameError otherwise.
    fn validate_tableau_to_tableau(&self, from_column: usize, to_column: usize, card_count: usize) -> Result<(), GameError> {
        // Only allow single card moves for now
        if card_count != 1 {
            return Err(GameError::OnlySingleCardMovesSupported);
        }
        let card = self
            .tableau
            .get_top_card(from_column)
            .ok_or(GameError::NoCardInTableauColumn)?;
        let dest_top = self.tableau.get_top_card(to_column);
        if let Some(top) = dest_top {
            if crate::rules::can_stack_on_tableau(card, top) {
                Ok(())
            } else {
                Err(GameError::CannotStackOnTableau)
            }
        } else {
            Ok(())
        }
    }
}
