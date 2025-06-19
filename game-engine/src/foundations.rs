//! Foundations implementation for FreeCell game state.
//!
//! Foundations are the 4 suit-based stacks where cards are built up from Ace to King.

use crate::card::Card;
use crate::rules;

#[derive(Clone, PartialEq, Eq, Hash)]
/// Represents the foundation piles where cards are collected by suit
pub struct Foundations {
    piles: Vec<Vec<Card>>,
}

impl Foundations {
    /// Create new foundations with the specified number of piles (usually 4)
    pub fn new(pile_count: usize) -> Self {
        Self { piles: vec![Vec::new(); pile_count] }
    }
    
    /// Add a card to a foundation pile
    pub fn add_card(&mut self, pile: usize, card: Card) -> Result<(), FoundationError> {
        if pile >= self.piles.len() {
            return Err(FoundationError::InvalidPile);
        }
        
        self.piles[pile].push(card);
        Ok(())
    }
    
    /// Remove the top card from a foundation pile
    pub fn remove_top_card(&mut self, pile: usize) -> Option<Card> {
        if pile >= self.piles.len() {
            return None;
        }
        self.piles[pile].pop()
    }
    
    /// Get a reference to the top card in a foundation pile
    pub fn get_top_card(&self, pile: usize) -> Option<&Card> {
        if pile >= self.piles.len() {
            return None;
        }
        self.piles[pile].last()
    }
    
    /// Get the number of foundation piles
    pub fn pile_count(&self) -> usize {
        self.piles.len()
    }
    
    /// Get the height of a foundation pile
    pub fn pile_height(&self, pile: usize) -> usize {
        if pile >= self.piles.len() {
            return 0;
        }
        self.piles[pile].len()
    }
    
    /// Get the total number of cards in all foundations
    pub fn total_cards(&self) -> usize {
        self.piles.iter().map(|pile| pile.len()).sum()
    }
    
    /// Check if all foundations are complete (game won)
    pub fn is_complete(&self, cards_per_suit: usize) -> bool {
        self.piles.iter().all(|pile| pile.len() == cards_per_suit)
    }
}

use std::fmt;

impl fmt::Debug for Foundations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Foundations");
        for pile in 0..self.pile_count() {
            let pile_name = format!("pile_{}", pile);
            match self.get_top_card(pile) {
                Some(card) => debug_struct.field(&pile_name, &format!("top: {:?}", card)),
                None => debug_struct.field(&pile_name, &"[empty]"),
            };
        }
        debug_struct.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error type for foundation operations
pub enum FoundationError {
    /// Attempted to access an invalid pile index
    InvalidPile,
    
    /// Attempted to add a non-Ace card to an empty foundation
    NonAceOnEmptyPile,
    
    /// Card doesn't follow sequence rules (must be same suit, one rank higher)
    InvalidSequence,
    
    /// Attempted to place card on a completed pile (King is already placed)
    PileComplete,
}

impl std::fmt::Display for FoundationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoundationError::InvalidPile => write!(f, "Invalid foundation pile index"),
            FoundationError::NonAceOnEmptyPile => write!(f, "Can only add Ace to empty foundation pile"),
            FoundationError::InvalidSequence => write!(f, "Card must be one rank higher and same suit"),
            FoundationError::PileComplete => write!(f, "Foundation pile is already complete"),
        }
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
        let foundations = Foundations::new(4);
        assert_eq!(
            foundations.pile_count(),
            4,
            "Foundations should have 4 piles"
        );
        for i in 0..foundations.pile_count() {
            assert!(
                foundations.is_pile_empty(i),
                "Pile {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_foundation() {
        let mut foundations = Foundations::new(4);
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        foundations.add_card(0, card.clone()).unwrap();
        assert!(!foundations.is_pile_empty(0));
        assert_eq!(foundations.get_card(0), Some(&card));
    }

    #[test]
    fn can_build_foundation_stack() {
        let mut foundations = Foundations::new(4);
        let ace = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        let two = Card {
            rank: Rank::Two,
            suit: Suit::Hearts,
        };
        let three = Card {
            rank: Rank::Three,
            suit: Suit::Hearts,
        };

        foundations.add_card(0, ace.clone()).unwrap();
        assert_eq!(foundations.get_card(0), Some(&ace));

        foundations.add_card(0, two.clone()).unwrap();
        assert_eq!(foundations.get_card(0), Some(&two));

        foundations.add_card(0, three.clone()).unwrap();
        assert_eq!(foundations.get_card(0), Some(&three));
    }

    #[test]
    #[should_panic(expected = "Can only add Ace to empty foundation pile")]
    fn cannot_add_non_ace_to_empty_foundation() {
        let mut foundations = Foundations::new(4);
        let not_ace = Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
        };
        foundations.add_card(0, not_ace).unwrap();
    }

    #[test]
    #[should_panic(expected = "Card must be one rank higher and same suit")]
    fn cannot_add_wrong_rank_or_suit_to_foundation() {
        let mut foundations = Foundations::new(4);
        let ace = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        let two_wrong_suit = Card {
            rank: Rank::Two,
            suit: Suit::Spades,
        };

        foundations.add_card(0, ace).unwrap();
        foundations.add_card(0, two_wrong_suit).unwrap(); // should panic
    }

    #[test]
    fn pile_is_complete_when_all_13_cards_of_suit_are_present() {
        let mut foundations = Foundations::new(4);
        for rank in 1..=13 {
            foundations.add_card(
                0,
                Card {
                    rank: Rank::try_from(rank).unwrap(),
                    suit: Suit::Hearts,
                },
            ).unwrap();
        }
        assert!(foundations.is_pile_complete(0));
    }

    #[test]
    fn pile_is_not_complete_if_missing_cards() {
        let mut foundations = Foundations::new(4);
        for rank in 1..=12 {
            foundations.add_card(
                0,
                Card {
                    rank: Rank::try_from(rank).unwrap(),
                    suit: Suit::Hearts,
                },
            ).unwrap();
        }
        assert!(!foundations.is_pile_complete(0));
    }
}
