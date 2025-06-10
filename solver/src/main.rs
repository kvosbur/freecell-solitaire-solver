//! FreeCell Solitaire Solver
//! 
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.

use freecell_game_engine::{GameState, Card, Suit};

fn main() {
    println!("FreeCell Solver starting...");
    
    // Example of using the game engine
    let game_state = GameState::new();
    println!("Created new game state");
    
    // Example of creating a card using the shared library
    let card = Card { rank: 1, suit: Suit::Hearts }; // Ace of Hearts
    println!("Created card: {:?}", card);
    
    // TODO: Implement actual solving algorithm
    println!("Solver logic will be implemented here");
}
