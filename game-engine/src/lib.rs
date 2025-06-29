//! Core library for FreeCell game logic.
//!
//! This library provides the fundamental building blocks for a FreeCell solitaire game,
//! including card representations, game rules, and game state management.

pub mod card;
pub mod foundations;
pub mod freecells;
pub mod game_state;
pub mod generation;
pub mod rules;
pub mod tableau;
pub mod engine;
pub mod action;

// Re-export commonly used types for convenience
pub use card::{Card, Color, Rank, Suit};
pub use foundations::Foundations;
pub use freecells::FreeCells;
pub use game_state::GameState;
pub use tableau::Tableau;
pub use engine::{deal_game, get_valid_moves, apply_move, is_winning_state};
