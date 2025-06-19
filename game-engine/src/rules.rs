use crate::card::Card;

/// Core rules module for FreeCell solitaire
pub struct Rules;

impl Rules {
    /// Check if a card can be stacked on a tableau column
    /// Works with both empty and non-empty columns
    pub fn can_stack_on_tableau(card: &Card, tableau_top: Option<&Card>) -> bool {
        match tableau_top {
            // Empty column - any card can be placed
            None => true,
            
            // Non-empty column - check color and rank
            Some(top) => card.color() != top.color() && (card.rank as u8) + 1 == (top.rank as u8)
        }
    }

    /// Check if a card can be moved to a foundation pile
    pub fn can_move_to_foundation(card: &Card, foundation_top: Option<&Card>) -> bool {
        match foundation_top {
            // Empty foundation - only Ace can be placed
            None => (card.rank) as u8 == 1,
            
            // Non-empty foundation - check suit and rank
            Some(top) => card.suit == top.suit && (card.rank as u8) == (top.rank as u8) + 1
        }
    }

    /// Check if a card can be moved to a freecell
    pub fn can_move_to_freecell(_card: &Card, freecell_content: Option<&Card>) -> bool {
        // Can only move to empty freecells
        freecell_content.is_none()
    }
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
        assert_eq!(Rules::can_stack_on_tableau(&moving_card, Some(&target_card)), expected);
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
        assert_eq!(Rules::can_move_to_foundation(&card, foundation_top.as_ref()), expected);
    }

    #[rstest]
    #[case(Card{rank: Rank::Five, suit: Suit::Hearts}, None, true)]
    #[case(Card{rank: Rank::Seven, suit: Suit::Spades}, Some(Card{rank: Rank::Two, suit: Suit::Clubs}), false)]
    fn can_move_to_freecell_test(
        #[case] card: Card,
        #[case] freecell: Option<Card>,
        #[case] expected: bool
    ) {
        let result = Rules::can_move_to_freecell(&card, freecell.as_ref());
        assert_eq!(result, expected);
    }
}
