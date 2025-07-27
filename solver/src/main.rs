//! FreeCell Solitaire Solver
//!
//! This application finds solutions to FreeCell solitaire games using the
//! shared game-engine library.
#![allow(dead_code)]
#![allow(unused)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod game_prep;
mod harness;
pub mod packed_state;
mod strategies;

use freecell_game_engine::generation::generate_deal;
use freecell_game_engine::r#move::Move;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use strategies::strat13::solve;

#[derive(Debug, Clone)]
pub struct SolverResult {
    pub solved: bool,
    pub solution_moves: Option<Vec<Move>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GameResult {
    seed: u64,
    solved: bool,
    execution_time_ms: u64,
    timestamp: String,
    move_count: Option<usize>, // None if not solved
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DetailedGameResult {
    seed: u64,
    solved: bool,
    execution_time_ms: u64,
    timestamp: String,
    solution_moves: Option<Vec<Move>>, // None if not solved
    move_count: Option<usize>, // None if not solved
}

#[derive(Serialize, Deserialize, Debug)]
struct BenchmarkResults {
    results: Vec<GameResult>,
    summary: BenchmarkSummary,
}

#[derive(Serialize, Deserialize, Debug)]
struct BenchmarkSummary {
    total_games: usize,
    solved_games: usize,
    failed_games: usize,
    average_time_ms: f64,
    timeout_secs: u64,
}

fn save_results_to_json(results: &Vec<GameResult>, filename: &str, timeout_secs: u64) {
    let solved_count = results.iter().filter(|r| r.solved).count();
    let failed_count = results.len() - solved_count;
    let avg_time = if !results.is_empty() {
        results.iter().map(|r| r.execution_time_ms as f64).sum::<f64>() / results.len() as f64
    } else {
        0.0
    };

    let summary = BenchmarkSummary {
        total_games: results.len(),
        solved_games: solved_count,
        failed_games: failed_count,
        average_time_ms: avg_time,
        timeout_secs,
    };

    let benchmark_results = BenchmarkResults {
        results: results.clone(),
        summary,
    };

    let json_string = serde_json::to_string_pretty(&benchmark_results).unwrap();
    fs::write(filename, json_string).expect("Failed to write JSON file");
    // println!("Results saved to {}", filename);
}

fn save_detailed_game_result(detailed_result: &DetailedGameResult, results_dir: &str) {
    // Create results directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(results_dir) {
        // println!("Warning: Failed to create results directory {}: {:?}", results_dir, e);
        return;
    }
    
    let filename = format!("{}/{}.json", results_dir, detailed_result.seed);
    let json_string = serde_json::to_string_pretty(detailed_result).unwrap();
    
    if let Err(e) = fs::write(&filename, json_string) {
        // println!("Warning: Failed to save detailed result for seed {}: {:?}", detailed_result.seed, e);
    }
}

fn load_existing_results(filename: &str) -> Vec<GameResult> {
    if let Ok(contents) = fs::read_to_string(filename) {
        if let Ok(benchmark_results) = serde_json::from_str::<BenchmarkResults>(&contents) {
            return benchmark_results.results;
        }
    }
    Vec::new()
}

fn do_seed_benchmark() {
    let allowed_timeout_secs = 120; // 2 minutes per game 
    let start_seed = 1u64;
    let max_seeds = 32000u64; // Test first 100 seeds
    let results_filename = "benchmark_summary.json";
    let results_dir = "results";
    
    // Load existing results if any
    let mut results = load_existing_results(results_filename);
    let mut processed_seeds: HashMap<u64, bool> = results.iter()
        .map(|r| (r.seed, true))
        .collect();
    
    println!("Starting seed benchmark (seeds {}-{}, timeout: {}s)", 
             start_seed, start_seed + max_seeds - 1, allowed_timeout_secs);
    // println!("Summary will be saved to: {}", results_filename);
    // println!("Detailed results will be saved to: {}/", results_dir);
    
    for seed in start_seed..start_seed + max_seeds {
        // Skip if already processed
        if processed_seeds.contains_key(&seed) {
            // println!("Seed {} already processed, skipping", seed);
            continue;
        }
        
        // println!("Testing seed {} ({}/{})...", seed, seed - start_seed + 1, max_seeds);
        
        let game_state = match generate_deal(seed) {
            Ok(state) => state,
            Err(e) => {
                println!("Failed to generate deal for seed {}: {:?}", seed, e);
                continue;
            }
        };
        
        let harness_result = harness::harness_with_timing(game_state, allowed_timeout_secs);
        let execution_time_ms = harness_result.execution_time.as_millis() as u64;
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        // Create summary result for the master file
        let summary_result = GameResult {
            seed,
            solved: harness_result.solved,
            execution_time_ms,
            timestamp: timestamp.clone(),
            move_count: harness_result.solution_moves.as_ref().map(|moves| moves.len()),
        };
        
        // Create detailed result for individual file
        let detailed_result = DetailedGameResult {
            seed,
            solved: harness_result.solved,
            execution_time_ms,
            timestamp,
            solution_moves: harness_result.solution_moves.clone(),
            move_count: harness_result.solution_moves.as_ref().map(|moves| moves.len()),
        };
        
        // Save detailed result to individual file
        save_detailed_game_result(&detailed_result, results_dir);
        
        results.push(summary_result);
        processed_seeds.insert(seed, true);
        
        if harness_result.solved {
            if let Some(ref moves) = harness_result.solution_moves {
                // println!("✓ Seed {} solved in {}ms with {} moves", seed, execution_time_ms, moves.len());
            } else {
                // println!("✓ Seed {} solved in {}ms", seed, execution_time_ms);
            }
        } else {
            // println!("✗ Seed {} failed/timeout after {}ms", seed, execution_time_ms);
        }
        
        // Print progress every 100 seeds
        if (seed - start_seed + 1) % 100 == 0 {
            println!("Progress: {} / {} seeds completed", seed - start_seed + 1, max_seeds);
        }
        
        // Save summary results after every 10 games or if this is the last one
        if results.len() % 10 == 0 || seed == start_seed + max_seeds - 1 {
            save_results_to_json(&results, results_filename, allowed_timeout_secs);
        }
    }
    
    // Final save and summary
    save_results_to_json(&results, results_filename, allowed_timeout_secs);
    
    let solved_count = results.iter().filter(|r| r.solved).count();
    println!("\n=== Benchmark Complete ===");
    println!("Total games tested: {}", results.len());
    println!("Games solved: {} ({:.1}%)", solved_count, 
             (solved_count as f64 / results.len() as f64) * 100.0);
    println!("Games failed/timeout: {}", results.len() - solved_count);
    println!("Summary saved to: {}", results_filename);
    println!("Detailed results saved to: {}/", results_dir);
}

fn do_benchmark() {
    // let allowed_timeout_secs = 60 * 60 * 24; // 24 hours
    let allowed_timeout_secs = 120; // 30 seconds
    let seed = 1;
    let mut move_count_to_undue: usize = 30;
    let game_state_initial = generate_deal(seed).unwrap();
    let solution = game_prep::get_game_solution(seed);
    println!("amount of moves in solution: {}", solution.len());

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
    let seed = 70;
    let allowed_timeout_secs = 60; // 24 hours
    let game_state = generate_deal(seed).unwrap();

    // Example of solving the game using strategy 1
    let harness_result = harness::harness_with_timing(game_state, allowed_timeout_secs);
    let execution_time_ms = harness_result.execution_time.as_millis() as u64;
    if harness_result.solved {
            if let Some(ref moves) = harness_result.solution_moves {
                println!("✓ Seed {} solved in {}ms with {} moves", seed, execution_time_ms, moves.len());
            } else {
                println!("✓ Seed {} solved in {}ms", seed, execution_time_ms);
            }
        } else {
            println!("✗ Seed {} failed/timeout after {}ms", seed, execution_time_ms);
        }
}

fn main() {
    println!("FreeCell Solver starting...");

    // Run new seed benchmark to test solver across multiple game seeds
    do_seed_benchmark();

    // Alternative benchmarks (commented out):
    // do_benchmark();  // Original benchmark testing move undoing
    // do_adhoc();      // Single seed testing
}
