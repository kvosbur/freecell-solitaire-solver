//! Foundations implementation for FreeCell game state.
//!
//! This module provides the Foundations component of the FreeCell game, which are the
//! 4 suit-based stacks where cards are built up from Ace to King. The main components are:
//! 
//! - [`Foundations`] - Represents the foundation piles in a game
//! - [`FoundationError`] - Errors that can occur during foundation operations
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::foundations::{Foundations, FoundationError};
//! use freecell_game_engine::card::{Card, Rank, Suit};
//! use freecell_game_engine::location::FoundationLocation;
//!
//! // Create new foundation piles
//! let mut foundations = Foundations::new();
//! 
//! // Place an Ace on an empty foundation pile
//! let card = Card::new(Rank::Ace, Suit::Hearts);
//! let location = FoundationLocation::new(0).unwrap();
//! foundations.place_card(location, card).unwrap();
//! 
//! // Check for cards in a foundation pile
//! let top_card = foundations.get_card(location).unwrap().unwrap();
//! assert_eq!(top_card.rank(), Rank::Ace);
//! 
//! // Check if a pile is empty
//! assert!(!foundations.is_pile_empty(location).unwrap());
//! ```
//!
//! This module primarily implements the physical state and operations of foundation piles.
//! While validation helpers are provided via `validate_card_placement()`, the component
//! itself does not enforce game rules during operations like `place_card()`.
//! This design allows higher-level game logic to implement and control rule enforcement.

use crate::card::{Card, Rank, Suit};
use crate::location::FoundationLocation;
use std::fmt;

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
///
/// let foundations = Foundations::new();
/// 
/// // Validation errors are returned by validate_card_placement
/// let result = foundations.validate_card_placement(0, &Card::new(Rank::Two, Suit::Hearts));
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
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
/// // Place a card in pile 0
/// let card = Card::new(Rank::Ace, Suit::Hearts);
/// let location = FoundationLocation::new(0).unwrap();
/// foundations.place_card(location, card).unwrap();
/// ```
pub struct Foundations {
    piles: [Vec<Card>; 4],
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
        }
    }
}

impl std::error::Error for FoundationError {}

impl Default for Foundations {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Foundations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Foundations");
        for pile in 0..self.pile_count() {
            let pile_name = format!("pile_{}", pile);
            let location = FoundationLocation::new(pile as u8).unwrap();
            match self.get_card(location) {
                Ok(Some(card)) => debug_struct.field(&pile_name, &format!("top: {:?}", card)),
                Ok(None) => debug_struct.field(&pile_name, &"[empty]"),
                Err(_) => debug_struct.field(&pile_name, &"[error]"),
            };
        }
        debug_struct.finish()
    }
}

impl fmt::Display for Foundations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Foundations:")?;
        for i in 0..self.pile_count() {
            let location = FoundationLocation::new(i as u8).unwrap();
            match self.get_card(location) {
                Ok(Some(card)) => writeln!(f, "  Pile {}: {} (height: {})", 
                                    i, card, self.pile_height(i))?,
                Ok(None) => writeln!(f, "  Pile {}: Empty", i)?,
                Err(_) => writeln!(f, "  Pile {}: Error", i)?,
            }
        }
        Ok(())
    }
}

impl Foundations {
    /// Create a new set of foundations with 4 empty piles.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    ///
    /// let foundations = Foundations::new();
    /// assert_eq!(foundations.pile_count(), 4);
    /// assert_eq!(foundations.empty_piles_count(), 4);
    /// ```
    pub fn new() -> Self {
        Self { 
            piles: [Vec::new(), Vec::new(), Vec::new(), Vec::new()]
        }
    }
    
    /// Place a card in a foundation pile at the specified index.
    ///
    /// # Errors
    ///
    /// Returns `FoundationError::InvalidPile` if the index is out of bounds.
    /// Note that this method does not validate whether the card placement follows FreeCell rules.
    /// Use `validate_card_placement()` for rule validation.
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
    /// foundations.place_card(FoundationLocation::new(0).unwrap(), card).unwrap();
    /// ```
    pub fn place_card(&mut self, location: FoundationLocation, card: Card) -> Result<(), FoundationError> {
        self.piles[location.index() as usize].push(card);
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
    /// foundations.place_card(location, card.clone()).unwrap();
    /// 
    /// // Then remove it
    /// let removed_card = foundations.remove_card(location).unwrap();
    /// assert_eq!(removed_card, Some(card));
    /// ```
    pub fn remove_card(&mut self, location: FoundationLocation) -> Result<Option<Card>, FoundationError> {
        Ok(self.piles[location.index() as usize].pop())
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
    /// foundations.place_card(location, card.clone()).unwrap();
    /// 
    /// // Get a reference to the card
    /// let card_ref = foundations.get_card(location).unwrap().unwrap();
    /// assert_eq!(card_ref.rank(), Rank::Ace);
    /// assert_eq!(card_ref.suit(), Suit::Hearts);
    /// ```
    pub fn get_card(&self, location: FoundationLocation) -> Result<Option<&Card>, FoundationError> {
        Ok(self.piles[location.index() as usize].last())
    }
    
    /// Get the number of foundation piles.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    ///
    /// let foundations = Foundations::new();
    /// assert_eq!(foundations.pile_count(), 4);
    /// ```
    pub fn pile_count(&self) -> usize {
        self.piles.len()
    }

    /// Count the number of empty foundation piles.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// assert_eq!(foundations.empty_piles_count(), 4);
    /// 
    /// // Place a card
    /// let location = FoundationLocation::new(0).unwrap();
    /// foundations.place_card(location, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// assert_eq!(foundations.empty_piles_count(), 3);
    /// ```
    pub fn empty_piles_count(&self) -> usize {
        self.piles.iter().filter(|pile| pile.is_empty()).count()
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
    /// assert!(foundations.is_pile_empty(location).unwrap());
    /// 
    /// // Place a card
    /// foundations.place_card(location, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// assert!(!foundations.is_pile_empty(location).unwrap());
    /// ```
    pub fn is_pile_empty(&self, location: FoundationLocation) -> Result<bool, FoundationError> {
        Ok(self.piles[location.index() as usize].is_empty())
    }

    /// Get the height (number of cards) of a foundation pile.
    ///
    /// Returns 0 for invalid pile indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// assert_eq!(foundations.pile_height(0), 0);
    /// 
    /// let location = FoundationLocation::new(0).unwrap();
    /// foundations.place_card(location, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// assert_eq!(foundations.pile_height(0), 1);
    /// ```
    pub fn pile_height(&self, pile: usize) -> usize {
        if pile >= self.piles.len() {
            return 0;
        }
        self.piles[pile].len()
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
    /// let location0 = FoundationLocation::new(0).unwrap();
    /// foundations.place_card(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// let location1 = FoundationLocation::new(1).unwrap();
    /// foundations.place_card(location1, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    /// assert_eq!(foundations.total_cards(), 2);
    /// ```
    pub fn total_cards(&self) -> usize {
        self.piles.iter().map(|pile| pile.len()).sum()
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
        self.piles.iter().all(|pile| pile.len() == 13)
    }

    /// Validates if a card can be legally placed on a foundation pile according to FreeCell rules
    /// Does not modify any state - only provides validation
    /// 
    /// # Rules checked:
    /// - Only Aces can be placed on empty piles
    /// - Cards must be same suit and one rank higher than the top card
    /// - Cannot add to a pile that already has a King (complete pile)
    pub fn validate_card_placement(&self, pile: usize, card: &Card) -> Result<(), FoundationError> {
        if pile >= self.piles.len() {
            return Err(FoundationError::InvalidPile(pile as u8));
        }
        
        let pile_ref = &self.piles[pile];
        
        // For empty piles, only Aces are allowed
        if pile_ref.is_empty() {
            if card.rank() != Rank::Ace {
                return Err(FoundationError::NonAceOnEmptyPile { new_card: *card });
            }
            return Ok(());
        }
        
        // For non-empty piles, check sequence rules
        if let Some(top_card) = pile_ref.last() {
            // Check if pile is already complete
            if top_card.rank() == Rank::King {
                return Err(FoundationError::PileComplete {
                    pile_index: pile as u8,
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

    /// Returns an iterator over the non-empty foundation piles, yielding (index, pile reference) pairs.
    ///
    /// This iterator provides a convenient way to iterate through all non-empty piles
    /// without having to check each pile individually.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::foundations::Foundations;
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    /// use freecell_game_engine::location::FoundationLocation;
    ///
    /// let mut foundations = Foundations::new();
    /// let location0 = FoundationLocation::new(0).unwrap();
    /// foundations.place_card(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// let location2 = FoundationLocation::new(2).unwrap();
    /// foundations.place_card(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
    /// 
    /// // Iterate through non-empty piles
    /// let non_empty: Vec<_> = foundations.non_empty_piles().collect();
    /// assert_eq!(non_empty.len(), 2);
    /// ```
    pub fn non_empty_piles(&self) -> impl Iterator<Item = (usize, &Vec<Card>)> + '_ {
        self.piles.iter()
            .enumerate()
            .filter(|(_, pile)| !pile.is_empty())
    }

    /// Find which pile a card of the given suit should go to.
    ///
    /// This is useful for auto-completing moves or finding a target pile for a card.
    /// Returns the pile index if a pile with the matching suit is found, or
    /// the first empty pile if no pile has that suit yet. Returns None if there's
    /// no suitable pile.
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
    /// // Place Ace of Hearts in the first pile
    /// let location0 = FoundationLocation::new(0).unwrap();
    /// foundations.place_card(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
    /// 
    /// // Find pile for another Hearts card
    /// let hearts_pile = foundations.find_pile_for_suit(Suit::Hearts);
    /// assert_eq!(hearts_pile, Some(0));
    /// 
    /// // Find pile for a new suit (will return first empty pile)
    /// let spades_pile = foundations.find_pile_for_suit(Suit::Spades);
    /// assert_eq!(spades_pile, Some(1)); // First empty pile
    /// ```
    pub fn find_pile_for_suit(&self, suit: Suit) -> Option<usize> {
        // First check if there's already a pile for this suit
        for (i, pile) in self.piles.iter().enumerate() {
            if let Some(card) = pile.last() {
                if card.suit() == suit {
                    return Some(i);
                }
            }
        }
        
        // If no pile has this suit yet, find the first empty pile
        for (i, pile) in self.piles.iter().enumerate() {
            if pile.is_empty() {
                return Some(i);
            }
        }
        
        // No suitable pile found
        None
    }

}

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
            foundations.pile_count(),
            4,
            "Foundations should have 4 piles"
        );
        for i in 0..foundations.pile_count() {
            assert_eq!(
                foundations.pile_height(i),
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
        foundations.place_card(location, card.clone()).unwrap();
        assert_eq!(foundations.pile_height(0), 1);
        
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

        foundations.place_card(location, ace.clone()).unwrap();
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), Rank::Ace);
        assert_eq!(top_card.suit(), Suit::Hearts);

        foundations.place_card(location, two.clone()).unwrap();
        let top_card = foundations.get_card(location).unwrap().unwrap();
        assert_eq!(top_card.rank(), Rank::Two);
        assert_eq!(top_card.suit(), Suit::Hearts);

        foundations.place_card(location, three.clone()).unwrap();
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
        assert!(foundations.place_card(valid_location, card).is_ok());

        // The following would not compile:
        // let invalid_location = FoundationLocation::new(4).unwrap();
    }

    #[test]
    fn error_implements_error_trait() {
        // Check that we can use FoundationError with Box<dyn Error>
        fn returns_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
            let foundations = Foundations::new();
            foundations.validate_card_placement(0, &Card::new(Rank::Two, Suit::Hearts))?;
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
        foundations.place_card(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        
        // Place Ace of Diamonds in second pile
        let location1 = FoundationLocation::new(1).unwrap();
        foundations.place_card(location1, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
        
        // For hearts, should return the first pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        
        // For diamonds, should return the second pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        
        // For spades or clubs, should return the next empty pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(2));
        
        // If we fill all piles with different suits
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        let location3 = FoundationLocation::new(3).unwrap();
        foundations.place_card(location3, Card::new(Rank::Ace, Suit::Spades)).unwrap();
        
        // Each suit should map to its pile
        assert_eq!(foundations.find_pile_for_suit(Suit::Hearts), Some(0));
        assert_eq!(foundations.find_pile_for_suit(Suit::Diamonds), Some(1));
        assert_eq!(foundations.find_pile_for_suit(Suit::Clubs), Some(2));
        assert_eq!(foundations.find_pile_for_suit(Suit::Spades), Some(3));
    }

    #[test]
    fn non_empty_piles_iterator_works() {
        let mut foundations = Foundations::new();
        
        // Empty foundations should yield no piles
        let non_empty: Vec<_> = foundations.non_empty_piles().collect();
        assert_eq!(non_empty.len(), 0);
        
        // Add cards to piles 0 and 2
        let location0 = FoundationLocation::new(0).unwrap();
        foundations.place_card(location0, Card::new(Rank::Ace, Suit::Hearts)).unwrap();
        let location2 = FoundationLocation::new(2).unwrap();
        foundations.place_card(location2, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
        
        // Should now have 2 non-empty piles
        let non_empty: Vec<_> = foundations.non_empty_piles().collect();
        assert_eq!(non_empty.len(), 2);
        assert_eq!(non_empty[0].0, 0); // First pile index is 0
        assert_eq!(non_empty[1].0, 2); // Second pile index is 2
        
        // Check the contents of the piles
        assert_eq!(non_empty[0].1.len(), 1);
        assert_eq!(non_empty[1].1.len(), 1);
    }

    #[test]
    fn can_use_foundation_location() {
        let mut foundations = Foundations::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        let location = FoundationLocation::new(0).unwrap();

        foundations.place_card(location, card.clone()).unwrap();
        assert_eq!(foundations.get_card(location).unwrap(), Some(&card));
        assert_eq!(foundations.remove_card(location).unwrap(), Some(card));
        assert_eq!(foundations.get_card(location).unwrap(), None);
    }
}
