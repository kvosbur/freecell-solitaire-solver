use crate::strategies::{SolverStrategy, SolverResult};
use freecell_game_engine::game_state::GameState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Runs the given strategy on the game state with a timeout.
/// Returns the SolverResult (solved, moves, stats).
pub fn harness(
    strategy: Arc<dyn SolverStrategy>,
    game_state: GameState,
    timeout_secs: u64,
) -> SolverResult {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag_thread = cancel_flag.clone();
    let strategy_clone = strategy.clone();
    let handle = thread::spawn(move || {
        strategy_clone.solve(game_state, cancel_flag_thread)
    });
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if handle.is_finished() {
            match handle.join() {
                Ok(result) => {
                    println!("Solve completed within timeout.");
                    return result;
                }
                Err(e) => {
                    println!("Error during solve: {:?}", e);
                    return SolverResult {
                        solved: false,
                        moves: vec![],
                        stats: Default::default(),
                    };
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("Timeout reached, requesting cancellation...");
    cancel_flag.store(true, Ordering::SeqCst);
    match handle.join() {
        Ok(result) => {
            println!("Solve completed (after timeout/cancel).");
            result
        }
        Err(e) => {
            println!("Error during solve: {:?}", e);
            SolverResult {
                solved: false,
                moves: vec![],
                stats: Default::default(),
            }
        }
    }
}
