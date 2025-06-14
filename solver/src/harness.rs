use crate::solve;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn harness(game_state: freecell_game_engine::game_state::GameState, timeout_secs: u64) -> bool {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag_thread = cancel_flag.clone();
    let handle = thread::spawn(move || {
        return solve::solve_with_cancel(game_state, cancel_flag_thread);
    });
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if handle.is_finished() {
            println!("Solve completed within timeout.");
            let _ = handle.join();
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("Timeout reached, requesting cancellation...");
    cancel_flag.store(true, Ordering::SeqCst);
    let result = handle.join();
    match result {
        Ok(val) => {
            println!("Solve completed: {:?}", val);
            return val;
        }
        Err(e) => {
            println!("Error during solve: {:?}", e);
            return false;
        }
    };
}
