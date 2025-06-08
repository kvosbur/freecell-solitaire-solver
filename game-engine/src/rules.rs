//! Game rules and validation logic for FreeCell moves.

use crate::card::Card;

/// Returns true if `moving` can be placed on `target` in the tableau (alternating color, descending rank).
pub fn can_stack_on_tableau(moving: &Card, target: &Card) -> bool {
    (moving.rank + 1 == target.rank) && (moving.color() != target.color())
}

/// Returns true if `card` can be placed on the foundation with the given top card (or empty).
pub fn can_move_to_foundation(card: &Card, foundation_top: Option<&Card>) -> bool {
    match foundation_top {
        None => card.rank == 1,
        Some(top) => card.suit == top.suit && card.rank == top.rank + 1,
    }
}

/// Returns Ok(()) if `card` can be placed in the free cell (only if empty), or an error message.
pub fn can_move_to_freecell(_card: &Card, freecell: Option<&Card>) -> Result<(), &'static str> {
    if freecell.is_none() {
        Ok(())
    } else {
        Err("Cell is already occupied")
    }
}

/// Returns true if `card` can be placed in an empty tableau column (always true).
pub fn can_move_to_empty_tableau(_card: &Card) -> bool {
    true
}

/// Returns false - cards can never be moved from foundations.
pub fn can_move_from_foundation() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit};
    use rstest::rstest;

    #[rstest]
    #[case(Card{rank: 7, suit: Suit::Hearts}, Card{rank: 8, suit: Suit::Spades}, true)]   // Red 7 on Black 8
    #[case(Card{rank: 6, suit: Suit::Clubs}, Card{rank: 7, suit: Suit::Diamonds}, true)]  // Black 6 on Red 7
    #[case(Card{rank: 1, suit: Suit::Hearts}, Card{rank: 2, suit: Suit::Clubs}, true)]    // Red Ace on Black 2
    #[case(Card{rank: 8, suit: Suit::Hearts}, Card{rank: 7, suit: Suit::Spades}, false)]  // Wrong rank order
    #[case(Card{rank: 7, suit: Suit::Hearts}, Card{rank: 8, suit: Suit::Diamonds}, false)] // Same color
    #[case(Card{rank: 7, suit: Suit::Hearts}, Card{rank: 7, suit: Suit::Spades}, false)]  // Same rank
    #[case(Card{rank: 7, suit: Suit::Hearts}, Card{rank: 9, suit: Suit::Spades}, false)]  // Gap in ranks
    fn can_stack_on_tableau_test(
        #[case] moving_card: Card,
        #[case] target_card: Card,
        #[case] expected: bool
    ) {
        assert_eq!(can_stack_on_tableau(&moving_card, &target_card), expected);
    }

    #[rstest]
    #[case(Card{rank: 1, suit: Suit::Hearts}, None, true)] // Ace on empty
    #[case(Card{rank: 2, suit: Suit::Hearts}, Some(Card{rank: 1, suit: Suit::Hearts}), true)] // 2 on Ace, same suit
    #[case(Card{rank: 13, suit: Suit::Spades}, Some(Card{rank: 12, suit: Suit::Spades}), true)] // King on Queen, same suit
    #[case(Card{rank: 2, suit: Suit::Hearts}, None, false)] // Non-Ace on empty
    #[case(Card{rank: 2, suit: Suit::Spades}, Some(Card{rank: 1, suit: Suit::Hearts}), false)] // Different suit
    #[case(Card{rank: 3, suit: Suit::Hearts}, Some(Card{rank: 1, suit: Suit::Hearts}), false)] // Skip rank
    #[case(Card{rank: 1, suit: Suit::Hearts}, Some(Card{rank: 1, suit: Suit::Hearts}), false)] // Ace on Ace
    fn can_move_to_foundation_test(
        #[case] card: Card,
        #[case] foundation_top: Option<Card>,
        #[case] expected: bool
    ) {
        let foundation_top_ref = foundation_top.as_ref();
        assert_eq!(can_move_to_foundation(&card, foundation_top_ref), expected);
    }

    #[rstest]
    #[case(Card{rank: 5, suit: Suit::Hearts}, None, true)] // Any card to empty free cell
    #[case(Card{rank: 7, suit: Suit::Spades}, Some(Card{rank: 2, suit: Suit::Clubs}), false)] // Can't place on occupied free cell
    fn can_move_to_freecell_test(
        #[case] card: Card,
        #[case] freecell: Option<Card>,
        #[case] expected: bool
    ) {
        let freecell_ref = freecell.as_ref();
        let result = can_move_to_freecell(&card, freecell_ref);
        if expected {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
        }
    }

    #[rstest]
    #[case(Card{rank: 3, suit: Suit::Diamonds}, true)] // Any card to empty tableau
    fn can_move_to_empty_tableau_test(
        #[case] card: Card,
        #[case] expected: bool
    ) {
        assert_eq!(can_move_to_empty_tableau(&card), expected);
    }

    #[test]
    fn foundation_cards_are_immutable_test() {
        assert_eq!(can_move_from_foundation(), false);
    }
}
