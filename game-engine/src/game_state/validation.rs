//! Move validation logic for GameState.
//!
//! This module provides validation functionality for different moves in a FreeCell game.
//! All validation methods return a Result indicating whether the move is valid.

use super::{GameState, GameError};
use crate::action::Action;
use crate::rules::Rules;

impl GameState {
    /// Validates a move without executing it.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to validate
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is valid
    /// * `Err(GameError)` with a specific error if the move is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// # use freecell_game_engine::game_state::GameState;
    /// # use freecell_game_engine::action::Action;
    /// #
    /// # let game = GameState::new();
    /// # let action = Action::TableauToTableau { from_column: 0, to_column: 1, card_count: 1 };
    /// #
    /// if let Err(e) = game.is_move_valid(&action) {
    ///     println!("Invalid move: {:?}", e);
    /// }
    /// ```
    pub fn is_move_valid(&self, action: &Action) -> Result<(), GameError> {
        use Action::*;
        match action {
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
    ///
    /// # Arguments
    ///
    /// * `from_column` - The index of the tableau column to take the card from
    /// * `to_pile` - The index of the foundation pile to place the card on
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal
    /// * `Err(GameError)` with a specific error if the move is invalid
    fn validate_tableau_to_foundation(&self, from_column: usize, to_pile: usize) -> Result<(), GameError> {
        // Get the top card from the tableau column
        let card_result = self.tableau().get_card(from_column);
        let card = match card_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::EmptySource),
            Err(err) => return Err(err.into()),
        };

        // Get the top card from the foundation pile
        let foundation_result = self.foundations().get_card(to_pile);
        let foundation_top = match foundation_result {
            Ok(card_opt) => card_opt,
            Err(err) => return Err(err.into()),
        };

        // Check if the card can be placed on the foundation pile
        if Rules::can_move_to_foundation(card, foundation_top) {
            Ok(())
        } else {
            Err(GameError::InvalidMove("Cannot move card to foundation".to_string()))
        }
    }

    /// Validates a Tableau-to-Freecell move.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The index of the tableau column to take the card from
    /// * `to_cell` - The index of the freecell to place the card into
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal
    /// * `Err(GameError)` with a specific error if the move is invalid
    fn validate_tableau_to_freecell(&self, from_column: usize, to_cell: usize) -> Result<(), GameError> {
        // Verify that the source tableau column has a card to move
        let card_result = self.tableau().get_card(from_column);
        match card_result {
            Ok(Some(_)) => {}, // Card exists
            Ok(None) => return Err(GameError::EmptySource),
            Err(err) => return Err(err.into()),
        };

        // Check if the target freecell is empty
        let freecell_result = self.freecells().get_card(to_cell);
        match freecell_result {
            Ok(Some(_)) => Err(GameError::InvalidMove("Freecell is already occupied".to_string())),
            Ok(None) => Ok(()), // Freecell is empty
            Err(err) => Err(err.into()),
        }
    }

    /// Validates a Freecell-to-Tableau move.
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The index of the freecell to take the card from
    /// * `to_column` - The index of the tableau column to place the card onto
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal
    /// * `Err(GameError)` with a specific error if the move is invalid
    fn validate_freecell_to_tableau(&self, from_cell: usize, to_column: usize) -> Result<(), GameError> {
        // Get the card from the freecell
        let freecell_result = self.freecells().get_card(from_cell);
        let card = match freecell_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::InvalidMove("No card in freecell".to_string())),
            Err(err) => return Err(err.into()),
        };

        // Get the top card from the destination tableau column
        let tableau_result = self.tableau().get_card(to_column);
        let tableau_top = match tableau_result {
            Ok(top) => top,
            Err(err) => return Err(err.into()),
        };

        // Check stacking rules
        match tableau_top {
            Some(top) => {
                if Rules::can_stack_on_tableau(card, Some(top)) {
                    Ok(())
                } else {
                    Err(GameError::CannotStackOnTableau)
                }
            }
            None => Ok(()), // Empty column, any card can go here
        }
    }

    /// Validates a Freecell-to-Foundation move.
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The index of the freecell to take the card from
    /// * `to_pile` - The index of the foundation pile to place the card onto
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal
    /// * `Err(GameError)` with a specific error if the move is invalid
    fn validate_freecell_to_foundation(&self, from_cell: usize, to_pile: usize) -> Result<(), GameError> {
        // Get the card from the freecell
        let freecell_result = self.freecells().get_card(from_cell);
        let card = match freecell_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::InvalidMove("No card in freecell".to_string())),
            Err(err) => return Err(err.into()),
        };

        // Get the top card from the foundation pile
        let foundation_result = self.foundations().get_card(to_pile);
        let foundation_top = match foundation_result {
            Ok(card_opt) => card_opt,
            Err(err) => return Err(err.into()),
        };

        // Check foundation rules
        if Rules::can_move_to_foundation(card, foundation_top) {
            Ok(())
        } else {
            Err(GameError::InvalidMove("Cannot move card to foundation".to_string()))
        }
    }

    /// Validates a Tableau-to-Tableau move.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The source tableau column index
    /// * `to_column` - The destination tableau column index
    /// * `card_count` - The number of cards to move
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal
    /// * `Err(GameError)` with a specific error if the move is invalid
    fn validate_tableau_to_tableau(&self, from_column: usize, to_column: usize, card_count: usize) -> Result<(), GameError> {
        // Only allow single card moves for now
        if card_count != 1 {
            return Err(GameError::OnlySingleCardMovesSupported);
        }

        // Verify source column has a card
        let source_result = self.tableau().get_card(from_column);
        let card = match source_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::NoCardInTableauColumn),
            Err(err) => return Err(err.into()),
        };

        // Get the top card of the destination column
        let dest_result = self.tableau().get_card(to_column);
        let dest_top = match dest_result {
            Ok(top) => top,
            Err(err) => return Err(err.into()),
        };

        // Check tableau stacking rules
        match dest_top {
            Some(top) => {
                if Rules::can_stack_on_tableau(card, Some(top)) {
                    Ok(())
                } else {
                    Err(GameError::CannotStackOnTableau)
                }
            }
            None => Ok(()), // Empty column, any card can go here
        }
    }
}
