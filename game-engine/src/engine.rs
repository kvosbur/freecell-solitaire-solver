use crate::game_state::GameState;
use crate::generation::GameGenerator;
use crate::action::Action;

pub fn deal_game(seed: u64) -> GameState {
    let mut generator = GameGenerator::new(seed);
    generator.generate();
    generator.game_state
}

pub fn get_valid_moves(state: &GameState) -> Vec<Action> {
    // Return all valid moves for the given state
    unimplemented!("Get valid moves")
}

pub fn apply_move(state: &GameState, mv: &Action) -> Result<GameState, &'static str> {
    // Apply the move to a copy of the state, returning new state
    // Returns error if the move is invalid
    unimplemented!("Apply move")
}

pub fn is_winning_state(state: &GameState) -> bool {
    // Check if all foundation piles are complete (13 cards each)
    (0..state.foundations.pile_count()).all(|i| state.foundations.is_pile_complete(i))
}
