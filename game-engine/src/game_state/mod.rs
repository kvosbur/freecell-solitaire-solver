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
    
    /// Returns an immutable reference to the game's tableau.
    ///
    /// The tableau consists of 8 columns where most of the cards are initially dealt.
    /// Cards are moved within and between tableau columns according to specific rules.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new();
    /// let tableau = game.tableau();
    /// // You can now inspect the tableau, e.g., tableau.column_count()
    /// ```
    pub fn tableau(&self) -> &Tableau { &self.tableau }

    /// Returns an immutable reference to the game's freecells.
    ///
    /// Freecells are 4 temporary storage slots where single cards can be placed.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new();
    /// let freecells = game.freecells();
    /// // You can now inspect the freecells, e.g., freecells.empty_cells_count()
    /// ```
    pub fn freecells(&self) -> &FreeCells { &self.freecells }

    /// Returns an immutable reference to the game's foundations.
    ///
    /// Foundations are 4 piles (one for each suit) where cards are built up
    /// from Ace to King. Completing all foundations wins the game.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new();
    /// let foundations = game.foundations();
    /// // You can now inspect the foundations, e.g., foundations.is_complete()
    /// ```
    pub fn foundations(&self) -> &Foundations { &self.foundations }
    
    /// Returns a mutable reference to the game's tableau.
    ///
    /// This method allows for direct modification of the tableau. Use with caution,
    /// as improper modifications can lead to an invalid game state.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// let tableau_mut = game.tableau_mut();
    /// // You can now modify the tableau, e.g., tableau_mut.place_card(...)
    /// ```
    pub fn tableau_mut(&mut self) -> &mut Tableau { &mut self.tableau }

    /// Returns a mutable reference to the game's freecells.
    ///
    /// This method allows for direct modification of the freecells. Use with caution,
    /// as improper modifications can lead to an invalid game state.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// let freecells_mut = game.freecells_mut();
    /// // You can now modify the freecells, e.g., freecells_mut.place_card(...)
    /// ```
    pub fn freecells_mut(&mut self) -> &mut FreeCells { &mut self.freecells }

    /// Returns a mutable reference to the game's foundations.
    ///
    /// This method allows for direct modification of the foundations. Use with caution,
    /// as improper modifications can lead to an invalid game state.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// let foundations_mut = game.foundations_mut();
    /// // You can now modify the foundations, e.g., foundations_mut.place_card(...)
    /// ```
    pub fn foundations_mut(&mut self) -> &mut Foundations { &mut self.foundations }
    
    /// Checks if the game has been won.
    ///
    /// A FreeCell game is won when all cards have been successfully moved
    /// to their respective foundation piles (i.e., all four foundation piles
    /// are complete from Ace to King for each suit).
    ///
    /// # Returns
    ///
    /// `true` if the game is won, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let mut game = GameState::new();
    /// // Initially, the game is not won
    /// assert!(!game.is_won());
    ///
    /// // (Imagine game state is manipulated to a winning state)
    /// // game.foundations_mut().force_complete_all_piles(); // Hypothetical method
    /// // assert!(game.is_won());
    /// ```
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
