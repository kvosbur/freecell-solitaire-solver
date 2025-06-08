//! Core library for FreeCell game logic.
//! 
//! This library provides the fundamental building blocks for a FreeCell solitaire game,
//! including card representations, game rules, and game state management.

pub mod card;
pub mod foundations;
pub mod freecells;
pub mod rules;
pub mod tableau;
pub mod game_state;

// Re-export commonly used types for convenience
pub use card::{Card, Suit, Color};
pub use foundations::Foundations;
pub use freecells::FreeCells;
pub use tableau::Tableau;
pub use game_state::GameState;
