use crate::solve;
use freecell_game_engine::r#move::Move;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct HarnessResult {
    pub solved: bool,
    pub execution_time: Duration,
    pub solution_moves: Option<Vec<Move>>,
}

pub fn harness(game_state: freecell_game_engine::game_state::GameState, timeout_secs: u64) -> bool {
    let result = harness_with_timing(game_state, timeout_secs);
    result.solved
}

pub fn harness_with_timing(game_state: freecell_game_engine::game_state::GameState, timeout_secs: u64) -> HarnessResult {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag_thread = cancel_flag.clone();
    let start_time = Instant::now();
    
    let handle = thread::spawn(move || {
        return solve::solve_with_cancel(game_state, cancel_flag_thread);
    });
    
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if handle.is_finished() {
            let execution_time = start_time.elapsed();
            println!("Solve completed within timeout in {:?}.", execution_time);
            match handle.join() {
                Ok(solver_result) => {
                    println!("Solve completed: {:?}", solver_result.solved);
                    return HarnessResult {
                        solved: solver_result.solved,
                        execution_time,
                        solution_moves: solver_result.solution_moves,
                    };
                }
                Err(e) => {
                    println!("Error during solve: {:?}", e);
                    return HarnessResult {
                        solved: false,
                        execution_time,
                        solution_moves: None,
                    };
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    let execution_time = start_time.elapsed();
    println!("Timeout reached, requesting cancellation...");
    cancel_flag.store(true, Ordering::SeqCst);
    let result = handle.join();
    match result {
        Ok(solver_result) => {
            println!("Solve completed: {:?}", solver_result.solved);
            return HarnessResult {
                solved: solver_result.solved,
                execution_time,
                solution_moves: solver_result.solution_moves,
            };
        }
        Err(e) => {
            println!("Error during solve: {:?}", e);
            return HarnessResult {
                solved: false,
                execution_time,
                solution_moves: None,
            };
        }
    };
}
