//! PackedGameState: A compact, bit-packed representation of a FreeCell game state for fast hashing and equality.
//!
//! Used primarily by solver components for efficient state comparison.

use crate::game_state::GameState;
use crate::card::Card;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PackedGameState {
    // 52 cards, 6 bits each (0 = empty, 1-52 = card id)
    tableau_cards: [u8; 52], // 0 means unused slot
    tableau_lens: [u8; 8],   // Number of cards in each column
    freecells: [u8; 4],      // 0 = empty, 1-52 = card id
    foundations: [u8; 4],    // Top rank in each foundation (0 = empty, 1-13)
}

impl PackedGameState {
    /// Convert a GameState into a PackedGameState
    pub fn from_game_state(gs: &GameState) -> Self {
        let mut tableau_cards = [0u8; 52];
        let mut tableau_lens = [0u8; 8];
        let mut idx = 0;
        for col in 0..gs.tableau.column_count() {
            let len = gs.tableau.column_length(col);
            tableau_lens[col] = len as u8;
            for i in 0..len {
                if let Some(card) = gs.tableau.get_card_at(col, i) {
                    tableau_cards[idx] = pack_card(card);
                    idx += 1;
                }
            }
        }
        let mut freecells = [0u8; 4];
        for i in 0..gs.freecells.cell_count() {
            freecells[i] = gs.freecells.get_card(i).map_or(0, |c| pack_card(c));
        }
        let mut foundations = [0u8; 4];
        for i in 0..gs.foundations.pile_count() {
            foundations[i] = gs.foundations.get_top_card(i).map_or(0, |c| c.rank as u8);
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
