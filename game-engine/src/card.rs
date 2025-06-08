//! Card-related types and functionality for FreeCell.

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

impl Card {
    pub fn color(&self) -> Color {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Spades | Suit::Clubs => Color::Black,
        }
    }

    pub fn is_valid_rank(&self) -> bool {
        self.rank >= 1 && self.rank <= 13
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Suit::Hearts, Color::Red)]
    #[case(Suit::Diamonds, Color::Red)]
    #[case(Suit::Spades, Color::Black)]
    #[case(Suit::Clubs, Color::Black)]
    fn card_has_correct_color(#[case] suit: Suit, #[case] expected_color: Color) {
        let card = Card { rank: 7, suit };
        assert_eq!(card.color(), expected_color);
    }

    #[rstest]
    #[case(1, true)]
    #[case(7, true)]
    #[case(11, true)]
    #[case(13, true)]
    #[case(0, false)]
    #[case(14, false)]
    fn card_validates_rank(#[case] rank: u8, #[case] expected: bool) {
        let card = Card { rank, suit: Suit::Hearts };
        assert_eq!(card.is_valid_rank(), expected);
    }
}
