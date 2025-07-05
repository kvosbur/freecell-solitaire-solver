use freecell_game_engine::r#move::Move;
use freecell_game_engine::GameState;
use std::collections::HashSet;
use std::time::Instant;

struct Counter {
    count: u64,
    start: Instant,
    cancel_flag: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

pub fn solve_with_cancel(
    game_state: GameState,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> bool {
    println!("Solving FreeCell game using strategy 1 with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    let mut visited = HashSet::new();
    let result = dfs(game_state, &mut path, &mut counter, &mut visited);
    if result {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
    }
    return result;
}

/// Attempts to solve the given FreeCell game state using recursive DFS.
pub fn solve(game: GameState) {
    println!("Solving FreeCell game using strategy 1...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    let mut visited = HashSet::new();
    if dfs(game, &mut path, &mut counter, &mut visited) {
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

fn dfs(
    game: GameState,
    path: &mut Vec<Action>,
    counter: &mut Counter,
    visited: &mut HashSet<GameState>,
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
    if path.len() > 200 {
        // Limit the depth to prevent excessive recursion
        return false;
    }
    if !visited.insert(game.clone()) {
        // Already visited this state
        return false;
    }
    let moves = game.get_available_moves();
    for m in moves {
        let mut cloned = game.clone();
        if cloned.execute_move(&m).is_ok() {
            path.push(m.clone());
            if dfs(cloned, path, counter, visited) {
                return true;
            }
            path.pop();
        }
    }
    counter.count += 1;
    if counter.count % 100000 == 0 {
        println!(
            "Checked {} game states, time:{:?}",
            counter.count,
            counter.start.elapsed()
        );
    }
    false
}
