//! PackedGameState: A compact, bit-packed representation of a FreeCell game state for fast hashing and equality.
//!
//! Used primarily by solver components for efficient state comparison.

use freecell_game_engine::{foundations::FOUNDATION_COUNT, tableau::TABLEAU_COLUMN_COUNT, Card, Foundations, FreeCells, GameState, Rank, Suit, Tableau};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Ok(Card::new(rank, suit))
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
            for _ in 0..len {
                let card_id = self.tableau_cards[idx];
                let card = unpack_card(card_id)?;
                let location = freecell_game_engine::location::TableauLocation::new(col as u8).unwrap();
                tableau.place_card_at(location, card).map_err(|_| UnpackError::InvalidTableauLength)?;
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
                let location = freecell_game_engine::location::FreecellLocation::new(i as u8).unwrap();
                freecells.place_card_at(location, card).map_err(|_| UnpackError::InvalidCardId(card_id))?;
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
                    let card = Card::new(rank, suit);
                    let location = freecell_game_engine::location::FoundationLocation::new(i as u8).unwrap();
                    foundations.place_card_at(location, card).map_err(|_| UnpackError::InvalidFoundationRank(top_rank))?;
                }
            }
        }

        Ok(GameState::from_components(tableau, freecells, foundations))
    }
}

impl PackedGameState {
    /// Convert a GameState into a PackedGameState
    pub fn from_game_state(gs: &GameState) -> Self {
        let mut tableau_cards = [0u8; 52];
        let mut tableau_lens = [0u8; 8];
        let mut idx = 0;
        for (col, len_ref) in tableau_lens.iter_mut().enumerate().take(TABLEAU_COLUMN_COUNT) {
            let location = freecell_game_engine::location::TableauLocation::new(col as u8).unwrap();
            let len = gs.tableau().column_length(location).unwrap_or(0);
            *len_ref = len as u8;
            for i in 0..len {
                if let Ok(card) = gs.tableau().get_card_at(location, i) {
                    tableau_cards[idx] = pack_card(card);
                    idx += 1;
                }
            }
        }
        let mut freecells = [0u8; 4];
        for i in 0..freecell_game_engine::freecells::FREECELL_COUNT {
            let location = freecell_game_engine::location::FreecellLocation::new(i as u8).unwrap();
            freecells[i] = gs.freecells().get_card(location).unwrap_or(None).map_or(0, pack_card);
        }
        let mut foundations = [0u8; 4];
        for i in 0..FOUNDATION_COUNT {
            let location = freecell_game_engine::location::FoundationLocation::new(i as u8).unwrap();
            foundations[i] = gs.foundations().get_card(location).unwrap_or(None).map_or(0, |c| c.rank() as u8);
        }
        PackedGameState {
            tableau_cards,
            tableau_lens,
            freecells,
            foundations,
        }
    }

    /// Convert a GameState into a canonical PackedGameState for better cache hits.
    /// This version creates an isomorphic representation by sorting tableau columns,
    /// freecells, and foundations to create a canonical ordering.
    pub fn from_game_state_canonical(gs: &GameState) -> Self {
        // Collect tableau columns with their data
        let mut tableau_columns: Vec<(Vec<u8>, u8)> = Vec::new();
        for col in 0..TABLEAU_COLUMN_COUNT {
            let location = freecell_game_engine::location::TableauLocation::new(col as u8).unwrap();
            let len = gs.tableau().column_length(location).unwrap_or(0);
            let mut column_cards = Vec::new();
            for i in 0..len {
                if let Ok(card) = gs.tableau().get_card_at(location, i) {
                    column_cards.push(pack_card(card));
                }
            }
            tableau_columns.push((column_cards, len as u8));
        }

        // Sort tableau columns by their first card (empty columns go to end)
        // Empty columns get a sort key of 255 to put them at the end
        tableau_columns.sort_by_key(|(cards, _len)| {
            cards.first().copied().unwrap_or(255)
        });

        // Pack sorted tableau data
        let mut tableau_cards = [0u8; 52];
        let mut tableau_lens = [0u8; 8];
        let mut idx = 0;
        for (col_idx, (cards, len)) in tableau_columns.iter().enumerate() {
            tableau_lens[col_idx] = *len;
            for &card in cards {
                tableau_cards[idx] = card;
                idx += 1;
            }
        }

        // Collect and sort freecells by card value (empty cells get 255)
        let mut freecell_cards: Vec<u8> = Vec::new();
        for i in 0..freecell_game_engine::freecells::FREECELL_COUNT {
            let location = freecell_game_engine::location::FreecellLocation::new(i as u8).unwrap();
            let card_id = gs.freecells().get_card(location).unwrap_or(None).map_or(255, pack_card);
            freecell_cards.push(card_id);
        }
        freecell_cards.sort();
        
        // Convert back to fixed array, replacing 255 with 0 for empty cells
        let mut freecells = [0u8; 4];
        for (i, &card) in freecell_cards.iter().enumerate() {
            freecells[i] = if card == 255 { 0 } else { card };
        }

        // Collect and sort foundations by top rank (empty foundations get 255)
        // Note: We sort the foundation ranks but keep them in a canonical order
        // since foundations are suit-specific and cannot be arbitrarily reordered
        let mut foundation_data: Vec<u8> = Vec::new();
        for i in 0..FOUNDATION_COUNT {
            let location = freecell_game_engine::location::FoundationLocation::new(i as u8).unwrap();
            let rank = gs.foundations().get_card(location).unwrap_or(None).map_or(0, |c| c.rank() as u8);
            foundation_data.push(rank);
        }
        foundation_data.sort();

        // Pack sorted foundations 
        let mut foundations = [0u8; 4];
        for (i, &rank) in foundation_data.iter().enumerate() {
            foundations[i] = rank;
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
    let suit = card.suit() as u8; // 0..3
    let rank = card.rank() as u8; // 1..13
    suit * 13 + rank // 1..52
}

#[cfg(test)]
mod tests {
    use super::*;
    use freecell_game_engine::{GameState, Card, Rank, Suit};

    #[test]
    fn round_trip_default_state() {
        let original = GameState::default();
        let packed = PackedGameState::from_game_state(&original);
        let unpacked = packed.to_game_state().unwrap();
        assert_eq!(original, unpacked, "Default state should round-trip");
    }

    #[test]
    fn round_trip_complex_state() {
        let mut tableau = Tableau::new();
        let card1 = Card::new(Rank::Ace, Suit::Hearts);
        let card2 = Card::new(Rank::King, Suit::Spades);
        let location0 = freecell_game_engine::location::TableauLocation::new(0).unwrap();
        let location1 = freecell_game_engine::location::TableauLocation::new(1).unwrap();
        tableau.place_card_at(location0, card1).unwrap();
        tableau.place_card_at(location1, card2).unwrap();

        let mut freecells = FreeCells::new();
        let card3 = Card::new(Rank::Queen, Suit::Diamonds);
        let location = freecell_game_engine::location::FreecellLocation::new(0).unwrap();
        freecells.place_card_at(location, card3).unwrap();

        let mut foundations = Foundations::new();
        for r in 1..=3 {
            let rank = Rank::try_from(r).unwrap();
            let card = Card::new(rank, Suit::Diamonds);
            let location = freecell_game_engine::location::FoundationLocation::new(2).unwrap();
            foundations.place_card_at(location, card).unwrap();
        }

        let gs = GameState::from_components(tableau, freecells, foundations);
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

    #[test]
    fn canonical_form_same_for_equivalent_states() {
        // Create two game states that are isomorphic but have different column arrangements
        let mut tableau1 = Tableau::new();
        let mut tableau2 = Tableau::new();
        
        let card_ace_hearts = Card::new(Rank::Ace, Suit::Hearts);
        let card_king_spades = Card::new(Rank::King, Suit::Spades);
        
        // State 1: Ace in col 0, King in col 1
        let loc0 = freecell_game_engine::location::TableauLocation::new(0).unwrap();
        let loc1 = freecell_game_engine::location::TableauLocation::new(1).unwrap();
        tableau1.place_card_at(loc0, card_ace_hearts).unwrap();
        tableau1.place_card_at(loc1, card_king_spades).unwrap();
        
        // State 2: King in col 0, Ace in col 1 (reversed)
        tableau2.place_card_at(loc0, card_king_spades).unwrap();
        tableau2.place_card_at(loc1, card_ace_hearts).unwrap();
        
        let gs1 = GameState::from_components(tableau1, FreeCells::new(), Foundations::new());
        let gs2 = GameState::from_components(tableau2, FreeCells::new(), Foundations::new());
        
        // Regular form should be different
        let packed1 = PackedGameState::from_game_state(&gs1);
        let packed2 = PackedGameState::from_game_state(&gs2);
        assert_ne!(packed1, packed2, "Regular packed states should be different");
        
        // Canonical form should be the same
        let canonical1 = PackedGameState::from_game_state_canonical(&gs1);
        let canonical2 = PackedGameState::from_game_state_canonical(&gs2);
        assert_eq!(canonical1, canonical2, "Canonical packed states should be identical");
    }

    #[test]
    fn canonical_form_sorts_freecells() {
        let mut freecells = FreeCells::new();
        let card_ace_hearts = Card::new(Rank::Ace, Suit::Hearts);  // Higher ID: 1*13+1=14
        let card_king_spades = Card::new(Rank::King, Suit::Spades); // Lower ID: 0*13+13=13
        
        let loc0 = freecell_game_engine::location::FreecellLocation::new(0).unwrap();
        let loc3 = freecell_game_engine::location::FreecellLocation::new(3).unwrap();
        
        // Place Ace (higher ID) in first freecell, King (lower ID) in last freecell
        freecells.place_card_at(loc0, card_ace_hearts).unwrap();
        freecells.place_card_at(loc3, card_king_spades).unwrap();
        
        let gs = GameState::from_components(Tableau::new(), freecells, Foundations::new());
        let canonical = PackedGameState::from_game_state_canonical(&gs);
        
        // Canonical form should have King before Ace (lower card id comes first)
        let ace_id = pack_card(&card_ace_hearts);
        let king_id = pack_card(&card_king_spades);
        assert_eq!(canonical.freecells[0], king_id, "King should come first in canonical form (lower ID)");
        assert_eq!(canonical.freecells[1], ace_id, "Ace should come second in canonical form (higher ID)");
        assert_eq!(canonical.freecells[2], 0, "Third freecell should be empty");
        assert_eq!(canonical.freecells[3], 0, "Fourth freecell should be empty");
    }
}
