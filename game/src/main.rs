//! FreeCell Solitaire Game
//! 
//! This application provides an interactive FreeCell solitaire game using the
//! shared game-engine library.

use freecell_game_engine::{GameState, Card, Suit};

fn main() {
    println!("FreeCell Game starting...");
    
    // Example of using the game engine
    let mut game_state = GameState::new();
    println!("Created new game state");
    
    // Example of accessing game components
    let tableau = &game_state.tableau;
    let freecells = &game_state.freecells;
    let foundations = &game_state.foundations;
    
    println!("Game components initialized:");
    println!("- Tableau: {} columns", tableau.column_count());
    println!("- FreeCells: {} empty cells", freecells.empty_cell_count());
    println!("- Foundations: {} piles", foundations.pile_count());
    
    // Example of creating a card using the shared library
    let card = Card { rank: 13, suit: Suit::Spades }; // King of Spades
    println!("Created card: {:?}", card);
    
    // TODO: Implement actual game interface (CLI, TUI, or GUI)
    println!("Game interface will be implemented here");
}
