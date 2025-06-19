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
    NoCardInFreecell,
    CannotMoveToFoundation,
    CannotStackOnTableau,
    CannotMoveToFreecell,
}

use std::fmt;

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
            GameError::NoCardInFreecell => write!(f, "No card in freecell"),
            GameError::CannotMoveToFoundation => write!(f, "Cannot move card to foundation"),
            GameError::CannotStackOnTableau => write!(f, "Cannot stack card on tableau"),
            GameError::CannotMoveToFreecell => write!(f, "Cannot move card to freecell"),
        }
    }
}
