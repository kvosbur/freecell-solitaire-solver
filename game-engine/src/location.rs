//! Provides type-safe, validated location identifiers for all game areas.
//!
//! This module introduces structs and enums to represent specific locations
//! within the FreeCell game, such as a particular tableau column, a freecell,
//! or a foundation pile. By using these types, the game engine can enforce

//! that all location-based operations are valid at compile time or through
//! runtime checks, preventing common errors like out-of-bounds indices.
//!
//! # Core Components
//!
//! - [`TableauLocation`]: A validated wrapper for a tableau column index (0-7).
//! - [`FreecellLocation`]: A validated wrapper for a freecell index (0-3).
//! - [`FoundationLocation`]: A validated wrapper for a foundation pile index (0-3).
//! - [`Location`]: An enum that consolidates all location types, useful for
//!   representing moves between different areas of the game.
//! - [`LocationError`]: An error type for location-related validation failures.

use std::fmt;
use serde::{Deserialize, Serialize};

// General error for location validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationError {
    InvalidTableauIndex(u8),
    InvalidFreecellIndex(u8),
    InvalidFoundationIndex(u8),
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationError::InvalidTableauIndex(i) => write!(f, "Invalid tableau index: {}", i),
            LocationError::InvalidFreecellIndex(i) => write!(f, "Invalid freecell index: {}", i),
            LocationError::InvalidFoundationIndex(i) => write!(f, "Invalid foundation index: {}", i),
        }
    }
}

impl std::error::Error for LocationError {}

/// Represents a validated location in a tableau column (0-7).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableauLocation {
    index: u8,
}

impl TableauLocation {
    /// Creates a new `TableauLocation` if the index is valid (0-7).
    pub fn new(index: u8) -> Result<Self, LocationError> {
        if index < 8 {
            Ok(Self { index })
        } else {
            Err(LocationError::InvalidTableauIndex(index))
        }
    }

    /// Returns the raw index of the tableau column.
    pub fn index(&self) -> u8 {
        self.index
    }
}

/// Represents a validated location in a freecell (0-3).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FreecellLocation {
    index: u8,
}

impl FreecellLocation {
    /// Creates a new `FreecellLocation` if the index is valid (0-3).
    pub fn new(index: u8) -> Result<Self, LocationError> {
        if index < 4 {
            Ok(Self { index })
        } else {
            Err(LocationError::InvalidFreecellIndex(index))
        }
    }

    /// Returns the raw index of the freecell.
    pub fn index(&self) -> u8 {
        self.index
    }
}

/// Represents a validated location in a foundation pile (0-3).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FoundationLocation {
    index: u8,
}

impl FoundationLocation {
    /// Creates a new `FoundationLocation` if the index is valid (0-3).
    pub fn new(index: u8) -> Result<Self, LocationError> {
        if index < 4 {
            Ok(Self { index })
        } else {
            Err(LocationError::InvalidFoundationIndex(index))
        }
    }

    /// Returns the raw index of the foundation pile.
    pub fn index(&self) -> u8 {
        self.index
    }
}

/// An enum that consolidates all location types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Location {
    Tableau(TableauLocation),
    Freecell(FreecellLocation),
    Foundation(FoundationLocation),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tableau_location_validation() {
        for i in 0..8 {
            assert!(TableauLocation::new(i).is_ok());
        }
        assert!(matches!(
            TableauLocation::new(8),
            Err(LocationError::InvalidTableauIndex(8))
        ));
        assert!(matches!(
            TableauLocation::new(255),
            Err(LocationError::InvalidTableauIndex(255))
        ));
    }

    #[test]
    fn freecell_location_validation() {
        for i in 0..4 {
            assert!(FreecellLocation::new(i).is_ok());
        }
        assert!(matches!(
            FreecellLocation::new(4),
            Err(LocationError::InvalidFreecellIndex(4))
        ));
        assert!(matches!(
            FreecellLocation::new(255),
            Err(LocationError::InvalidFreecellIndex(255))
        ));
    }

    #[test]
    fn foundation_location_validation() {
        for i in 0..4 {
            assert!(FoundationLocation::new(i).is_ok());
        }
        assert!(matches!(
            FoundationLocation::new(4),
            Err(LocationError::InvalidFoundationIndex(4))
        ));
        assert!(matches!(
            FoundationLocation::new(255),
            Err(LocationError::InvalidFoundationIndex(255))
        ));
    }

    #[test]
    fn location_enum_creation() {
        let tableau_loc = TableauLocation::new(0).unwrap();
        let freecell_loc = FreecellLocation::new(1).unwrap();
        let foundation_loc = FoundationLocation::new(2).unwrap();

        let loc1 = Location::Tableau(tableau_loc);
        let loc2 = Location::Freecell(freecell_loc);
        let loc3 = Location::Foundation(foundation_loc);

        assert_eq!(loc1, Location::Tableau(TableauLocation::new(0).unwrap()));
        assert_eq!(loc2, Location::Freecell(FreecellLocation::new(1).unwrap()));
        assert_eq!(loc3, Location::Foundation(FoundationLocation::new(2).unwrap()));
    }
}
