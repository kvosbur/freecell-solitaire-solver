//! Foundations implementation for FreeCell solitaire.
//!
//! # Overview
//!
//! In FreeCell solitaire, the foundations are the four piles at the top of the game board where 
//! cards are ultimately collected to win the game. Each foundation pile corresponds to one suit 
//! (Hearts, Diamonds, Clubs, or Spades) and must be built up from Ace to King.
//!
//! This module provides:
//! - [`Foundations`] - The main struct representing all foundation piles
//! - [`FoundationError`] - Errors that can occur during foundation operations
//!
//! # Foundation Rules
//!
//! The rules for building foundations in FreeCell are:
//!
//! 1. Each foundation corresponds to one suit (Hearts, Diamonds, Clubs, or Spades)
//! 2. Cards in each foundation must start with an Ace
//! 3. Cards must be placed in ascending order (A, 2, 3, ..., Q, K)
//! 4. All cards must be of the same suit within a foundation pile
//! 5. When all 52 cards are in the foundations (13 in each pile), the game is won
//!
//! # Usage
//!
//! ```
//! use freecell_game_engine::foundations::{Foundations, FoundationError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//! use freecell_game_engine::location::FoundationLocation;
//!
//! // Create new foundation piles
//! let mut foundations = Foundations::new();
//! 
//! // Create a location for the Hearts foundation (index 0)
//! let location = FoundationLocation::new(0).unwrap();
//! 
//! // Place an Ace of Hearts (first card must be an Ace)
//! foundations.place_card_at(location, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
//! 
//! // Place the Two of Hearts (must follow sequence)
//! foundations.place_card_at(location, Card::new(Rank::Two, Suit::Hearts)).unwrap();
//! 
//! // Check the top card
//! let top_card = foundations.get_card(location).unwrap().unwrap();
//! assert_eq!(top_card.rank(), Rank::Two);
//! assert_eq!(top_card.suit(), Suit::Hearts);
//! 
//! // Check if the foundation is complete
//! assert!(!foundations.is_complete());
//! 
//! // Get the number of cards in a pile
//! assert_eq!(foundations.get_card(location).unwrap().is_some(), true);
//! assert_eq!(foundations.is_empty(location).unwrap(), false);
//! ```
//!
//! # Design Notes
//!
//! This module implements the physical state and operations of foundation piles.
//! While validation helpers are provided via `validate_card_placement()`, the component
//! itself does not enforce game rules during operations like `place_card()`.
//! This design allows higher-level game logic to implement and control rule enforcement.
//!
//! All methods that operate on a specific foundation pile take a [`FoundationLocation`] 
//! parameter for type safety, which ensures that only valid foundation indices can be used.
//!
//! The implementation uses fixed-size arrays for efficient memory usage and performance:
//! - Each pile is represented as an array of `Option<Card>` with capacity for 13 cards (A-K)
//! - A separate `heights` array tracks the current size of each pile for O(1) access
//! - This approach offers better performance than vectors when accessing top cards or checking pile status

use crate::card::{Card, Rank, Suit};
use crate::location::FoundationLocation;
use std::fmt;

/// The number of foundation piles in FreeCell (one for each suit).
pub const FOUNDATION_COUNT: usize = 4;

/// The maximum number of cards in each foundation pile (Ace through King).
pub const FOUNDATION_CAPACITY: usize = 13;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for foundation operations.
///
/// This enum represents all the possible error conditions that can occur
/// when interacting with foundation piles.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::foundations::{Foundations, FoundationError};
/// use freecell_game_engine::card::{Card, Rank, Suit};
/// use freecell_game_engine::location::FoundationLocation;
///
/// let foundations = Foundations::new();
/// 
/// // Validation errors are returned by validate_card_placement
/// let location = FoundationLocation::new(0).unwrap();
/// let result = foundations.validate_card_placement(location, &Card::new(Rank::Two, Suit::Hearts));
/// assert!(matches!(result, Err(FoundationError::NonAceOnEmptyPile { .. })));
/// ```
pub enum FoundationError {
    /// Attempted to access an invalid pile index.
    InvalidPile(u8),
    
    /// Attempted to add a non-Ace card to an empty foundation.
    NonAceOnEmptyPile { new_card: Card },
    
    /// Card doesn't follow sequence rules (must be same suit, one rank higher).
    InvalidSequence {
        top_card: Card,
        new_card: Card,
    },
    
    /// Attempted to place card on a completed pile (King is already placed).
    PileComplete {
        pile_index: u8,
        new_card: Card,
    },
    
    /// No suitable foundation pile available for this card.
    NoAvailablePile { card: Card },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Represents the foundation piles where cards are collected by suit.
///
/// Foundations are the 4 suit-based stacks in a FreeCell game where cards are built up 
/// from Ace to King. These piles are the goal destination for all cards, and when all cards
/// are moved to these piles, the game is won.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::foundations::Foundations;
/// use freecell_game_engine::card::{Card, Rank, Suit};
/// use freecell_game_engine::location::FoundationLocation;
///
/// // Create a new empty set of foundations
/// let mut foundations = Foundations::new();
///
/// // Place a card automatically (recommended)
/// let ace_hearts = Card::new(Rank::Ace, Suit::Hearts);
/// let location = foundations.place_card(ace_hearts).unwrap();
///
/// // Or place a card at a specific location
/// let ace_spades = Card::new(Rank::Ace, Suit::Spades);
/// let specific_location = FoundationLocation::new(1).unwrap();
/// foundations.place_card_at(specific_location, ace_spades).unwrap();
/// ```
pub struct Foundations {
    // Fixed-size array for each pile with options for each card position
    // Using fixed-size arrays for efficient memory usage and stack allocation
    piles: [[Option<Card>; FOUNDATION_CAPACITY]; FOUNDATION_COUNT],
    // Track the current height of each pile for O(1) access to pile information
    // This avoids having to scan through arrays to find the first None element
    heights: [usize; FOUNDATION_COUNT],
}

impl fmt::Display for FoundationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FoundationError::InvalidPile(index) => write!(f, "Invalid foundation pile index: {}", index),
            FoundationError::NonAceOnEmptyPile { new_card } => write!(
                f,
                "Cannot place {} on empty foundation: only Aces are allowed",
                new_card
            ),
            FoundationError::InvalidSequence { top_card, new_card } => write!(
                f,
                "Cannot place {} on {}: invalid sequence",
                new_card, top_card
            ),
            FoundationError::PileComplete { pile_index, new_card } => write!(
                f,
                "Cannot place {} on pile {}: pile is already complete",
                new_card, pile_index
            ),
            FoundationError::NoAvailablePile { card } => write!(
                f,
                "No available foundation pile for {}",
                card
            ),
        }
    }
}

impl std::error::Error for FoundationError {}

impl Default for Foundations {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Foundations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Foundations:")?;
        for i in 0..FOUNDATION_COUNT {
            let location = FoundationLocation::new(i as u8).unwrap();
            match self.get_card(location) {
                Ok(Some(card)) => {
                    let height = self.height(location);
                    let suit_name = format!("{:?}", card.suit());
                    writeln!(f, "  {}: {} (height: {}/{})", 
                            suit_name, card, height, FOUNDATION_CAPACITY)?;
                },
                Ok(None) => writeln!(f, "  {}: Empty", match i {
                    0 => "Hearts",
                    1 => "Diamonds", 
                    2 => "Clubs",
                    3 => "Spades",
                    _ => "Unknown"
                })?,
                Err(_) => writeln!(f, "  Foundation {}: Error", i)?,
            }
        }
        Ok(())
    }
}

impl Foundations {
    /// Create a new set of foundations with empty piles.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::{Foundations, FOUNDATION_COUNT};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let foundations = Foundations::new();
    /// 
    /// // Verify all piles are empty
    /// for i in 0..FOUNDATION_COUNT {
    ///     let location = FoundationLocation::new(i as u8).unwrap();
    ///     assert!(foundations.is_empty(location).unwrap());
    /// }
    /// ```
    pub fn new() -> Self {
        // Initialize with empty piles and zero heights
        Self { 
            piles: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            heights: [0; FOUNDATION_COUNT]
        }
    }
    
    /// Place a card in the appropriate foundation pile automatically.
    ///
    /// This method finds the correct pile for the card based on its suit,
    /// validates that the placement follows FreeCell rules, and places the card.
    ///
    /// # Returns
    ///
    /// Returns the location where the card was placed.
    ///
    /// # Errors
    ///
    /// - `FoundationError::NoAvailablePile` if there's no suitable pile for this card
    /// - `FoundationError::NonAceOnEmptyPile` if trying to place a non-Ace on an empty pile
    /// - `FoundationError::InvalidSequence` if the card doesn't follow the sequence rules
    /// - `FoundationError::PileComplete` if the pile already has a King
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut foundations = Foundations::new();
    ///
    /// // Automatically place an Ace
    /// let ace_hearts = Card::new(Rank::Ace, Suit::Hearts);
    /// let location = foundations.place_card(ace_hearts).unwrap();
    ///
    /// // Automatically place the Two of Hearts (will go to the same pile)
    /// let two_hearts = Card::new(Rank::Two, Suit::Hearts);
    /// foundations.place_card(two_hearts).unwrap();
    /// ```
    pub fn place_card(&mut self, card: Card) -> Result<FoundationLocation, FoundationError> {
        // Find appropriate pile
        let suit = card.suit();
        let pile = self.find_pile_for_suit(suit)
            .ok_or(FoundationError::NoAvailablePile { card: card.clone() })?;
        
        // Convert to location
        let location = FoundationLocation::new(pile as u8)
            .map_err(|_| FoundationError::InvalidPile(pile as u8))?;
            
        // Validate placement
        self.validate_card_placement(location, &card)?;
        
        // Place the card
        self.place_card_at(location, card)?;
        
        Ok(location)
    }

    /// Place a card in a specific foundation pile at the given location.
    ///
    /// This method validates that the card placement follows FreeCell rules before placing the card.
    ///
    /// # Errors
    ///
    /// - `FoundationError::NonAceOnEmptyPile` if trying to place a non-Ace on an empty pile
    /// - `FoundationError::InvalidSequence` if the card doesn't follow the sequence rules
    /// - `FoundationError::PileComplete` if the pile already has a King
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// foundations.place_card_at(FoundationLocation::new(0).unwrap(), card).unwrap();
    /// ```
    pub fn place_card_at(&mut self, location: FoundationLocation, card: Card) -> Result<(), FoundationError> {
        // Validate the card placement first - this covers all the rule checks including capacity
        self.validate_card_placement(location, &card)?;
        
        let idx = location.index() as usize;
        let height = self.heights[idx];
        
        // Store the card at the current height position
        self.piles[idx][height] = Some(card);
        
        // Increment the height
        self.heights[idx] += 1;
        
        Ok(())
    }
    
    /// Remove and return the top card from a foundation pile.
    ///
    /// Returns the card if one was present, or `None` if the pile was empty.
    ///
    /// # Errors
    ///
    /// Returns `FoundationError::InvalidPile` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// 
    /// // Place a card first
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// let location = FoundationLocation::new(0).unwrap();
    /// foundations.place_card_at(location, card.clone()).unwrap();
    /// 
    /// // Then remove it
    /// let removed_card = foundations.remove_card(location).unwrap();
    /// assert_eq!(removed_card, Some(card));
    /// ```
    pub fn remove_card(&mut self, location: FoundationLocation) -> Result<Option<Card>, FoundationError> {
        let idx = location.index() as usize;
        let height = self.heights[idx];
        
        if height == 0 {
            return Ok(None);
        }
        
        // Get the new height after removing the card
        let new_height = height - 1;
        
        // Get the card
        let card = self.piles[idx][new_height].take();
        
        // Decrement the height
        self.heights[idx] = new_height;
        
        Ok(card)
    }
    
    /// Get a reference to the top card in a foundation pile without removing it.
    ///
    /// # Errors
    ///
    /// Returns `FoundationError::InvalidPile` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// let location = FoundationLocation::new(0).unwrap();
    /// foundations.place_card_at(location, card.clone()).unwrap();
    /// 
    /// // Get a reference to the card
    /// let card_ref = foundations.get_card(location).unwrap().unwrap();
    /// assert_eq!(card_ref.rank(), Rank::Ace);
    /// assert_eq!(card_ref.suit(), Suit::Hearts);
    /// ```
    pub fn get_card(&self, location: FoundationLocation) -> Result<Option<&Card>, FoundationError> {
        let idx = location.index() as usize;
        let height = self.heights[idx];
        
        if height == 0 {
            Ok(None)
        } else {
            // Subtract 1 from height to get the index of the top card
            Ok(self.piles[idx][height - 1].as_ref())
        }
    }

    /// Check if a pile is empty.
    ///
    /// # Errors
    ///
    /// Returns `FoundationError::InvalidPile` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// let location = FoundationLocation::new(0).unwrap();
    /// assert!(foundations.is_empty(location).unwrap());
    /// 
    /// // Place a card
    /// foundations.place_card_at(location, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// assert!(!foundations.is_empty(location).unwrap());
    /// ```
    pub fn is_empty(&self, location: FoundationLocation) -> Result<bool, FoundationError> {
        Ok(self.heights[location.index() as usize] == 0)
    }

    /// Get the total number of cards in all foundations.
    ///
    /// This is useful for tracking progress toward game completion.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// assert_eq!(foundations.total_cards(), 0);
    /// 
    /// // Place cards automatically
    /// foundations.place_card(Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// foundations.place_card(Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    /// assert_eq!(foundations.total_cards(), 2);
    /// ```
    pub fn total_cards(&self) -> usize {
        self.heights.iter().sum()
    }

    /// Check if all foundations are complete (game won).
    ///
    /// The game is considered complete when all foundation piles have all 13 cards.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let mut foundations = Foundations::new();
    /// assert!(!foundations.is_complete()); // Empty foundations are not complete
    /// 
    /// // A fully populated foundation would return true
    /// ```
    pub fn is_complete(&self) -> bool {
        self.heights.iter().all(|&height| height == FOUNDATION_CAPACITY)
    }

    /// Validates if a card can be legally placed on a foundation pile according to FreeCell rules
    /// Does not modify any state - only provides validation
    /// 
    /// # Rules checked:
    /// - Only Aces can be placed on empty piles
    /// - Cards must be same suit and one rank higher than the top card
    /// - Cannot add to a pile that already has a King (complete pile)
    /// 
    /// # Errors
    ///
    /// - `FoundationError::NonAceOnEmptyPile` if trying to place a non-Ace on an empty pile
    /// - `FoundationError::InvalidSequence` if the card doesn't follow the sequence rules
    /// - `FoundationError::PileComplete` if the pile already has a King
    pub fn validate_card_placement(&self, location: FoundationLocation, card: &Card) -> Result<(), FoundationError> {
        let pile_idx = location.index() as usize;
        let height = self.heights[pile_idx];
        
        // For empty piles, only Aces are allowed
        if height == 0 {
            if card.rank() != Rank::Ace {
                return Err(FoundationError::NonAceOnEmptyPile { new_card: *card });
            }
            return Ok(());
        }
        
        // For non-empty piles, check sequence rules
        if let Some(top_card) = self.get_card(location)? {
            // Check if pile is already complete
            if top_card.rank() == Rank::King {
                return Err(FoundationError::PileComplete {
                    pile_index: location.index(),
                    new_card: *card,
                });
            }
            
            // Check if card follows sequence rules
            let expected_rank = Rank::try_from((top_card.rank() as u8) + 1)
                .map_err(|_| FoundationError::InvalidSequence { top_card: *top_card, new_card: *card })?;
                
            if card.suit() != top_card.suit() || card.rank() != expected_rank {
                return Err(FoundationError::InvalidSequence { top_card: *top_card, new_card: *card });
            }
        }
        
        Ok(())
    }

    /// Find which pile a card of the given suit should go to.
    ///
    /// This is used internally by `place_card()` to find the correct pile for automatic placement.
    /// Returns the pile index if a pile with the matching suit is found, or
    /// the first empty pile if no pile has that suit yet. Returns None if there's
    /// no suitable pile.
    ///
    /// Note: This method is only available within the crate (`pub(crate)`).
    ///
    /// # Internal Example (not available to external users)
    ///
    /// ```ignore
    /// // Internal crate code:
    /// let mut foundations = Foundations::new();
    /// 
    /// foundations.place_card_at(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// 
    /// // Find pile for Hearts cards
    /// let hearts_pile = foundations.find_pile_for_suit(Suit::Hearts);
    /// assert_eq!(hearts_pile, Some(0));
    /// 
    /// // Find pile for a new suit (will return first empty pile)
    /// let spades_pile = foundations.find_pile_for_suit(Suit::Spades);
    /// assert_eq!(spades_pile, Some(1)); // First empty pile
    /// ```
    pub(crate) fn find_pile_for_suit(&self, suit: Suit) -> Option<usize> {
        // First check if there's already a pile for this suit
        for i in 0..FOUNDATION_COUNT {
            let location = FoundationLocation::new(i as u8).unwrap();
            if let Ok(Some(card)) = self.get_card(location) {
                if card.suit() == suit {
                    return Some(i);
                }
            }
        }
        
        // If no pile has this suit yet, find the first empty pile
        for i in 0..FOUNDATION_COUNT {
            if self.heights[i] == 0 {
                return Some(i);
            }
        }
        
        // No suitable pile found
        None
    }
}

    /// Get the height (number of cards) of a foundation pile.
    ///
    /// This is a private implementation method used internally by other methods.
    fn height(&self, location: FoundationLocation) -> usize {
        self.heights[location.index() as usize]
    }
}

impl std::error::Error for FoundationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{Card, Suit},
        Rank,
    };

    #[test]
    fn foundations_initialize_with_four_empty_piles() {
        let foundations = Foundations::new();
        assert_eq!(
            FOUNDATION_COUNT,
            4,
            "Foundations should have 4 piles"
        );
        for i in 0..FOUNDATION_COUNT {
            let location = FoundationLocation::new(i as u8).unwrap();
            assert_eq!(
                foundations.height(location),
                0,
                "Pile {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_foundation() {
        let mut foundations = Foundations::new();
        let card = Card::new(Rank::Ace, Suit::Hearts);
        let location = FoundationLocation::new(0).unwrap();
        foundations.place_card_at(location, card.clone()).unwrap();
        assert_eq!(foundations.height(location), 1);
        
        // Compare top card's rank and suit instead of the card itself
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), card.rank());
        assert_eq!(top_card.suit(), card.suit());
    }

    #[test]
    fn can_build_foundation_stack() {
        let mut foundations = Foundations::new();
        let ace = Card::new(Rank::Ace, Suit::Hearts);
        let two = Card::new(Rank::Two, Suit::Hearts);
        let three = Card::new(Rank::Three, Suit::Hearts);
        let location = FoundationLocation::new(0).unwrap();

        foundations.place_card_at(location, ace.clone()).unwrap();
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), Rank::Ace);
        assert_eq!(top_card.suit(), Suit::Hearts);

        foundations.place_card_at(location, two.clone()).unwrap();
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), Rank::Two);
        assert_eq!(top_card.suit(), Suit::Hearts);

        foundations.place_card_at(location, three.clone()).unwrap();
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), Rank::Three);
        assert_eq!(top_card.suit(), Suit::Hearts);
    }

    // Note: With FoundationLocation, out-of-bounds errors are prevented at compile time.
    #[test]
    fn foundation_location_prevents_out_of_bounds() {
        let mut foundations = Foundations::new();
        let card = Card::new(Rank::Ace, Suit::Hearts);
        
        // This will fail to compile if using an invalid index with FoundationLocation::new
        let valid_location = FoundationLocation::new(3).unwrap();
        assert!(foundations.place_card_at(valid_location, card).is_ok());

        // The following would not compile:
        // let invalid_location = FoundationLocation::new(4).unwrap();
    }

    #[test]
    fn error_implements_error_trait() {
        // Check that we can use FoundationError with Box<dyn Error>
        fn returns_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
            let foundations = Foundations::new();
            let location = FoundationLocation::new(0).unwrap();
            foundations.validate_card_placement(location, &Card::new(Rank::Two, Suit::Hearts))?;
            Ok(())
        }

        let result = returns_box_dyn_error();
        assert!(result.is_err());
        
        // Convert to string to check Display implementation works
        let error_string = result.unwrap_err().to_string();
        assert!(error_string.contains("only Aces are allowed"));
    }

    #[test]
    fn can_find_pile_for_specific_suit() {
        let mut foundations = Foundations::new();
        
        // All piles are empty, so first pile should be returned for any suit
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        
        // Place Ace of Hearts in first pile
        let location0 = FoundationLocation::new(0).unwrap();
        foundations.place_card_at(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        
        // Place Ace of Diamonds in second pile
        let location1 = FoundationLocation::new(1).unwrap();
        foundations.place_card_at(location1, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();

        // Place Ace of Diamonds in second pile
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card_at(location2, Card::new(Rank::Ace, Suit::Spades)).unwrap();

        // Place Ace of Diamonds in second pile
        let location3 = FoundationLocation::new(3).unwrap();
        foundations.place_card_at(location3, Card::new(Rank::Ace, Suit::Clubs)).unwrap();

        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(2));
        assert_eq!(foundations.find_pile_for_suit(Suit::Clubs), Some(3));
        
        // If we fill all piles with different suits
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card_at(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        let location3 = FoundationLocation::new(3).unwrap();
        foundations.place_card_at(location3, Card::new(Rank::Ace, Suit::Spades)).unwrap();
        
        // Each suit should map to its pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        assert_eq!(foundations.find_pile_for_suit(Suit::Clubs), Some(2));
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(3));
    }

    #[test]
    fn find_pile_for_suit_returns_first_empty_pile_for_new_suit() {
        let mut foundations = Foundations::new();
        
        // With all piles empty, first pile should be returned for any suit
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        
        // Place Ace of Hearts in first pile
        let location0 = FoundationLocation::new(0).unwrap();
        foundations.place_card_at(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        
        // For a new suit, should return the next empty pile (index 1)
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        
        // Place Ace of Diamonds in second pile
        let location1 = FoundationLocation::new(1).unwrap();
        foundations.place_card_at(location1, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
        
        // For next new suit, should return the next empty pile (index 2)
        assert_eq!(foundations.find_pile_for_suit(Suit::Clubs), Some(2));
        
        // Place Ace of Clubs in third pile
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card_at(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        
        // For final new suit, should return the last empty pile (index 3)
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(3));
    }

    #[test]
    fn find_pile_for_suit_returns_correct_pile_for_existing_suit() {
        let mut foundations = Foundations::new();
        
        // Place each suit in a specific pile
        let hearts_pile = FoundationLocation::new(0).unwrap();
        foundations.place_card_at(hearts_pile, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        
        let diamonds_pile = FoundationLocation::new(1).unwrap();
        foundations.place_card_at(diamonds_pile, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
        
        let clubs_pile = FoundationLocation::new(2).unwrap();
        foundations.place_card_at(clubs_pile, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        
        let spades_pile = FoundationLocation::new(3).unwrap();
        foundations.place_card_at(spades_pile, Card::new(Rank::Ace, Suit::Spades)).unwrap();
        
        // Now test that each suit maps to the correct pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        assert_eq!(foundations.find_pile_for_suit(Suit::Clubs), Some(2));
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(3));
        
        // Add some more cards to piles to ensure we're looking at suit, not just first card
        foundations.place_card_at(hearts_pile, Card::new(Rank::Two, Suit::Hearts)).unwrap();
        foundations.place_card_at(diamonds_pile, Card::new(Rank::Two, Suit::Diamonds)).unwrap();
        
        // Verify we still find the correct piles
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
    }

    #[test]
    fn checking_non_empty_piles_works() {
        let mut foundations = Foundations::new();
        
        // Check all piles are initially empty
        let mut non_empty_count = 0;
        for i in 0..FOUNDATION_COUNT {
            let location = FoundationLocation::new(i as u8).unwrap();
            if !foundations.is_empty(location).unwrap() {
                non_empty_count += 1;
            }
        }
        assert_eq!(non_empty_count, 0, "All piles should be empty initially");
        
        // Add cards to piles 0 and 2
        let location0 = FoundationLocation::new(0).unwrap();
        foundations.place_card_at(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card_at(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        
        // Should now have 2 non-empty piles
        let mut non_empty_locations = Vec::new();
        for i in 0..FOUNDATION_COUNT {
            let location = FoundationLocation::new(i as u8).unwrap();
            if !foundations.is_empty(location).unwrap() {
                non_empty_locations.push((i, foundations.height(location)));
            }
        }
        
        assert_eq!(non_empty_locations.len(), 2, "Should have 2 non-empty piles");
        assert_eq!(non_empty_locations[0].0, 0, "First pile index should be 0");
        assert_eq!(non_empty_locations[1].0, 2, "Second pile index should be 2");
        
        // Check the heights of the piles
        assert_eq!(non_empty_locations[0].1, 1, "First pile should have 1 card");
        assert_eq!(non_empty_locations[1].1, 1, "Second pile should have 1 card");
    }

    #[test]
    fn can_use_foundation_location() {
        let mut foundations = Foundations::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        let location = FoundationLocation::new(0).unwrap();

        foundations.place_card_at(location, card.clone()).unwrap();
        assert_eq!(foundations.get_card(location).unwrap(), Some(&card));
        assert_eq!(foundations.remove_card(location).unwrap(), Some(card));
        assert_eq!(foundations.get_card(location).unwrap(), None);
    }

    #[test]
    fn auto_place_cards_works() {
        let mut foundations = Foundations::new();
        
        // Place Ace of Hearts
        let ace_hearts = Card::new(Rank::Ace, Suit::Hearts);
        let location = foundations.place_card(ace_hearts).unwrap();
        
        // Location should be the first pile
        assert_eq!(location.index(), 0);
        
        // Place Two of Hearts - should go to the same pile
        let two_hearts = Card::new(Rank::Two, Suit::Hearts);
        let location = foundations.place_card(two_hearts).unwrap();
        assert_eq!(location.index(), 0);
        
        // Place Ace of Spades - should go to a different pile
        let ace_spades = Card::new(Rank::Ace, Suit::Spades);
        let location = foundations.place_card(ace_spades).unwrap();
        assert_eq!(location.index(), 1); // Should go to the next empty pile
        
        // Get top cards and verify
        let hearts_pile = FoundationLocation::new(0).unwrap();
        let spades_pile = FoundationLocation::new(1).unwrap();
        
        assert_eq!(foundations.get_card(hearts_pile).unwrap().unwrap().rank(), Rank::Two);
        assert_eq!(foundations.get_card(hearts_pile).unwrap().unwrap().suit(), Suit::Hearts);
        
        assert_eq!(foundations.get_card(spades_pile).unwrap().unwrap().rank(), Rank::Ace);
        assert_eq!(foundations.get_card(spades_pile).unwrap().unwrap().suit(), Suit::Spades);
        
        // Try placing an invalid card (Three of Hearts without Two)
        let three_spades = Card::new(Rank::Three, Suit::Spades);
        let result = foundations.place_card(three_spades);
        assert!(result.is_err());
        assert!(matches!(result, Err(FoundationError::InvalidSequence { .. })));
    }
}
