//! Card-related types and functionality for FreeCell.
//!
//! This module provides the core card primitives needed for implementing
//! a FreeCell solitaire game, including:
//!
//! - [`Card`] - Represents a playing card with a rank and suit
//! - [`Rank`] - Enumerates possible card ranks (Ace through King)
//! - [`Suit`] - Enumerates possible card suits (Spades, Hearts, Diamonds, Clubs)
//! - [`Color`] - Enumerates card colors (Red or Black)
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::card::{Card, Rank, Suit, Color};
//!
//! // Create a new card
//! let card = Card::new(Rank::Ace, Suit::Spades);
//! 
//! // Get card properties
//! assert_eq!(card.rank(), Rank::Ace);
//! assert_eq!(card.suit(), Suit::Spades);
//! assert_eq!(card.color(), Color::Black);
//! 
//! // Compare cards
//! let higher_card = Card::new(Rank::Two, Suit::Spades);
//! assert!(higher_card.is_one_higher_than(&card));
//! ```

use core::fmt;

/// Represents a playing card with a rank and suit.
///
/// Cards are used as the primary building block for the FreeCell solitaire game.
/// Each card has a rank (Ace through King) and a suit (Spades, Hearts, Diamonds, or Clubs).
///
/// # Examples
///
/// ```
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// println!("{}", card); // Outputs: "Ace of Spades"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

/// Represents the rank of a playing card.
///
/// Ranks range from Ace (value 1) to King (value 13).
/// The numeric value is accessible by casting to u8: `rank as u8`.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::card::Rank;
///
/// let rank = Rank::Ace;
/// assert_eq!(rank as u8, 1);
///
/// let rank_from_number = Rank::try_from(5).unwrap();
/// assert_eq!(rank_from_number, Rank::Five);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// Represents the suit of a playing card.
///
/// The four standard suits are Spades, Hearts, Diamonds, and Clubs.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::card::{Suit, Color};
///
/// let suit = Suit::Hearts;
/// // Hearts are red
/// assert_eq!(suit.color(), Color::Red);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

/// Represents the color of a playing card (Red or Black).
///
/// - Red suits: Hearts and Diamonds
/// - Black suits: Spades and Clubs
///
/// This is particularly important for FreeCell rules where
/// cards must alternate colors in tableau columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// Conversion from numeric values to Rank.
///
/// Allows creating a Rank from a u8 value (1-13), where:
/// - 1 = Ace
/// - 2-10 = Corresponding number card
/// - 11 = Jack
/// - 12 = Queen
/// - 13 = King
///
/// Returns an error for values outside the range 1-13.
impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Rank::Ace),
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            _ => Err(()),
        }
    }
}

/// Conversion from numeric values to Suit.
///
/// Allows creating a Suit from a u8 value (0-3), where:
/// - 0 = Spades
/// - 1 = Hearts
/// - 2 = Diamonds
/// - 3 = Clubs
///
/// Returns an error for values outside the range 0-3.
impl TryFrom<u8> for Suit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Spades),
            1 => Ok(Suit::Hearts),
            2 => Ok(Suit::Diamonds),
            3 => Ok(Suit::Clubs),
            _ => Err(()),
        }
    }
}

/// Returns the color associated with this suit.
///
/// - Hearts and Diamonds are Red
/// - Spades and Clubs are Black
impl Suit {
    /// Returns the color of this suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Suit, Color};
    ///
    /// assert_eq!(Suit::Hearts.color(), Color::Red);
    /// assert_eq!(Suit::Spades.color(), Color::Black);
    /// ```
    pub fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }

    /// Returns the foundation index for this suit.
    ///
    /// Each suit corresponds to a specific foundation pile:
    /// - Spades = 0
    /// - Hearts = 1
    /// - Diamonds = 2
    /// - Clubs = 3
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::Suit;
    ///
    /// assert_eq!(Suit::Spades.foundation_index(), 0);
    /// assert_eq!(Suit::Hearts.foundation_index(), 1);
    /// assert_eq!(Suit::Diamonds.foundation_index(), 2);
    /// assert_eq!(Suit::Clubs.foundation_index(), 3);
    /// ```
    pub fn foundation_index(&self) -> u8 {
        match self {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3,
        }
    }
}

impl Card {
    /// Creates a new card with the given rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// ```
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
    
    /// Returns the color of the card (Red or Black).
    ///
    /// The color is determined by the suit:
    /// - Hearts and Diamonds are Red
    /// - Spades and Clubs are Black
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Card, Rank, Suit, Color};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// assert_eq!(card.color(), Color::Red);
    /// ```
    pub fn color(&self) -> Color {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Clubs | Suit::Spades => Color::Black,
        }
    }
    
    /// Returns the card's rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(card.rank(), Rank::Ace);
    /// ```
    pub fn rank(&self) -> Rank {
        self.rank
    }
    
    /// Returns the card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(card.suit(), Suit::Spades);
    /// ```
    pub fn suit(&self) -> Suit {
        self.suit
    }
    
    /// Checks if this card is exactly one rank higher than the other card.
    ///
    /// This is primarily used to determine valid moves in FreeCell,
    /// where cards in the tableau must be stacked in descending order.
    ///
    /// Note that this only checks rank, not suit or color.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::card::{Card, Rank, Suit};
    ///
    /// let higher = Card::new(Rank::Two, Suit::Spades);
    /// let lower = Card::new(Rank::Ace, Suit::Hearts);
    ///
    /// assert!(higher.is_one_higher_than(&lower));
    /// ```
    pub fn is_one_higher_than(&self, other: &Card) -> bool {
        self.rank as u8 == other.rank as u8 + 1
    }
}

/// Formats the card for display as "Rank of Suit".
///
/// # Examples
///
/// ```
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// assert_eq!(format!("{}", card), "Ace of Spades");
/// ```
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
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
        let card = Card {
            rank: Rank::Ace,
            suit,
        };
        assert_eq!(card.color(), expected_color);
    }

    #[rstest]
    #[case(Rank::Ace, Suit::Spades, Rank::Two, Suit::Spades, false)]
    #[case(Rank::Ace, Suit::Hearts, Rank::Ace, Suit::Hearts, false)]
    #[case(Rank::Three, Suit::Hearts, Rank::Two, Suit::Hearts, true)]
    #[case(Rank::Ten, Suit::Diamonds, Rank::Nine, Suit::Diamonds, true)]
    fn card_is_higher_than(
        #[case] rank1: Rank,
        #[case] suit1: Suit,
        #[case] rank2: Rank,
        #[case] suit2: Suit,
        #[case] expected: bool,
    ) {
        let card1 = Card::new(rank1, suit1);
        let card2 = Card::new(rank2, suit2);
        assert_eq!(expected, card1.is_one_higher_than(&card2));
    }
}
