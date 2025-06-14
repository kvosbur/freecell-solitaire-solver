//! FreeCell Solitaire Solver
//!
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.
mod game_prep;
mod strategies;

use freecell_game_engine::generation::GameGenerator;
use strategies::strat1::solve;

fn main() {
    let seed = 1;
    let move_count_to_undue: usize = 20;
    let mut game_generator = GameGenerator::new(seed);
    game_generator.generate();
    let mut game_state = game_generator.game_state.clone();
    let solution = game_prep::get_game_solution(seed);
    let subset_moves_to_apply = solution[0..solution.len() - move_count_to_undue].to_vec();

    for m in &subset_moves_to_apply {
        game_state.execute_move(m).unwrap();
    }
    println!("Game state after applying solution moves: {:?}", game_state);
    println!("Number of moves applied: {}", subset_moves_to_apply.len());
    println!("Number of moves in solution: {}", solution.len());

    println!("FreeCell Solver starting...");

    solve::solve(game_state);
}
