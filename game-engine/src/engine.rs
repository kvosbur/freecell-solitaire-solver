use crate::deal_algorithm;
use crate::game_state::GameState;
use crate::move_type::Move;

pub fn deal_game(seed: u32) -> GameState {
    deal_algorithm::deal_game(seed)
}

pub fn get_valid_moves(state: &GameState) -> Vec<Move> {
    // Return all valid moves for the given state
    unimplemented!("Get valid moves")
}

pub fn apply_move(state: &GameState, mv: &Move) -> Result<GameState, &'static str> {
    // Apply the move to a copy of the state, returning new state
    // Returns error if the move is invalid
    unimplemented!("Apply move")
}

pub fn is_winning_state(state: &GameState) -> bool {
    // Check if all cards are in foundations
    state.foundations.iter().all(|foundation| foundation.len() == 13)
}

// Helper function if you need to make a deep copy
pub fn clone_state(state: &GameState) -> GameState {
    state.clone() // Rust's Clone trait handles this nicely
}