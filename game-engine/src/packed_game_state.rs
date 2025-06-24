//! PackedGameState: A compact, bit-packed representation of a FreeCell game state for fast hashing and equality.
//!
//! Used primarily by solver components for efficient state comparison.

use crate::game_state::GameState;
use crate::card::{Card, Rank, Suit};
use crate::tableau::Tableau;
use crate::freecells::FreeCells;
use crate::foundations::Foundations;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PackedGameState {
    // 52 cards, 6 bits each (0 = empty, 1-52 = card id)
    tableau_cards: [u8; 52], // 0 means unused slot
    tableau_lens: [u8; 8],   // Number of cards in each column
    freecells: [u8; 4],      // 0 = empty, 1-52 = card id
    foundations: [u8; 4],    // Top rank in each foundation (0 = empty, 1-13)
}

/// Error type for unpacking a PackedGameState
#[derive(Debug)]
pub enum UnpackError {
    InvalidCardId(u8),
    InvalidRank(u8),
    InvalidSuit(u8),
    InvalidTableauLength,
    InvalidFoundationRank(u8),
    NotEnoughTableauCards,
    TooManyTableauCards,
}

fn unpack_card(id: u8) -> Result<Card, UnpackError> {
    if id == 0 || id > 52 {
        return Err(UnpackError::InvalidCardId(id));
    }
    let id = id - 1;
    let suit = Suit::try_from(id / 13).map_err(|_| UnpackError::InvalidSuit(id / 13))?;
    let rank = Rank::try_from((id % 13) + 1).map_err(|_| UnpackError::InvalidRank((id % 13) + 1))?;
    Ok(Card { rank, suit })
}

impl PackedGameState {
    /// Convert a PackedGameState into a GameState
    pub fn to_game_state(&self) -> Result<GameState, UnpackError> {
        // Tableau
        let mut tableau = Tableau::new();
        let mut idx = 0;
        for col in 0..8 {
            let len = self.tableau_lens[col] as usize;
            if idx + len > self.tableau_cards.len() {
                return Err(UnpackError::NotEnoughTableauCards);
            }
            for i in 0..len {
                let card_id = self.tableau_cards[idx];
                let card = unpack_card(card_id)?;
                tableau.place_card(col, card).map_err(|_| UnpackError::InvalidTableauLength)?;
                idx += 1;
            }
        }
        if idx > self.tableau_cards.len() {
            return Err(UnpackError::TooManyTableauCards);
        }

        // FreeCells
        let mut freecells = FreeCells::new();
        for i in 0..4 {
            let card_id = self.freecells[i];
            if card_id != 0 {
                let card = unpack_card(card_id)?;
                freecells.place_card(i, card).map_err(|_| UnpackError::InvalidCardId(card_id))?;
            }
        }

        // Foundations
        let mut foundations = Foundations::new();
        for i in 0..4 {
            let top_rank = self.foundations[i];
            if top_rank > 13 {
                return Err(UnpackError::InvalidFoundationRank(top_rank));
            }
            if top_rank > 0 {
                let suit = Suit::try_from(i as u8).map_err(|_| UnpackError::InvalidSuit(i as u8))?;
                for r in 1..=top_rank {
                    let rank = Rank::try_from(r).map_err(|_| UnpackError::InvalidRank(r))?;
                    let card = Card { rank, suit };
                    foundations.place_card(i, card).map_err(|_| UnpackError::InvalidFoundationRank(top_rank))?;
                }
            }
        }

        let mut gs = GameState::new();
        // Overwrite tableau
        *gs.tableau_mut() = tableau;
        *gs.freecells_mut() = freecells;
        *gs.foundations_mut() = foundations;
        Ok(gs)
    }
}

impl PackedGameState {
    /// Convert a GameState into a PackedGameState
    pub fn from_game_state(gs: &GameState) -> Self {
        let mut tableau_cards = [0u8; 52];
        let mut tableau_lens = [0u8; 8];
        let mut idx = 0;
        for col in 0..gs.tableau().column_count() {
            let len = gs.tableau().column_length(col);
            tableau_lens[col] = len as u8;
            for i in 0..len {
                if let Some(card) = gs.tableau().get_card_at(col, i) {
                    tableau_cards[idx] = pack_card(card);
                    idx += 1;
                }
            }
        }
        let mut freecells = [0u8; 4];
        for i in 0..gs.freecells().cell_count() {
            freecells[i] = gs.freecells().get_card(i).map_or(0, |c| pack_card(c));
        }
        let mut foundations = [0u8; 4];
        for i in 0..gs.foundations().pile_count() {
            foundations[i] = gs.foundations().get_card(i).map_or(0, |c| c.rank as u8);
        }
        PackedGameState {
            tableau_cards,
            tableau_lens,
            freecells,
            foundations,
        }
    }
}

/// Packs a card into a 1-based id: 1..52 (0 = empty)
fn pack_card(card: &Card) -> u8 {
    let suit = card.suit as u8; // 0..3
    let rank = card.rank as u8; // 1..13
    suit * 13 + rank // 1..52
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameState;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn round_trip_default_state() {
        let original = GameState::default();
        let packed = PackedGameState::from_game_state(&original);
        let unpacked = packed.to_game_state().unwrap();
        assert_eq!(original, unpacked, "Default state should round-trip");
    }

    #[test]
    fn round_trip_complex_state() {
        let mut gs = GameState::default();
        // Place cards in tableau
        let card1 = Card { rank: Rank::Ace, suit: Suit::Hearts };
        let card2 = Card { rank: Rank::King, suit: Suit::Spades };
        gs.tableau_mut().place_card(0, card1).unwrap();
        gs.tableau_mut().place_card(1, card2).unwrap();
        // Place card in freecell
        let card3 = Card { rank: Rank::Queen, suit: Suit::Diamonds };
        gs.freecells_mut().place_card(0, card3).unwrap();
        // Build up a foundation
        for r in 1..=3 {
            let rank = Rank::try_from(r).unwrap();
            let card = Card { rank, suit: Suit::Diamonds };
            gs.foundations_mut().place_card(2, card).unwrap();
        }
        let packed = PackedGameState::from_game_state(&gs);
        let unpacked = packed.to_game_state().unwrap();
        assert_eq!(gs, unpacked, "Complex state should round-trip");
    }

    #[test]
    fn error_on_invalid_card_id() {
        let mut packed = PackedGameState::from_game_state(&GameState::default());
        packed.tableau_cards[0] = 99; // Invalid card id
        packed.tableau_lens[0] = 1; // Ensure column 0 has one card
        let result = packed.to_game_state();
        assert!(matches!(result, Err(UnpackError::InvalidCardId(99))));
    }

    #[test]
    fn error_on_invalid_foundation_rank() {
        let mut packed = PackedGameState::from_game_state(&GameState::default());
        packed.foundations[0] = 42; // Invalid foundation rank
        let result = packed.to_game_state();
        assert!(matches!(result, Err(UnpackError::InvalidFoundationRank(42))));
    }
}
