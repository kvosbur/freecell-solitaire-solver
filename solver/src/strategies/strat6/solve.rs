use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState};
use std::collections::HashSet;
use std::time::Instant;

struct Counter {
    count: u64,
    start: Instant,
    cancel_flag: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

/// Attempts to solve the given FreeCell game state using recursive DFS with ancestor tracking.
/// Only tracks states from the current path (ancestors) to prevent cycles, allowing revisiting
/// states from other branches that may now be reachable with fewer moves or different context.
fn dfs(
    game: &mut GameState,
    path: &mut Vec<Move>,
    counter: &mut Counter,
    ancestors: &mut HashSet<PackedGameState>,
) -> bool {
    if counter
        .cancel_flag
        .as_ref()
        .map_or(false, |flag| flag.load(std::sync::atomic::Ordering::SeqCst))
    {
        return false;
    }
    if game.is_won().unwrap_or(false) {
        return true;
    }
    if path.len() > 86 {
        // Limit the depth to prevent excessive recursion
        return false;
    }
    
    let packed = PackedGameState::from_game_state(game);
    
    // Check if this state is already in our current path (would create a cycle)
    if ancestors.contains(&packed) {
        return false;
    }
    
    // Add current state to ancestors before exploring children
    ancestors.insert(packed.clone());
    
    let moves = game.get_available_moves();
    for m in moves {
        if game.execute_move(&m).is_ok() {
            path.push(m.clone());
            if dfs(game, path, counter, ancestors) {
                // Remove from ancestors before returning success
                ancestors.remove(&packed);
                return true;
            }
            path.pop();
            game.undo_move(&m);
        } else {
            println!("Failed to execute move: {:?}", m);
        }
    }
    
    // Remove current state from ancestors when backtracking
    ancestors.remove(&packed);
    
    counter.count += 1;
    if counter.count % 1000000 == 0 {
        println!(
            "Checked {} game states, time:{:?}",
            counter.count,
            counter.start.elapsed()
        );
    }
    false
}

pub fn solve_with_cancel(
    mut game_state: GameState,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> bool {
    println!("Solving FreeCell game using strategy 6 (Ancestor tracking) with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    let result = dfs(&mut game_state, &mut path, &mut counter, &mut ancestors);
    if result {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
    }
    println!(
        "Checked {} game states, at end time:{:?}",
        counter.count,
        counter.start.elapsed()
    );
    return result;
}

pub fn solve(mut game: GameState) {
    println!("Solving FreeCell game using strategy 6 (Ancestor tracking)...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    if dfs(&mut game, &mut path, &mut counter, &mut ancestors) {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
        // for m in path {
        //     println!("{:?}", m);
        // }
    } else {
        println!("No solution found.");
    }
}
