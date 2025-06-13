//! FreeCell Solitaire Solver
//!
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.
mod strategies;

use freecell_game_engine::generation::GameGenerator;
use strategies::strat1::solve;

fn main() {
    println!("FreeCell Solver starting...");

    let mut game_generator = GameGenerator::new(1);
    game_generator.generate();
    let game_state = game_generator.game_state.clone();
    solve::solve(game_state);
}
