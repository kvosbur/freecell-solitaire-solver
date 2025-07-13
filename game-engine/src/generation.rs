//! Deal generation for FreeCell games, specifically implementing the Microsoft FreeCell
//! deal algorithm for compatibility with existing solvers and game implementations.
//!
//! This module provides a single public function, [`generate_deal`], which takes a
//! seed (corresponding to the "deal number" in Microsoft FreeCell) and returns a
//! [`GameState`] representing the initial layout of the cards.
//!
//! The deal generation process strictly adheres to the algorithm used in Microsoft FreeCell,
//! ensuring that the generated game states are bit-for-bit compatible with those produced
//! by the original game and other solvers that implement the same algorithm. This is crucial
//! for comparing solver performance and validating solutions across different implementations.
//!
//! # Algorithm Details
//!
//! The core of the algorithm involves:
//! 1. A custom Linear Congruential Generator (LCG) with specific parameters.
//! 2. A Fisher-Yates shuffle variant that uses the LCG to determine card positions.
//! 3. A fixed distribution pattern of shuffled cards into the 8 tableau columns.
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::generation::{generate_deal, GenerationError};
//! use freecell_game_engine::game_state::GameState;
//!
//! // Generate the famous Microsoft FreeCell deal #1
//! let game_state: GameState = generate_deal(1).unwrap();
//!
//! // Generate deal #617, known for being interesting
//! let game_state_617: GameState = generate_deal(617).unwrap();
//!
//! // Attempt to generate a deal with an invalid seed (e.g., 0)
//! let error = generate_deal(0);
//! assert!(matches!(error, Err(GenerationError::InvalidSeed)));
//! ```

use crate::{Card, GameState, Rank, Suit};
use std::fmt;

/// Error type for deal generation operations.
///
/// This enum represents all the possible error conditions that can occur
/// during the generation of a FreeCell deal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerationError {
    /// Attempted to generate a deal with an invalid seed.
    ///
    /// Microsoft FreeCell deals typically use seeds from 1 to 32000.
    InvalidSeed,
    /// An unexpected error occurred during the deal generation process.
    ///
    /// This error indicates a logical flaw in the generation algorithm itself,
    /// rather than an invalid input.
    DealGenerationFailed,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerationError::InvalidSeed => write!(f, "Invalid seed provided for deal generation. Seeds must be positive integers (e.g., 1-32000 for Microsoft FreeCell compatibility)."),
            GenerationError::DealGenerationFailed => write!(f, "An internal error occurred during deal generation. This indicates a bug in the generation algorithm."),
        }
    }
}

impl std::error::Error for GenerationError {}

// Constants for the Microsoft FreeCell Linear Congruential Generator (LCG)
// These values are critical for ensuring bit-perfect compatibility with Microsoft FreeCell.
const LCG_MULTIPLIER: u64 = 214013;
const LCG_INCREMENT: u64 = 2531011;
const LCG_MODULUS: u64 = 2u64.pow(31); // 2^31
const LCG_DIVISOR: u64 = 2u64.pow(16); // 2^16

/// A private Linear Congruential Generator (LCG) implementation
/// that mimics the random number generation used in Microsoft FreeCell.
///
/// This RNG is specifically designed to reproduce the exact sequence of
/// "random" numbers that the original Microsoft FreeCell game uses for
/// shuffling cards. Its parameters are fixed to ensure compatibility.
struct MicrosoftRng {
    state: u64,
}

impl MicrosoftRng {
    /// Creates a new `MicrosoftRng` instance with the given seed.
    ///
    /// The seed is used to initialize the internal state of the LCG.
    ///
    /// # Arguments
    /// * `seed` - The initial seed value for the RNG.
    fn new(seed: u64) -> Self {
        MicrosoftRng { state: seed }
    }

    /// Generates the next "random" value in the sequence.
    ///
    /// This method applies the specific LCG formula used by Microsoft FreeCell.
    /// The returned value is in the range [0, 32767] (0 to 2^15 - 1).
    fn next_value(&mut self) -> u64 {
        self.state = (self.state * LCG_MULTIPLIER + LCG_INCREMENT) % LCG_MODULUS;
        self.state / LCG_DIVISOR
    }
}

/// Creates a standard 52-card deck in a predefined sorted order.
///
/// The order of cards in this initial deck is important for reproducing
/// the exact shuffle behavior of the Microsoft FreeCell algorithm.
///
/// # Returns
/// A `Vec<Card>` containing all 52 cards, sorted by suit then rank.
fn create_standard_deck() -> Vec<Card> {
    let mut cards = Vec::with_capacity(52);
    for rank in [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ]
    .iter()
    {
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades].iter() {
            cards.push(Card::new(*rank, *suit));
        }
    }
    cards
}

/// Shuffles a deck of cards using the specific Fisher-Yates variant
/// implemented in Microsoft FreeCell.
///
/// This shuffle algorithm relies on the `MicrosoftRng` to generate
/// the indices for swapping cards, ensuring bit-perfect compatibility
/// with the original game's card distribution.
///
/// # Arguments
/// * `deck` - A mutable reference to the `Vec<Card>` to be shuffled.
/// * `rng` - A mutable reference to the `MicrosoftRng` instance used for randomness.
fn microsoft_shuffle(deck: &mut [Card], rng: &mut MicrosoftRng) {
    for i in (1..deck.len()).rev() {
        // The Microsoft algorithm uses a specific way to get the index:
        // it takes the next RNG value and mods it by (i + 1).
        let j = (rng.next_value() as usize) % (i + 1);
        deck.swap(i, j);
    }
}

/// Generates a Microsoft FreeCell compatible deal from a seed.
///
/// This function orchestrates the creation of a standard deck, shuffles it
/// using the Microsoft-specific algorithm, and then distributes the cards
/// into an 8-column tableau to form the initial `GameState`.
///
/// The `seed` parameter directly corresponds to the "deal number" found
/// in Microsoft FreeCell. Using the same seed will always produce the
/// identical game layout, which is essential for solver validation and
/// comparing results across different FreeCell implementations.
///
/// # Arguments
/// * `seed` - The seed value (deal number) for the generation. Must be a positive integer.
///
/// # Returns
/// A `Result` which is:
/// - `Ok(GameState)` containing the initial game layout if generation is successful.
/// - `Err(GenerationError)` if the seed is invalid or an internal error occurs.
///
/// # Errors
/// Returns `GenerationError::InvalidSeed` if the provided `seed` is 0.
/// Returns `GenerationError::DealGenerationFailed` if there's an unexpected issue
/// during the card distribution process (e.g., if the deck somehow becomes empty
/// prematurely, though this should not happen with a valid algorithm).
///
/// # Examples
///
/// ```
/// use freecell_game_engine::generation::{generate_deal, GenerationError};
/// use freecell_game_engine::game_state::GameState;
/// use freecell_game_engine::card::{Card, Rank, Suit};
///
/// // Generate the famous Microsoft FreeCell deal #1
/// let game_state: GameState = generate_deal(1).unwrap();
///
/// // Verify a card from deal #1 (e.g., the first card in column 0)
/// assert_eq!(game_state.tableau().get_card_at(0, 0).unwrap(), &Card::new(Rank::Jack, Suit::Diamonds));
///
/// // Attempt to generate a deal with an invalid seed (e.g., 0)
/// let error = generate_deal(0);
/// assert!(matches!(error, Err(GenerationError::InvalidSeed)));
/// ```
pub fn generate_deal(seed: u64) -> Result<GameState, GenerationError> {
    if seed == 0 {
        return Err(GenerationError::InvalidSeed);
    }

    let mut rng = MicrosoftRng::new(seed);
    let mut deck = create_standard_deck();
    microsoft_shuffle(&mut deck, &mut rng);

    let mut tableau = crate::tableau::Tableau::new();
    let mut column_idx = 0;
    let max_columns = 8;

    // Distribute cards into tableau columns
    while let Some(card) = deck.pop() {
        let location = crate::location::TableauLocation::new(column_idx as u8).unwrap();
        tableau.place_card_at_no_checks(location, card);

        column_idx = (column_idx + 1) % max_columns;
    }

    Ok(GameState::new_with_tableau(tableau))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Rank, Suit};

    #[test]
    fn test_game_1_layout() {
        let game = generate_deal(1).unwrap();

        // Verify the layout matches the expected game #1 layout

        // Define expected cards for each column
        let expected_layout = [
            // Column 0
            vec![
                Card::new(Rank::Jack, Suit::Diamonds), // J♦
                Card::new(Rank::King, Suit::Diamonds), // K♦
                Card::new(Rank::Two, Suit::Spades),    // 2♠
                Card::new(Rank::Four, Suit::Clubs),    // 4♣
                Card::new(Rank::Three, Suit::Spades),  // 3♠
                Card::new(Rank::Six, Suit::Diamonds),  // 6♦
                Card::new(Rank::Six, Suit::Spades),    // 6♠
            ],
            // Column 1
            vec![
                Card::new(Rank::Two, Suit::Diamonds), // 2♦
                Card::new(Rank::King, Suit::Clubs),   // K♣
                Card::new(Rank::King, Suit::Spades),  // K♠
                Card::new(Rank::Five, Suit::Clubs),   // 5♣
                Card::new(Rank::Ten, Suit::Diamonds), // 10♦
                Card::new(Rank::Eight, Suit::Spades), // 8♠
                Card::new(Rank::Nine, Suit::Clubs),   // 9♣
            ],
            // Column 2
            vec![
                Card::new(Rank::Nine, Suit::Hearts),    // 9♥
                Card::new(Rank::Nine, Suit::Spades),    // 9♠
                Card::new(Rank::Nine, Suit::Diamonds),  // 9♦
                Card::new(Rank::Ten, Suit::Spades),     // 10♠
                Card::new(Rank::Four, Suit::Spades),    // 4♠
                Card::new(Rank::Eight, Suit::Diamonds), // 8♦
                Card::new(Rank::Two, Suit::Hearts),     // 2♥
            ],
            // Column 3
            vec![
                Card::new(Rank::Jack, Suit::Clubs),     // J♣
                Card::new(Rank::Five, Suit::Spades),    // 5♠
                Card::new(Rank::Queen, Suit::Diamonds), // Q♦
                Card::new(Rank::Queen, Suit::Hearts),   // Q♥
                Card::new(Rank::Ten, Suit::Hearts),     // 10♥
                Card::new(Rank::Queen, Suit::Spades),   // Q♠
                Card::new(Rank::Six, Suit::Hearts),     // 6♥
            ],
            // Column 4
            vec![
                Card::new(Rank::Five, Suit::Diamonds), // 5♦
                Card::new(Rank::Ace, Suit::Diamonds),  // A♦
                Card::new(Rank::Jack, Suit::Spades),   // J♠
                Card::new(Rank::Four, Suit::Hearts),   // 4♥
                Card::new(Rank::Eight, Suit::Hearts),  // 8♥
                Card::new(Rank::Six, Suit::Clubs),     // 6♣
            ],
            // Column 5
            vec![
                Card::new(Rank::Seven, Suit::Hearts),   // 7♥
                Card::new(Rank::Queen, Suit::Clubs),    // Q♣
                Card::new(Rank::Ace, Suit::Spades),     // A♠
                Card::new(Rank::Ace, Suit::Clubs),      // A♣
                Card::new(Rank::Two, Suit::Clubs),      // 2♣
                Card::new(Rank::Three, Suit::Diamonds), // 3♦
            ],
            // Column 6
            vec![
                Card::new(Rank::Seven, Suit::Clubs),   // 7♣
                Card::new(Rank::King, Suit::Hearts),   // K♥
                Card::new(Rank::Ace, Suit::Hearts),    // A♥
                Card::new(Rank::Four, Suit::Diamonds), // 4♦
                Card::new(Rank::Jack, Suit::Hearts),   // J♥
                Card::new(Rank::Eight, Suit::Clubs),   // 8♣
            ],
            // Column 7
            vec![
                Card::new(Rank::Five, Suit::Hearts),    // 5♥
                Card::new(Rank::Three, Suit::Hearts),   // 3♥
                Card::new(Rank::Three, Suit::Clubs),    // 3♣
                Card::new(Rank::Seven, Suit::Spades),   // 7♠
                Card::new(Rank::Seven, Suit::Diamonds), // 7♦
                Card::new(Rank::Ten, Suit::Clubs),      // 10♣
            ],
        ];

        // Check that each column has the expected cards
        for (col_idx, expected_column) in expected_layout.iter().enumerate() {
            let location = crate::location::TableauLocation::new(col_idx as u8).unwrap();
            assert_eq!(
                game.tableau().column_length(location).unwrap(),
                expected_column.len(),
                "Column {} has wrong number of cards",
                col_idx
            );

            for (card_idx, expected_card) in expected_column.iter().enumerate() {
                assert_eq!(
                    game.tableau().get_card_at(location, card_idx).unwrap(),
                    expected_card,
                    "Mismatch at column {}, card {}",
                    col_idx,
                    card_idx
                );
            }
        }
    }

    #[test]
    fn test_additional_game_layouts() {
        // Test games known for being interesting
        let test_cases = [
            // Game #617
            (
                617,
                vec![
                    Card::new(Rank::Seven, Suit::Diamonds), // 7♦
                    Card::new(Rank::Ten, Suit::Diamonds),   // 10♦
                    Card::new(Rank::Ten, Suit::Hearts),     // 10♥
                    Card::new(Rank::King, Suit::Diamonds),  // K♦
                    Card::new(Rank::Four, Suit::Clubs),     // 4♣
                    Card::new(Rank::Four, Suit::Spades),    // 4♠
                    Card::new(Rank::Jack, Suit::Diamonds),  // J♦
                ],
            ),
            // Game #11982 (famously unsolvable with standard FreeCell rules)
            (
                11982,
                vec![
                    Card::new(Rank::Ace, Suit::Hearts),     // A♥
                    Card::new(Rank::Three, Suit::Diamonds), // 3♦
                    Card::new(Rank::King, Suit::Diamonds),  // K♦
                    Card::new(Rank::Jack, Suit::Clubs),     // J♣
                    Card::new(Rank::Six, Suit::Clubs),      // 6♣
                    Card::new(Rank::Jack, Suit::Diamonds),  // J♦
                    Card::new(Rank::King, Suit::Clubs),     // K♣
                ],
            ),
        ];

        for (seed, expected_column) in test_cases {
            let game = generate_deal(seed).unwrap();

            // Test just the first column
            let location = crate::location::TableauLocation::new(0).unwrap();
            assert_eq!(
                game.tableau().column_length(location).unwrap(),
                expected_column.len(),
                "Game #{} column 0 has wrong number of cards",
                seed
            );

            for (card_idx, expected_card) in expected_column.iter().enumerate() {
                assert_eq!(
                    game.tableau().get_card_at(location, card_idx).unwrap(),
                    expected_card,
                    "Game #{} mismatch at column 0, card {}",
                    seed,
                    card_idx
                );
            }
        }
    }
}
