//! Error types for GameState operations.
//!
//! This module defines the `GameError` enum, which encapsulates all possible
//! errors that can occur during game state manipulations, such as invalid moves,
//! out-of-bounds access, or attempts to move cards from empty sources.
//!
//! `GameError` provides a unified error handling mechanism for the FreeCell game engine,
//! making it easier to manage and interpret errors across different components.
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::{GameState, Move};
//! use freecell_game_engine::game_state::GameError;
//! use freecell_game_engine::location::{TableauLocation, FreecellLocation};
//!
//! let mut game = GameState::new();
//! // Attempt an invalid move (e.g., moving from an empty column)
//! let invalid_move = Move::tableau_to_freecell(0, 0).unwrap();
//! let result = game.is_move_valid(&invalid_move);
//!
//! if let Err(err) = result {
//!    // The error message will be more specific now, e.g.:
//!    // "Invalid move T(0)->F(0): Source tableau column is empty"
//!    println!("Invalid move: {}", err);
//! }
//! ```

use crate::r#move::Move;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum GameError {
    /// A location-based error occurred.
    Location(LocationError),
    /// A freecell-related error occurred.
    FreeCell {
        error: FreeCellError,
        attempted_move: Option<Move>,
        operation: String,
    },
    /// A foundation-related error occurred.
    Foundation {
        error: FoundationError,
        attempted_move: Option<Move>,
        operation: String,
    },
    /// A tableau-related error occurred.
    Tableau {
        error: TableauError,
        attempted_move: Option<Move>,
        operation: String,
    },
    /// The attempted move is invalid for a specific reason.
    InvalidMove {
        reason: String,
        attempted_move: Move,
    },
    /// Indicates that a multi-card move was attempted when only single card moves are supported.
    OnlySingleCardMovesSupported,
}

use std::fmt;
use crate::freecells::FreeCellError;
use crate::foundations::FoundationError;
use crate::location::LocationError;
use crate::tableau::TableauError;

impl From<LocationError> for GameError {
    fn from(err: LocationError) -> Self {
        GameError::Location(err)
    }
}


impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::Location(err) => write!(f, "Location error: {}", err),
            GameError::FreeCell { error, attempted_move, operation } => {
                let move_str = attempted_move.map_or("".to_string(), |m| format!(" during move {}", m));
                write!(f, "FreeCell error during {}: {}{}", operation, error, move_str)
            }
            GameError::Foundation { error, attempted_move, operation } => {
                let move_str = attempted_move.map_or("".to_string(), |m| format!(" during move {}", m));
                write!(f, "Foundation error during {}: {}{}", operation, error, move_str)
            }
            GameError::Tableau { error, attempted_move, operation } => {
                let move_str = attempted_move.map_or("".to_string(), |m| format!(" during move {}", m));
                write!(f, "Tableau error during {}: {}{}", operation, error, move_str)
            }
            GameError::InvalidMove { reason, attempted_move } => {
                write!(f, "Invalid move {}: {}", attempted_move, reason)
            }
            GameError::OnlySingleCardMovesSupported => write!(f, "Only single card moves are supported"),
        }
    }
}

impl std::error::Error for GameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GameError::Location(err) => Some(err),
            GameError::FreeCell { error, .. } => Some(error),
            GameError::Foundation { error, .. } => Some(error),
            GameError::Tableau { error, .. } => Some(error),
            _ => None,
        }
    }
}
