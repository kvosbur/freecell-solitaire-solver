//! Move representation for FreeCell game transitions.
//!
//! This module defines the `Move` struct which represents all possible moves in a FreeCell game.
//! Moves represent the transfer of cards between different areas of the game
//! according to FreeCell rules.
//!
//! # Move Types
//!
//! All moves represent transferring cards between these areas:
//! - **Tableau**: The main playing area with 8 columns.
//! - **FreeCells**: 4 temporary storage cells.
//! - **Foundations**: 4 piles for building up suits from Ace to King.
//!
//! Moves can be for a single card or a sequence of cards (tableau-to-tableau).
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::{GameState, Move};
//! use freecell_game_engine::location::*;
//!
//! // Create a new game
//! let mut game = GameState::new();
//!
//! // Define a move to transfer a card from a tableau column to a freecell
//! let move_cmd = Move::single(
//!     Location::Tableau(TableauLocation::new(0).unwrap()),
//!     Location::Freecell(FreecellLocation::new(0).unwrap())
//! );
//!
//! // Validate and execute the move
//! if game.is_move_valid(&move_cmd).is_ok() {
//!     game.execute_move(&move_cmd).unwrap();
//! }
//! ```
use crate::location::{Location, TableauLocation, FreecellLocation, FoundationLocation, LocationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Move {
    pub source: Location,
    pub destination: Location,
    pub card_count: u8,
}

impl Move {
    /// Creates a new single-card move.
    pub fn single(source: Location, destination: Location) -> Self {
        Self {
            source,
            destination,
            card_count: 1,
        }
    }

    /// Creates a new multi-card sequence move.
    pub fn sequence(source: Location, destination: Location, card_count: u8) -> Self {
        Self {
            source,
            destination,
            card_count,
        }
    }

    pub fn tableau_to_foundation(from: u8, to: u8) -> Result<Self, LocationError> {
        Ok(Self::single(
            Location::Tableau(TableauLocation::new(from)?),
            Location::Foundation(FoundationLocation::new(to)?),
        ))
    }

    pub fn tableau_to_freecell(from: u8, to: u8) -> Result<Self, LocationError> {
        Ok(Self::single(
            Location::Tableau(TableauLocation::new(from)?),
            Location::Freecell(FreecellLocation::new(to)?),
        ))
    }

    pub fn freecell_to_tableau(from: u8, to: u8) -> Result<Self, LocationError> {
        Ok(Self::single(
            Location::Freecell(FreecellLocation::new(from)?),
            Location::Tableau(TableauLocation::new(to)?),
        ))
    }

    pub fn freecell_to_foundation(from: u8, to: u8) -> Result<Self, LocationError> {
        Ok(Self::single(
            Location::Freecell(FreecellLocation::new(from)?),
            Location::Foundation(FoundationLocation::new(to)?),
        ))
    }

    pub fn tableau_to_tableau(from: u8, to: u8, card_count: u8) -> Result<Self, LocationError> {
        Ok(Self::sequence(
            Location::Tableau(TableauLocation::new(from)?),
            Location::Tableau(TableauLocation::new(to)?),
            card_count,
        ))
    }

    /// Returns the source `Location` of the move.
    pub fn source(&self) -> Location {
        self.source
    }

    /// Returns the destination `Location` of the move.
    pub fn destination(&self) -> Location {
        self.destination
    }

    /// Returns the number of cards moved.
    pub fn card_count(&self) -> u8 {
        self.card_count
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.card_count == 1 {
            write!(f, "{} → {}", self.source, self.destination)
        } else {
            write!(f, "{} → {} ({} cards)", self.source, self.destination, self.card_count)
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Tableau(loc) => write!(f, "Tableau {}", loc.index()),
            Location::Freecell(loc) => write!(f, "Freecell {}", loc.index()),
            Location::Foundation(loc) => write!(f, "Foundation {}", loc.index()),
        }
    }
}
