//! Move validation logic for GameState.
//!
//! This module provides validation functionality for different moves in a FreeCell game.
//! All validation methods return a Result indicating whether the move is valid.

use super::{GameState, GameError};
use crate::r#move::Move;
impl GameState {
    /// Validates a move without executing it.
    ///
    /// This method serves as the primary entry point for checking the legality
    /// of any `Move` within the current `GameState`. It delegates to specific
    /// validation helper functions based on the type of move.
    ///
    /// # Arguments
    ///
    /// * `m` - A reference to the `Move` to validate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is valid according to FreeCell rules.
    /// * `Err(GameError)` with a specific error if the move is invalid,
    ///   providing details on why the move cannot be made.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move};
    /// use freecell_game_engine::game_state::GameError;
    ///
    /// let game = GameState::new(); // Represents a new, shuffled game
    ///
    /// // Example of a potentially valid move (depending on game state)
    /// let valid_move = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
    /// if game.is_move_valid(&valid_move).is_ok() {
    ///     println!("Move is valid!");
    /// }
    ///
    /// // Example of an invalid move (e.g., moving from an empty source)
    /// let invalid_move = Move::TableauToFoundation { from_column: 7, to_pile: 0 };
    /// let result = game.is_move_valid(&invalid_move);
    /// assert!(result.is_err());
    /// if let Err(GameError::EmptySource) = result {
    ///     println!("Caught expected error: Cannot move from empty source.");
    /// }
    /// ```
    pub fn is_move_valid(&self, m: &Move) -> Result<(), GameError> {
        use Move::*;
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

    /// Validates a move from a tableau column to a foundation pile.
    ///
    /// Checks if the top card of `from_column` can be legally placed on `to_pile`.
    /// This involves verifying the source column is not empty and the card
    /// adheres to foundation stacking rules (same suit, ascending rank).
    ///
    /// # Arguments
    ///
    /// * `from_column` - The 0-indexed tableau column from which to move the card.
    /// * `to_pile` - The 0-indexed foundation pile to which to move the card.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal.
    /// * `Err(GameError)` if the move is invalid (e.g., empty source, invalid card placement).
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    /// use freecell_game_engine::game_state::GameError;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
    ///
    /// let result = game.validate_tableau_to_foundation(0, 0);
    /// // assert!(result.is_ok() || matches!(result, Err(GameError::InvalidMove(_))));
    /// ```
    pub fn validate_tableau_to_foundation(&self, from_column: u8, to_pile: u8) -> Result<(), GameError> {
        // Get the top card from the tableau column
        let card_result = self.tableau().get_card(from_column as usize);
        let card = match card_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::EmptySource),
            Err(err) => return Err(err.into()),
        };

        // Check if the card can be placed on the foundation pile
        self.foundations().validate_card_placement(to_pile as usize, card)
            .map_err(|e| GameError::InvalidMove(format!("Cannot move card to foundation: {}", e)))
    }

    /// Validates a move from a tableau column to a freecell.
    ///
    /// Checks if the top card of `from_column` can be legally moved to `to_cell`.
    /// This involves verifying the source column is not empty and the destination
    /// freecell is empty.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The 0-indexed tableau column from which to move the card.
    /// * `to_cell` - The 0-indexed freecell to which to move the card.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal.
    /// * `Err(GameError)` if the move is invalid (e.g., empty source, occupied freecell).
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    /// use freecell_game_engine::game_state::GameError;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move
    /// // game.tableau_mut().place_card(0, Card::new(Rank::King, Suit::Spades)).unwrap();
    ///
    /// let result = game.validate_tableau_to_freecell(0, 0);
    /// // assert!(result.is_ok() || matches!(result, Err(GameError::InvalidMove(_))));
    /// ```
    pub fn validate_tableau_to_freecell(&self, from_column: u8, to_cell: u8) -> Result<(), GameError> {
        // Verify that the source tableau column has a card to move
        let card_result = self.tableau().get_card(from_column as usize);
        match card_result {
            Ok(Some(_)) => {}, // Card exists
            Ok(None) => return Err(GameError::EmptySource),
            Err(err) => return Err(err.into()),
        };

        // Check if the target freecell is empty
        let freecell_result = self.freecells().get_card(to_cell as usize);
        match freecell_result {
            Ok(Some(_)) => Err(GameError::InvalidMove("Freecell is already occupied".to_string())),
            Ok(None) => Ok(()), // Freecell is empty
            Err(err) => Err(err.into()),
        }
    }

    /// Validates a move from a freecell to a tableau column.
    ///
    /// Checks if the card in `from_cell` can be legally placed on `to_column`.
    /// This involves verifying the source freecell is not empty and the card
    /// adheres to tableau stacking rules (alternating colors, descending rank).
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The 0-indexed freecell from which to move the card.
    /// * `to_column` - The 0-indexed tableau column to which to move the card.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal.
    /// * `Err(GameError)` if the move is invalid (e.g., empty freecell, invalid card placement).
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    /// use freecell_game_engine::game_state::GameError;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move
    /// // game.freecells_mut().place_card(0, Card::new(Rank::King, Suit::Spades)).unwrap();
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Queen, Suit::Hearts)).unwrap();
    ///
    /// let result = game.validate_freecell_to_tableau(0, 0);
    /// // assert!(result.is_ok() || matches!(result, Err(GameError::InvalidMove(_))));
    /// ```
    pub fn validate_freecell_to_tableau(&self, from_cell: u8, to_column: u8) -> Result<(), GameError> {
        // Get the card from the freecell
        let freecell_result = self.freecells().get_card(from_cell as usize);
        let card = match freecell_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::InvalidMove("No card in freecell".to_string())),
            Err(err) => return Err(err.into()),
        };

        // Check stacking rules
        self.tableau().validate_card_placement(to_column as usize, card)
            .map_err(|e| GameError::InvalidMove(format!("Cannot stack on tableau: {}", e)))
    }

    /// Validates a move from a freecell to a foundation pile.
    ///
    /// Checks if the card in `from_cell` can be legally placed on `to_pile`.
    /// This involves verifying the source freecell is not empty and the card
    /// adheres to foundation stacking rules.
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The 0-indexed freecell from which to move the card.
    /// * `to_pile` - The 0-indexed foundation pile to which to move the card.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is legal.
    /// * `Err(GameError)` if the move is invalid (e.g., empty freecell, invalid card placement).
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    /// use freecell_game_engine::game_state::GameError;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move
    /// // game.freecells_mut().place_card(0, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    ///
    /// let result = game.validate_freecell_to_foundation(0, 0);
    /// // assert!(result.is_ok() || matches!(result, Err(GameError::InvalidMove(_))));
    /// ```
    pub fn validate_freecell_to_foundation(&self, from_cell: u8, to_pile: u8) -> Result<(), GameError> {
        // Get the card from the freecell
        let freecell_result = self.freecells().get_card(from_cell as usize);
        let card = match freecell_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::InvalidMove("No card in freecell".to_string())),
            Err(err) => return Err(err.into()),
        };

        // Check foundation rules
        self.foundations().validate_card_placement(to_pile as usize, card)
            .map_err(|e| GameError::InvalidMove(format!("Cannot move card to foundation: {}", e)))
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
    pub fn validate_tableau_to_tableau(&self, from_column: u8, to_column: u8, card_count: u8) -> Result<(), GameError> {
        // Only allow single card moves for now
        if card_count != 1 {
            return Err(GameError::OnlySingleCardMovesSupported);
        }

        // Verify source column has a card
        let source_result = self.tableau().get_card(from_column as usize);
        let card = match source_result {
            Ok(Some(card)) => card,
            Ok(None) => return Err(GameError::NoCardInTableauColumn),
            Err(err) => return Err(err.into()),
        };

        // Check tableau stacking rules
        self.tableau().validate_card_placement(to_column as usize, card)
            .map_err(|e| GameError::InvalidMove(format!("Cannot stack on tableau: {}", e)))
    }
}
