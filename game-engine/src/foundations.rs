//! Foundations implementation for FreeCell game state.
//!
//! Foundations are the 4 suit-based stacks where cards are built up from Ace to King.

use crate::card::Card;
use crate::rules;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Foundations {
    piles: [Vec<Card>; 4],
}

impl Foundations {
    /// Returns the top card of the given pile, or None if the pile is empty.
    pub fn get_top_card(&self, pile: usize) -> Option<&Card> {
        self.piles[pile].last()
    }
    pub fn new() -> Self {
        Self {
            piles: Default::default(),
        }
    }

    pub fn pile_count(&self) -> usize {
        self.piles.len()
    }

    pub fn is_pile_empty(&self, index: usize) -> bool {
        self.piles[index].is_empty()
    }

    pub fn add_card(&mut self, index: usize, card: Card) {
        let foundation_top = self.get_card(index);
        if !rules::can_move_to_foundation(&card, foundation_top) {
            if foundation_top.is_none() {
                panic!("Can only add Ace to empty foundation pile");
            } else {
                panic!("Card must be one rank higher and same suit");
            }
        }
        self.piles[index].push(card);
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.piles[index].last()
    }

    /// Returns true if the specified foundation pile is complete (Ace through King, 13 cards).
    pub fn is_pile_complete(&self, index: usize) -> bool {
        let pile = &self.piles[index];
        if pile.len() != 13 {
            return false;
        }
        true
    }

    pub fn remove_top_card(&mut self, pile: usize) -> Option<Card> {
        if pile < self.piles.len() {
            self.piles[pile].pop()
        } else {
            None
        }
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
            assert!(
                foundations.is_pile_empty(i),
                "Pile {} should be empty on initialization",
                i
            );
        }
    }

    #[test]
    fn can_add_card_to_empty_foundation() {
        let mut foundations = Foundations::new();
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        foundations.add_card(0, card.clone());
        assert!(!foundations.is_pile_empty(0));
        assert_eq!(foundations.get_card(0), Some(&card));
    }

    #[test]
    fn can_build_foundation_stack() {
        let mut foundations = Foundations::new();
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

        foundations.add_card(0, ace.clone());
        assert_eq!(foundations.get_card(0), Some(&ace));

        foundations.add_card(0, two.clone());
        assert_eq!(foundations.get_card(0), Some(&two));

        foundations.add_card(0, three.clone());
        assert_eq!(foundations.get_card(0), Some(&three));
    }

    #[test]
    #[should_panic(expected = "Can only add Ace to empty foundation pile")]
    fn cannot_add_non_ace_to_empty_foundation() {
        let mut foundations = Foundations::new();
        let not_ace = Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
        };
        foundations.add_card(0, not_ace);
    }

    #[test]
    #[should_panic(expected = "Card must be one rank higher and same suit")]
    fn cannot_add_wrong_rank_or_suit_to_foundation() {
        let mut foundations = Foundations::new();
        let ace = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        let two_wrong_suit = Card {
            rank: Rank::Two,
            suit: Suit::Spades,
        };

        foundations.add_card(0, ace);
        foundations.add_card(0, two_wrong_suit); // should panic
    }

    #[test]
    fn pile_is_complete_when_all_13_cards_of_suit_are_present() {
        let mut foundations = Foundations::new();
        for rank in 1..=13 {
            foundations.add_card(
                0,
                Card {
                    rank: Rank::try_from(rank).unwrap(),
                    suit: Suit::Hearts,
                },
            );
        }
        assert!(foundations.is_pile_complete(0));
    }

    #[test]
    fn pile_is_not_complete_if_missing_cards() {
        let mut foundations = Foundations::new();
        for rank in 1..=12 {
            foundations.add_card(
                0,
                Card {
                    rank: Rank::try_from(rank).unwrap(),
                    suit: Suit::Hearts,
                },
            );
        }
        assert!(!foundations.is_pile_complete(0));
    }
}
