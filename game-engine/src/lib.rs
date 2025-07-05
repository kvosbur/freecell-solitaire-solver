//! A pure, type-safe, and fast implementation of FreeCell solitaire game logic.
//!
//! This crate provides the core building blocks for creating a FreeCell game,
//! solver, or other related applications. It is designed to be:
//!
//! - **Pure**: Contains only game logic, with no UI, I/O, or solver-specific features.
//! - **Type-Safe**: Uses Rust's type system to prevent invalid game states and moves.
//! - **Well-Documented**: Includes comprehensive documentation with examples and explanations
//!   of FreeCell rules, making it accessible even to those unfamiliar with the game.
//!
//! ## What is FreeCell?
//!
//! FreeCell is a solitaire card game played with a standard 52-card deck. All cards are
//! dealt face-up from the start into 8 tableau columns. The goal is to move all cards
//! to the 4 foundation piles, building each suit up from Ace to King.
//!
//! ### Game Layout
//!
//! The game is laid out in three main areas:
//!
//! ```text
//! [FreeCells]    [Foundations]
//! [ ][ ][ ][ ]   [ ][ ][ ][ ]
//!
//! [Tableau Columns]
//! [A♠] [K♥] [Q♦] [J♣] ...
//! [2♠] [... ] [...] [...]
//! ...
//! ```
//!
//! - **The Tableau**: 8 columns that hold cards in play. Cards can be moved between
//!   columns in descending order with alternating colors.
//! - **The FreeCells**: 4 temporary storage slots at the top-left that can each hold
//!   a single card. These are crucial for maneuvering cards.
//! - **The Foundations**: 4 piles at the top-right where cards are moved to win the
//!   game. Each pile is built up by suit, from Ace to King.
//!
//! ## Key Concepts
//!
//! - **Move**: A transfer of one or more cards from one location to another.
//! - **Location**: A specific place in the game, such as a tableau column, freecell, or foundation pile.
//! - **GameState**: A snapshot of the entire game at a specific moment, including the state of
//!   the tableau, freecells, and foundations.
//!
//! # Getting Started
//!
//! To use this crate, you'll primarily interact with the [`GameState`] struct, which represents
//! the current state of the game. You can create a new game, inspect its state, and execute moves.
//!
//! # FreeCell Rules
//!
//! The goal of FreeCell is to move all 52 cards to the foundation piles, building up each suit from Ace to King.
//!
//! ## Card Movement
//!
//! - **Tableau to Tableau**: A card can be moved from the top of one tableau column to another if the destination card is one rank higher and of the opposite color. For example, a red 9 can be moved onto a black 10.
//! - **Tableau to FreeCell**: Any single card from the top of a tableau column can be moved to an empty freecell.
//! - **Tableau to Foundation**: The top card of a tableau column can be moved to a foundation pile if it is an Ace or one rank higher than the top card of the foundation pile of the same suit.
//! - **FreeCell to Tableau**: A card from a freecell can be moved to the top of a tableau column if the move is valid according to the tableau-to-tableau rules.
//! - **FreeCell to Foundation**: A card from a freecell can be moved to a foundation pile if the move is valid according to the tableau-to-foundation rules.
//!
//! ## Moving Multiple Cards
//!
//! While you can only move one card at a time, you can move a sequence of cards from one tableau column to another if you have enough empty freecells and/or tableau columns. The number of cards you can move is `(1 + number of empty freecells) * 2 ^ (number of empty tableau columns)`. This logic is not yet implemented in this crate.
//!
//! ```rust
//! use freecell_game_engine::{GameState, Move};
//! use freecell_game_engine::location::{TableauLocation, FreecellLocation};
//!
//! // Create a new game state (typically from a shuffled deal)
//! let mut game = GameState::new();
//!
//! // Define a move to transfer a card
//! let move_cmd = Move::tableau_to_freecell(0, 0).unwrap();
//!
//! // Validate and execute the move
//! // Note: In a real game, you would need to ensure the move is valid
//! // for the current state of the game.
//! // if let Err(err) = game.execute_move(&move_cmd) {
//! //    println!("Error executing move: {}", err);
//! // } else {
//! //   println!("Move executed successfully!");
//! // }
//! ```
//!
//! This crate provides the foundation for building more complex applications, such as a
//! graphical FreeCell game or an automated solver.

pub mod card;
pub mod foundations;
pub mod freecells;
pub mod game_state;
pub mod generation;
pub mod location;
pub mod tableau;
pub mod r#move;

// Re-export commonly used types for convenience
pub use card::{Card, Color, Rank, Suit};
pub use foundations::Foundations;
pub use freecells::FreeCells;
pub use game_state::GameState;
pub use tableau::Tableau;
pub use r#move::Move;
