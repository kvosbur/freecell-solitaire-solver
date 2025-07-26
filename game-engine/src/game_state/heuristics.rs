//! Utility functions for evaluating heuristics on GameState.

use crate::game_state::GameState;
use crate::card::Card;

/// Calculates a heuristic score for the given game state.
/// 
/// This heuristic scores states based on the number of inversions in each tableau column,
/// where an inversion is a pair of cards that are out of order (i.e., a higher-ranked card
/// appears before a lower-ranked one).
pub fn score_state(state: &GameState) -> i32 {
    let mut score = 0;
    for column in state.tableau().columns() {
        for window in column.windows(2) {
            if let [first, second] = window {
                if second.rank() > first.rank() {
                    score += 1;
                }
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use crate::tableau::Tableau;
    use crate::freecells::FreeCells;
    use crate::foundations::Foundations;
    use crate::game_state::GameState;

    fn make_column(ranks: &[Rank]) -> Vec<Card> {
        ranks.iter().map(|&r| Card::new(r, Suit::Spades)).collect()
    }

    fn make_tableau_with_column(cards: &[Card], col_idx: u8) -> Tableau {
        let mut tableau = Tableau::new();
        for card in cards.iter() {
            tableau.place_card_at_no_checks(crate::location::TableauLocation::new(col_idx).unwrap(), *card);
        }
        tableau
    }

    #[test]
    fn test_score_state_empty_tableau() {
        let state = GameState::from_components(Tableau::new(), FreeCells::new(), Foundations::new());
        assert_eq!(score_state(&state), 0);
    }

    #[test]
    fn test_score_state_no_inversions() {
        // Descending order: King, Queen, Jack
        let cards = make_column(&[Rank::King, Rank::Queen, Rank::Jack]);
        let tableau = make_tableau_with_column(&cards, 0);
        let state = GameState::from_components(tableau, FreeCells::new(), Foundations::new());
        assert_eq!(score_state(&state), 0);
    }

    #[test]
    fn test_score_state_one_inversion() {
        // King, Jack, Queen (Jack < Queen, so one inversion)
        let cards = make_column(&[Rank::King, Rank::Jack, Rank::Queen]);
        let tableau = make_tableau_with_column(&cards, 0);
        let state = GameState::from_components(tableau, FreeCells::new(), Foundations::new());
        assert_eq!(score_state(&state), 1);
    }

    #[test]
    fn test_score_state_multiple_inversions() {
        // Jack, King, Queen (Jack < King, King > Queen, so one inversion)
        let cards = make_column(&[Rank::Jack, Rank::King, Rank::Queen]);
        let tableau = make_tableau_with_column(&cards, 0);
        let state = GameState::from_components(tableau, FreeCells::new(), Foundations::new());
        assert_eq!(score_state(&state), 1);
    }

    #[test]
    fn test_score_state_multiple_columns() {
        let cards1 = make_column(&[Rank::King, Rank::Queen, Rank::Jack]); // 0 inversions
        let cards2 = make_column(&[Rank::Ace, Rank::King, Rank::Queen]); // 1 inversion
        let mut tableau = Tableau::new();
        for card in &cards1 {
            tableau.place_card_at_no_checks(crate::location::TableauLocation::new(0).unwrap(), *card);
        }
        for card in &cards2 {
            tableau.place_card_at_no_checks(crate::location::TableauLocation::new(1).unwrap(), *card);
        }
        let state = GameState::from_components(tableau, FreeCells::new(), Foundations::new());
        assert_eq!(score_state(&state), 1);
    }
}
