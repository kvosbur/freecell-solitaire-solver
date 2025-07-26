/*!
GameState: The core struct representing the state of a FreeCell game.

This module provides the main `GameState` struct, which encapsulates the tableau, freecells, and foundations.
It exposes methods for creating, inspecting, and manipulating the game state, following Rust best practices.

# Examples

```rust
use freecell_game_engine::game_state::GameState;

let mut state = GameState::default();
assert!(!state.is_won().unwrap());
```
*/

mod error;
mod validation;
mod execution;
mod moves;
mod heuristics;

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

    /// Create a new game state with a given tableau
    pub(crate) fn new_with_tableau(tableau: Tableau) -> Self {
        Self {
            tableau,
            freecells: FreeCells::new(),
            foundations: Foundations::new(),
        }
    }

    /// Create a new game state with given components
    pub fn from_components(tableau: Tableau, freecells: FreeCells, foundations: Foundations) -> Self {
        Self {
            tableau,
            freecells,
            foundations,
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
    /// assert!(!game.is_won().unwrap());
    ///
    /// // (Imagine game state is manipulated to a winning state)
    /// // let mut foundations = Foundations::new();
    /// // ... populate foundations ...
    /// // let game = GameState::from_components(Tableau::new(), FreeCells::new(), foundations);
    /// // assert!(game.is_won().unwrap());
    /// ```
    pub fn is_won(&self) -> Result<bool, GameError> {
        Ok(self.foundations.is_complete())
    }

    pub fn get_card(&self, location: crate::location::Location) -> Result<Option<&crate::card::Card>, GameError> {
        use crate::location::Location::*;
        match location {
            Tableau(l) => self.tableau.get_card(l).map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: None,
                operation: "get_card".to_string(),
            }),
            Freecell(l) => self.freecells.get_card(l).map_err(|e| GameError::FreeCell {
                error: e,
                attempted_move: None,
                operation: "get_card".to_string(),
            }),
            Foundation(l) => self.foundations.get_card(l).map_err(|e| GameError::Foundation {
                error: e,
                attempted_move: None,
                operation: "get_card".to_string(),
            }),
        }
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
