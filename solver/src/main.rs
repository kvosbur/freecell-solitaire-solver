//! FreeCell Solitaire Solver
//!
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.
mod game_prep;
mod harness;
mod strategies;

use freecell_game_engine::generation::GameGenerator;
use strategies::strat2::solve;

fn do_benchmark() {
    let allowed_timeout_secs = 60 * 10; // 10 minutes
    let seed = 1;
    let mut move_count_to_undue: usize = 24;
    let mut game_generator = GameGenerator::new(seed);
    game_generator.generate();
    let solution = game_prep::get_game_solution(seed);

    while move_count_to_undue < solution.len() {
        let mut game_state = game_generator.game_state.clone();
        println!(
            "Trying to undue {} moves from solution of length {}",
            move_count_to_undue,
            solution.len()
        );
        let subset_moves_to_apply = solution[0..solution.len() - move_count_to_undue].to_vec();
        for m in &subset_moves_to_apply {
            game_state.execute_move(m).unwrap();
        }
        let result = harness::harness(game_state.clone(), allowed_timeout_secs);
        if result {
            println!("Succeeded with {} moves undone", move_count_to_undue);
            move_count_to_undue += 1;
        } else {
            println!("Failed with {} moves undone", move_count_to_undue);
            break;
        }
    }
    println!(
        "Benchmark completed. Last successful move count to undue: {}",
        move_count_to_undue - 1
    );
}

fn do_adhoc() {
    let seed = 1;
    let move_count_to_undue = 29; // Change this to test different scenarios
    let allowed_timeout_secs = 60 * 2;
    let mut game_generator = GameGenerator::new(seed);
    game_generator.generate();
    let solution = game_prep::get_game_solution(seed);
    let mut game_state = game_generator.game_state.clone();

    let subset_moves_to_apply = solution[0..solution.len() - move_count_to_undue].to_vec();
    for m in &subset_moves_to_apply {
        game_state.execute_move(m).unwrap();
    }
    println!("Game state generated for seed {}", seed);

    // Example of solving the game using strategy 1
    let result = harness::harness(game_state.clone(), allowed_timeout_secs);
    if result {
        println!(
            "Successfully solved the game with {} moves undone",
            move_count_to_undue
        );
    } else {
        println!(
            "Failed to solve the game with {} moves undone",
            move_count_to_undue
        );
    }
}

fn main() {
    println!("FreeCell Solver starting...");

    // Run benchmark to find the maximum number of moves that can be undone
    // do_benchmark();

    // Run adhoc test with a specific seed and move count to undue
    do_adhoc();
}
