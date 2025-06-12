use crate::engine::{GameState, Move, deal_game, get_valid_moves, apply_move, is_winning_state};
use rand::Rng;

pub struct GameEngine {
    current_state: GameState,
}

impl GameEngine {
    pub fn new(seed: Option<u32>) -> Self {
        let game_seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        GameEngine {
            current_state: deal_game(game_seed),
        }
    }
    
    pub fn get_state(&self) -> &GameState {
        &self.current_state
    }
    
    pub fn get_valid_moves(&self) -> Vec<Move> {
        get_valid_moves(&self.current_state)
    }
    
    pub fn apply_move(&mut self, mv: &Move) -> Result<(), &'static str> {
        match apply_move(&self.current_state, mv) {
            Ok(new_state) => {
                self.current_state = new_state;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
    
    pub fn is_winning_state(&self) -> bool {
        is_winning_state(&self.current_state)
    }
    
    pub fn reset(&mut self, seed: Option<u32>) {
        let game_seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        self.current_state = deal_game(game_seed);
    }
}