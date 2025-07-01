use freecell_game_engine::{r#move::Move, GameState};
use crate::packed_state::PackedGameState;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::time::Instant;

struct Counter {
    count: u64,
    start: Instant,
    cancel_flag: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

/// Attempts to solve the given FreeCell game state using recursive DFS with LRU cache for visited states.
fn dfs(
    game: &mut GameState,
    path: &mut Vec<Move>,
    counter: &mut Counter,
    visited: &mut LruCache<PackedGameState, ()>,
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
    let packed = PackedGameState::from_game_state(game);
    if visited.put(packed, ()).is_some() {
        // Already visited this state (recently)
        return false;
    }
    let moves = game.get_available_moves();
    for m in moves {
        if game.execute_move(&m).is_ok() {
            path.push(m.clone());
            if dfs(game, path, counter, visited) {
                return true;
            }
            path.pop();
            game.undo_move(&m);
        }
    }
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
    println!("Solving FreeCell game using strategy 5 (LRU) with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    // Set LRU cache size here (e.g., 10 million entries)
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let mut visited = LruCache::new(lru_size);
    let result = dfs(&mut game_state, &mut path, &mut counter, &mut visited);
    if result {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
    }
    return result;
}

pub fn solve(mut game: GameState) {
    println!("Solving FreeCell game using strategy 5 (LRU)...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    // Set LRU cache size here (e.g., 10 million entries)
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let mut visited = LruCache::new(lru_size);
    if dfs(&mut game, &mut path, &mut counter, &mut visited) {
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
