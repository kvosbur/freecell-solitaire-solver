use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState};
use freecell_game_engine::game_state::heuristics::score_state;
use lru::LruCache;
use std::collections::HashSet;
use std::num::NonZeroUsize;
use std::time::Instant;

struct Counter {
    count: u64,
    start: Instant,
    cancel_flag: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

fn dfs(
    game: &mut GameState,
    path: &mut Vec<Move>,
    counter: &mut Counter,
    ancestors: &mut HashSet<PackedGameState>,
    visited: &mut [LruCache<PackedGameState, ()>],
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
    let score = score_state(game);
    if score != 0 && path.len() > 200 {
        // Limit the depth to prevent excessive recursion
        return false;
    }

    let packed = PackedGameState::from_game_state_canonical(game);
    if ancestors.contains(&packed) {
        return false;
    }

    if score > 0 {
        let idx = score as usize;
        if visited[idx].contains(&packed) {
            return false;
        }
        visited[idx].put(packed.clone(), ());
    }

    ancestors.insert(packed.clone());

    let moves = if score == 0 {
        let mut moves = Vec::new();
        game.get_tableau_to_foundation_moves(&mut moves);
        game.get_freecell_to_foundation_moves(&mut moves);
        if moves.is_empty() {
            println!("{}", game);
            // Abort the process if no moves are available
            std::process::exit(1);
        }
        moves
    } else {
        game.get_available_moves()
    };
    for m in moves {
        if game.execute_move(&m).is_ok() {
            path.push(m.clone());
            if dfs(game, path, counter, ancestors, visited) {
                ancestors.remove(&packed);
                return true;
            }
            path.pop();
            game.undo_move(&m);
        }
    }

    ancestors.remove(&packed);

    counter.count += 1;
    if counter.count % 1000000 == 0 {
        println!(
            "Checked {} game states, time:{:?}, current score: {}",
            counter.count,
            counter.start.elapsed(),
            score
        );
    }
    false
}

pub fn solve_with_cancel(
    mut game_state: GameState,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> bool {
    println!("Solving FreeCell game using strategy 8 (Heuristic-bucketed LRU cache) with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    let mut ancestors = HashSet::new();
    let lru_size = NonZeroUsize::new(250_000).unwrap();
    let start_score = score_state(&game_state);
    println!("Starting score: {}", start_score);
    let mut visited: Vec<LruCache<PackedGameState, ()>> = (0..=start_score).map(|_| LruCache::new(lru_size)).collect();

    let result = dfs(&mut game_state, &mut path, &mut counter, &mut ancestors, &mut visited);
    if result {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
    } else {
        println!("Final game state:\n{}", game_state);
    }
    println!(
        "Checked {} game states, at end time:{:?}",
        counter.count,
        counter.start.elapsed()
    );
    return result;
}

pub fn solve(mut game: GameState) {
    println!("Solving FreeCell game using strategy 8 (Heuristic-bucketed LRU cache)...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    let mut ancestors = HashSet::new();
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let start_score = score_state(&game);
    let mut visited: Vec<LruCache<PackedGameState, ()>> = (0..=start_score).map(|_| LruCache::new(lru_size)).collect();

    if dfs(&mut game, &mut path, &mut counter, &mut ancestors, &mut visited) {
        println!(
            "Solution found! {:?} moves {:?} time",
            path.len(),
            counter.start.elapsed()
        );
    } else {
        println!("No solution found.");
    }
}
