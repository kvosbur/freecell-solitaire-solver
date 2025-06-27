//! Error types for GameState operations.

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    InvalidMove(String),
    IndexOutOfBounds {
        component: &'static str,
        index: usize,
    },
    EmptySource,
    OnlySingleCardMovesSupported,
    NoCardInTableauColumn,
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
