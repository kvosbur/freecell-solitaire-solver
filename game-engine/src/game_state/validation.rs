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
    /// use freecell_game_engine::{GameState, Move};
    /// use freecell_game_engine::game_state::GameError;
    /// use freecell_game_engine::location::{TableauLocation, FreecellLocation, FoundationLocation};
    ///
    /// let game = GameState::new(); // Represents a new, shuffled game
    ///
    /// // Example of a potentially valid move (depending on game state)
    /// let valid_move = Move::tableau_to_freecell(0, 0).unwrap();
    /// if let Err(err) = game.is_move_valid(&valid_move) {
    ///    println!("Invalid move: {}", err);
    /// } else {
    ///   println!("Move is valid!");
    /// }
    /// ```
    pub fn is_move_valid(&self, m: &Move) -> Result<(), GameError> {
        use crate::location::Location::*;
        match (m.source, m.destination) {
            (Tableau(from), Foundation(to)) => {
                self.validate_tableau_to_foundation(from.index(), to.index(), m)
            }
            (Tableau(from), Freecell(to)) => {
                self.validate_tableau_to_freecell(from.index(), to.index(), m)
            }
            (Freecell(from), Tableau(to)) => {
                self.validate_freecell_to_tableau(from.index(), to.index(), m)
            }
            (Freecell(from), Foundation(to)) => {
                self.validate_freecell_to_foundation(from.index(), to.index(), m)
            }
            (Tableau(from), Tableau(to)) => {
                self.validate_tableau_to_tableau(from.index(), to.index(), m.card_count, m)
            }
            _ => Err(GameError::InvalidMove {
                reason: "Moves between these locations are not supported".to_string(),
                attempted_move: *m,
            }),
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
    /// use freecell_game_engine::{GameState, Card, Rank, Suit, Move};
    /// use freecell_game_engine::game_state::GameError;
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move.
    /// // let location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(location, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
    ///
    /// // Validate the move. The result depends on the initial deal.
    /// let move_cmd = Move::tableau_to_foundation(0, 0).unwrap();
    /// let result = game.is_move_valid(&move_cmd);
    /// ```
    fn validate_tableau_to_foundation(&self, from_column: u8, to_pile: u8, m: &Move) -> Result<(), GameError> {
        let location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        let card = self.tableau.get_card(location)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_foundation".to_string(),
            })?
            .ok_or_else(|| GameError::InvalidMove {
                reason: "Source tableau column is empty".to_string(),
                attempted_move: *m,
            })?;
        self.foundations.validate_card_placement(to_pile as usize, card)
            .map_err(|e| GameError::Foundation {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_foundation".to_string(),
            })?;
        Ok(())
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
    /// use freecell_game_engine::{GameState, Card, Rank, Suit, Move};
    /// use freecell_game_engine::game_state::GameError;
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move.
    /// // let location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(location, Card::new(Rank::King, Suit::Spades)).unwrap();
    ///
    /// // Validate the move. The result depends on the initial deal.
    /// let move_cmd = Move::tableau_to_freecell(0, 0).unwrap();
    /// let result = game.is_move_valid(&move_cmd);
    /// ```
    fn validate_tableau_to_freecell(&self, from_column: u8, to_cell: u8, m: &Move) -> Result<(), GameError> {
        let location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        if self.tableau.get_card(location)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_freecell".to_string(),
            })?
            .is_none()
        {
            return Err(GameError::InvalidMove {
                reason: "Source tableau column is empty".to_string(),
                attempted_move: *m,
            });
        }
        let location = crate::location::FreecellLocation::new(to_cell).map_err(GameError::Location)?;
        if self.freecells.get_card(location)
            .map_err(|e| GameError::FreeCell {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_freecell".to_string(),
            })?
            .is_some()
        {
            return Err(GameError::InvalidMove {
                reason: "Destination freecell is occupied".to_string(),
                attempted_move: *m,
            });
        }
        Ok(())
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
    /// use freecell_game_engine::{GameState, Card, Rank, Suit, Move};
    /// use freecell_game_engine::game_state::GameError;
    /// use freecell_game_engine::location::{FreecellLocation, TableauLocation};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move.
    /// // let freecell_location = FreecellLocation::new(0).unwrap();
    /// // game.freecells_mut().place_card(freecell_location, Card::new(Rank::King, Suit::Spades)).unwrap();
    /// // let tableau_location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(tableau_location, Card::new(Rank::Queen, Suit::Hearts)).unwrap();
    ///
    /// // Validate the move. The result depends on the initial deal.
    /// let move_cmd = Move::freecell_to_tableau(0, 0).unwrap();
    /// let result = game.is_move_valid(&move_cmd);
    /// ```
    fn validate_freecell_to_tableau(&self, from_cell: u8, to_column: u8, m: &Move) -> Result<(), GameError> {
        let location = crate::location::FreecellLocation::new(from_cell).map_err(GameError::Location)?;
        let card = self.freecells.get_card(location)
            .map_err(|e| GameError::FreeCell {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_freecell_to_tableau".to_string(),
            })?
            .ok_or_else(|| GameError::InvalidMove {
                reason: "Source freecell is empty".to_string(),
                attempted_move: *m,
            })?;
        self.tableau.validate_card_placement(to_column as usize, card)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_freecell_to_tableau".to_string(),
            })?;
        Ok(())
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
    /// use freecell_game_engine::{GameState, Card, Rank, Suit, Move};
    /// use freecell_game_engine::game_state::GameError;
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up for a valid move.
    /// // let location = FreecellLocation::new(0).unwrap();
    /// // game.freecells_mut().place_card(location, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    ///
    /// // Validate the move. The result depends on the initial deal.
    /// let move_cmd = Move::freecell_to_foundation(0, 0).unwrap();
    /// let result = game.is_move_valid(&move_cmd);
    /// ```
    fn validate_freecell_to_foundation(&self, from_cell: u8, to_pile: u8, m: &Move) -> Result<(), GameError> {
        let location = crate::location::FreecellLocation::new(from_cell).map_err(GameError::Location)?;
        let card = self.freecells.get_card(location)
            .map_err(|e| GameError::FreeCell {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_freecell_to_foundation".to_string(),
            })?
            .ok_or_else(|| GameError::InvalidMove {
                reason: "Source freecell is empty".to_string(),
                attempted_move: *m,
            })?;
        self.foundations.validate_card_placement(to_pile as usize, card)
            .map_err(|e| GameError::Foundation {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_freecell_to_foundation".to_string(),
            })?;
        Ok(())
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
    fn validate_tableau_to_tableau(&self, from_column: u8, to_column: u8, card_count: u8, m: &Move) -> Result<(), GameError> {
        // Only allow single card moves for now
        if card_count != 1 {
            return Err(GameError::OnlySingleCardMovesSupported);
        }

        let from_location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        let card = self.tableau.get_card(from_location)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_tableau".to_string(),
            })?
            .ok_or_else(|| GameError::InvalidMove {
                reason: "Source tableau column is empty".to_string(),
                attempted_move: *m,
            })?;
        self.tableau.validate_card_placement(to_column as usize, card)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "validate_tableau_to_tableau".to_string(),
            })?;
        Ok(())
    }
}
