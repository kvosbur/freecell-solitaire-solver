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
//! use freecell_game_engine::{GameState, r#move::Move};
//! use freecell_game_engine::game_state::GameError;
//!
//! let mut game = GameState::new();
//! // Attempt an invalid move (e.g., moving from an empty column)
//! let invalid_move = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
//! let result = game.execute_move(&invalid_move);
//!
//! assert!(result.is_err());
//! if let Err(GameError::EmptySource) = result {
//!     println!("Caught expected error: Empty source column!");
//! }
//! ```

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    /// Represents a general invalid move with a descriptive message.
    InvalidMove(String),
    /// Indicates an attempt to access an index outside the valid range for a component.
    IndexOutOfBounds {
        /// The name of the component (e.g., "freecell", "tableau", "foundation").
        component: &'static str,
        /// The index that was out of bounds.
        index: usize,
    },
    /// Attempted to move a card from an empty source (e.g., empty tableau column or freecell).
    EmptySource,
    /// Indicates that a multi-card move was attempted when only single card moves are supported.
    OnlySingleCardMovesSupported,
    /// No card was found in the specified tableau column for an operation.
    NoCardInTableauColumn,
    /// Attempted to stack a card on a tableau column in an invalid way (e.g., wrong rank or color).
    CannotStackOnTableau,
}

use std::fmt;
use crate::freecells::FreeCellError;
use crate::foundations::FoundationError;
use crate::tableau::TableauError;

impl From<FreeCellError> for GameError {
    fn from(err: FreeCellError) -> Self {
        match err {
            FreeCellError::InvalidCell => GameError::IndexOutOfBounds {
                component: "freecell",
                index: 0, // Could be enhanced to include actual index
            },
            FreeCellError::CellOccupied => GameError::InvalidMove("Freecell is already occupied".to_string()),
            FreeCellError::NoEmptyCells => GameError::InvalidMove("No empty freecells available".to_string()),
        }
    }
}

impl From<FoundationError> for GameError {
    fn from(err: FoundationError) -> Self {
        match err {
            FoundationError::InvalidPile => GameError::IndexOutOfBounds {
                component: "foundation",
                index: 0, // Could be enhanced to include actual index
            },
            FoundationError::NonAceOnEmptyPile => GameError::InvalidMove("Can only add Ace to empty foundation pile".to_string()),
            FoundationError::InvalidSequence => GameError::InvalidMove("Card must be one rank higher and same suit".to_string()),
            FoundationError::PileComplete => GameError::InvalidMove("Foundation pile is already complete".to_string()),
        }
    }
}

impl From<TableauError> for GameError {
    fn from(err: TableauError) -> Self {
        match err {
            TableauError::InvalidColumn => GameError::IndexOutOfBounds {
                component: "tableau",
                index: 0, // Could be enhanced to include actual index
            },
            TableauError::InvalidStack => GameError::InvalidMove("Invalid tableau stack move".to_string()),
            TableauError::EmptyColumn => GameError::NoCardInTableauColumn,
            TableauError::InvalidCardIndex => GameError::InvalidMove("Invalid card index in tableau column".to_string()),
            TableauError::InvalidColor => GameError::InvalidMove("Cards must be of alternating colors".to_string()),
            TableauError::InvalidRank => GameError::InvalidMove("Cards must be in descending rank order".to_string()),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
            GameError::IndexOutOfBounds { component, index } => {
                write!(f, "Index {} out of bounds for {}", index, component)
            }
            GameError::EmptySource => write!(f, "Cannot move from empty source"),
            GameError::OnlySingleCardMovesSupported => write!(f, "Only single card moves are supported"),
            GameError::NoCardInTableauColumn => write!(f, "No card in tableau column"),
            GameError::CannotStackOnTableau => write!(f, "Cannot stack card on tableau"),
        }
    }
}
