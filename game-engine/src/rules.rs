use crate::card::Card;

/// Returns true if `moving` can be placed on `target` in the tableau (alternating color, descending rank).
pub fn can_stack_on_tableau(moving: &Card, target: &Card) -> bool {
    (moving.rank as u8 + 1 == target.rank as u8) && (moving.color() != target.color())
}

/// Returns true if `card` can be placed on the foundation with the given top card (or empty).
pub fn can_move_to_foundation(card: &Card, foundation_top: Option<&Card>) -> bool {
    match foundation_top {
        None => card.rank as u8 == 1,
        Some(top) => card.suit == top.suit && (card.rank as u8) == (top.rank as u8 + 1),
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
    use crate::card::{Card, Rank, Suit};
    use rstest::rstest;

    #[rstest]
    #[case(Card{rank: Rank::Seven, suit: Suit::Hearts}, Card{rank: Rank::Eight, suit: Suit::Spades}, true)]
    #[case(Card{rank: Rank::Six, suit: Suit::Clubs}, Card{rank: Rank::Seven, suit: Suit::Diamonds}, true)]
    #[case(Card{rank: Rank::Ace, suit: Suit::Hearts}, Card{rank: Rank::Two, suit: Suit::Clubs}, true)]
    #[case(Card{rank: Rank::Eight, suit: Suit::Hearts}, Card{rank: Rank::Seven, suit: Suit::Spades}, false)]
    #[case(Card{rank: Rank::Seven, suit: Suit::Hearts}, Card{rank: Rank::Eight, suit: Suit::Diamonds}, false)]
    #[case(Card{rank: Rank::Seven, suit: Suit::Hearts}, Card{rank: Rank::Seven, suit: Suit::Spades}, false)]
    #[case(Card{rank: Rank::Seven, suit: Suit::Hearts}, Card{rank: Rank::Nine, suit: Suit::Spades}, false)]
    fn can_stack_on_tableau_test(
        #[case] moving_card: Card,
        #[case] target_card: Card,
        #[case] expected: bool
    ) {
        assert_eq!(can_stack_on_tableau(&moving_card, &target_card), expected);
    }

    #[rstest]
    #[case(Card{rank: Rank::Ace, suit: Suit::Hearts}, None, true)]
    #[case(Card{rank: Rank::Two, suit: Suit::Hearts}, Some(Card{rank: Rank::Ace, suit: Suit::Hearts}), true)]
    #[case(Card{rank: Rank::King, suit: Suit::Spades}, Some(Card{rank: Rank::Queen, suit: Suit::Spades}), true)]
    #[case(Card{rank: Rank::Two, suit: Suit::Hearts}, None, false)]
    #[case(Card{rank: Rank::Two, suit: Suit::Spades}, Some(Card{rank: Rank::Ace, suit: Suit::Hearts}), false)]
    #[case(Card{rank: Rank::Three, suit: Suit::Hearts}, Some(Card{rank: Rank::Ace, suit: Suit::Hearts}), false)]
    #[case(Card{rank: Rank::Ace, suit: Suit::Hearts}, Some(Card{rank: Rank::Ace, suit: Suit::Hearts}), false)]
    fn can_move_to_foundation_test(
        #[case] card: Card,
        #[case] foundation_top: Option<Card>,
        #[case] expected: bool
    ) {
        let foundation_top_ref = foundation_top.as_ref();
        assert_eq!(can_move_to_foundation(&card, foundation_top_ref), expected);
    }

    #[rstest]
    #[case(Card{rank: Rank::Five, suit: Suit::Hearts}, None, true)]
    #[case(Card{rank: Rank::Seven, suit: Suit::Spades}, Some(Card{rank: Rank::Two, suit: Suit::Clubs}), false)]
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
    #[case(Card{rank: Rank::Three, suit: Suit::Diamonds}, true)]
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
