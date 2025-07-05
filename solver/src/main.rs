//! FreeCell Solitaire Solver
//!
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.
mod game_prep;
mod strategies;
pub mod packed_state;

use strategies::StrategyRegistry;
use std::sync::Arc;
use clap::{Arg, Command};
mod harness;
use freecell_game_engine::generation::generate_deal;

fn do_benchmark(strategy_name: &str, timeout_secs: u64) {
    let seed = 1;
    let mut move_count_to_undue: usize = 32;
    let game_state_initial = generate_deal(seed).unwrap();
    let solution = game_prep::get_game_solution(seed);

    let registry = StrategyRegistry::auto_discover();
    let strategy = registry.get_strategy(strategy_name)
        .unwrap_or_else(|_| panic!("Strategy '{}' not found", strategy_name));

    while move_count_to_undue < solution.len() {
        let mut game_state = game_state_initial.clone();
        println!(
            "Trying to undue {} moves from solution of length {}",
            move_count_to_undue,
            solution.len()
        );
        let subset_moves_to_apply = solution[0..solution.len() - move_count_to_undue].to_vec();
        for m in &subset_moves_to_apply {
            game_state.execute_move(m).unwrap();
        }
        let result = harness::harness(strategy.clone(), game_state.clone(), timeout_secs);
        if result.solved {
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

fn do_adhoc(strategy_name: &str, timeout_secs: u64) {
    let seed = 1;
    let move_count_to_undue = 40; // Change this to test different scenarios
    let game_state_initial = generate_deal(seed).unwrap();
    let solution = game_prep::get_game_solution(seed);
    let mut game_state = game_state_initial.clone();

    let subset_moves_to_apply = solution[0..solution.len() - move_count_to_undue].to_vec();
    for m in &subset_moves_to_apply {
        game_state.execute_move(m).unwrap();
    }
    println!("Game state generated for seed {}", seed);

    let registry = StrategyRegistry::auto_discover();
    let strategy = registry.get_strategy(strategy_name)
        .unwrap_or_else(|_| panic!("Strategy '{}' not found", strategy_name));
    let result = harness::harness(strategy, game_state.clone(), timeout_secs);
    if result.solved {
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
    let matches = Command::new("freecell-solver")
        .version("1.0")
        .about("FreeCell Solitaire Solver with pluggable strategies")
        .arg(
            Arg::new("strategy")
                .short('s')
                .long("strategy")
                .value_name("STRATEGY")
                .help("Strategy to use (strat1, strat2, strat3, strat4, strat5)")
                .default_value("strat5")
        )
        .arg(
            Arg::new("list-strategies")
                .long("list-strategies")
                .help("List available strategies")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout in seconds")
                .default_value("86400") // 24 hours
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .help("Run benchmark mode")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let registry = StrategyRegistry::auto_discover();

    // Handle list strategies
    if matches.get_flag("list-strategies") {
        println!("Available strategies:");
        for (name, description) in registry.list_strategies() {
            println!("  {}: {}", name, description);
        }
        return;
    }

    // Set strategy
    let strategy_name = matches.get_one::<String>("strategy").unwrap();
    println!("Using strategy: {}", strategy_name);

    // Get timeout
    let timeout_secs: u64 = matches.get_one::<String>("timeout")
        .unwrap()
        .parse()
        .expect("Invalid timeout value");

    if matches.get_flag("benchmark") {
        do_benchmark(strategy_name, timeout_secs);
    } else {
        do_adhoc(strategy_name, timeout_secs);
    }
}
