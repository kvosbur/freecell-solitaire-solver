/*!
GameState: The core struct representing the state of a FreeCell game.

This module provides the main `GameState` struct, which encapsulates the tableau, freecells, and foundations.
It exposes methods for creating, inspecting, and manipulating the game state, following Rust best practices.

# Examples

```rust
use game_engine::game_state::GameState;

let mut state = GameState::default();
assert!(!state.is_game_won());
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

/// The full state of a FreeCell game, including tableau, freecells, and foundations.
///
/// Use [`GameState::new()`] or [`GameState::default()`] to create a new game.
/// 
/// # Examples
/// ```
/// let state = GameState::default();
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    /// The tableau: 8 columns of cards.
    pub tableau: Tableau,
    /// The freecells: 4 temporary storage cells.
    pub freecells: FreeCells,
    /// The foundations: 4 suit-based foundation piles.
    pub foundations: Foundations,
}

impl GameState {
    /// Creates a new, empty game state.
    ///
    /// Equivalent to [`GameState::default()`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if all foundation piles are complete (i.e., game is won).
    ///
    /// # Examples
    /// ```
    /// let state = GameState::default();
    /// assert!(!state.is_game_won());
    /// ```
    pub fn is_game_won(&self) -> bool {
        (0..self.foundations.pile_count()).all(|i| self.foundations.is_pile_complete(i))
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
