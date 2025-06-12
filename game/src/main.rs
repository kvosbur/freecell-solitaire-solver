//! FreeCell Solitaire Game
//!
//! This application provides an interactive FreeCell solitaire game using the
//! shared game-engine library.

use freecell_game_engine::{generation::GameGenerator, Card, Suit, Rank};

fn main() {
    println!("FreeCell Game starting...");

    let mut game_generator = GameGenerator::new(1);
    game_generator.generate();

    // Example of using the game engine
    let game_state = game_generator.game_state;
    println!("Created new game state");

    // Example of accessing game components
    let tableau = &game_state.tableau;
    let freecells = &game_state.freecells;
    let foundations = &game_state.foundations;

    println!("Game components initialized:");
    println!("- Tableau: {:?}", tableau);
    println!("- FreeCells: {:?} empty cells", freecells);
    println!("- Foundations: {:?} piles", foundations);

    // Example of creating a card using the shared library
    let card = Card {
        rank: Rank::King,
        suit: Suit::Spades,
    }; // King of Spades
    println!("Created card: {:?}", card);

    // TODO: Implement actual game interface (CLI, TUI, or GUI)
    println!("Game interface will be implemented here");
}
