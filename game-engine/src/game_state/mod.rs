/*!
GameState: The core struct representing the state of a FreeCell game.

This module provides the main `GameState` struct, which encapsulates the tableau, freecells, and foundations.
It exposes methods for creating, inspecting, and manipulating the game state, following Rust best practices.

# Examples

```rust
use freecell_game_engine::game_state::GameState;

let mut state = GameState::default();
assert!(!state.is_won());
```
*/

mod error;
mod validation;
mod execution;
mod moves;

pub use error::GameError;

use crate::tableau::Tableau;
use crate::freecells::FreeCells;
use crate::foundations::Foundations;

/// Represents the complete state of a FreeCell game
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    tableau: Tableau,
    freecells: FreeCells,
    foundations: Foundations,
}

impl GameState {
    /// Create a new game state with default components
    pub fn new() -> Self {
        Self {
            tableau: Tableau::new(),
            freecells: FreeCells::new(),
            foundations: Foundations::new(),
        }
    }
    
    // Accessor methods
    pub fn tableau(&self) -> &Tableau { &self.tableau }
    pub fn freecells(&self) -> &FreeCells { &self.freecells }
    pub fn foundations(&self) -> &Foundations { &self.foundations }
    
    // Mutable accessor methods (careful with these!)
    pub fn tableau_mut(&mut self) -> &mut Tableau { &mut self.tableau }
    pub fn freecells_mut(&mut self) -> &mut FreeCells { &mut self.freecells }
    pub fn foundations_mut(&mut self) -> &mut Foundations { &mut self.foundations }
    
    /// Check if the game is won
    pub fn is_won(&self) -> bool {
        self.foundations.is_complete()
    }
}

impl Default for GameState {
    /// Returns a new, empty game state.
    fn default() -> Self {
        Self {
            tableau: Tableau::new(),
            freecells: FreeCells::new(),
            foundations: Foundations::new(),
        }
    }
}
