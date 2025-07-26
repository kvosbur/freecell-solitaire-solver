use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState, location::Location};
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

/// Helper function to extract tableau column index from a location
fn get_tableau_column(location: &Location) -> Option<u8> {
    match location {
        Location::Tableau(tableau_loc) => Some(tableau_loc.index()),
        _ => None,
    }
}

/// Sorts moves to prefer moves from the same tableau column as the previous move
fn sort_moves_by_column_preference(moves: Vec<Move>, preferred_column: Option<u8>) -> Vec<Move> {
    if let Some(column) = preferred_column {
        let mut preferred_moves = Vec::new();
        let mut other_moves = Vec::new();
        
        for m in moves {
            if let Some(source_column) = get_tableau_column(&m.source) {
                if source_column == column {
                    preferred_moves.push(m);
                } else {
                    other_moves.push(m);
                }
            } else {
                other_moves.push(m);
            }
        }
        
        // Return preferred moves first, then others
        preferred_moves.extend(other_moves);
        preferred_moves
    } else {
        moves
    }
}

/// Attempts to solve the given FreeCell game state using recursive DFS that combines:
/// 1. Tableau column preference optimization from strategy 8
/// 2. Heuristic-bucketed LRU cache system from strategy 9
/// 3. Enhanced move selection based on game state scoring
fn dfs(
    game: &mut GameState,
    path: &mut Vec<Move>,
    counter: &mut Counter,
    ancestors: &mut HashSet<PackedGameState>,
    visited: &mut [LruCache<PackedGameState, ()>],
    previous_tableau_column: Option<u8>,
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
    
    // First check: Is this state in our current path? (Cycle detection)
    if ancestors.contains(&packed) {
        return false;
    }
    
    // Second check: Have we seen this state before in any path? (Heuristic-bucketed pruning)
    if score > 0 {
        let idx = score as usize;
        if visited[idx].contains(&packed) {
            return false;
        }
        visited[idx].put(packed.clone(), ());
    }
    
    // Add to ancestor tracking
    ancestors.insert(packed.clone());
    
    // Get moves based on game state score (strategy 9 approach)
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
    
    // Apply tableau column preference sorting (strategy 8 approach)
    let sorted_moves = sort_moves_by_column_preference(moves, previous_tableau_column);
    
    for m in sorted_moves {
        if game.execute_move(&m).is_ok() {
            path.push(m.clone());
            
            // Determine the new preferred column for the next iteration
            let next_preferred_column = get_tableau_column(&m.source);
            
            if dfs(game, path, counter, ancestors, visited, next_preferred_column) {
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
    println!("Solving FreeCell game using strategy 10 (Merged strat8+strat9: Tableau column preference + Heuristic-bucketed LRU cache) with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    // Use heuristic-bucketed LRU cache for efficient pruning
    let lru_size = NonZeroUsize::new(5_000_000).unwrap();
    let start_score = score_state(&game_state);
    println!("Starting score: {}", start_score);
    let mut visited: Vec<LruCache<PackedGameState, ()>> = (0..=start_score).map(|_| LruCache::new(lru_size)).collect();
    
    let result = dfs(&mut game_state, &mut path, &mut counter, &mut ancestors, &mut visited, None);
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
    println!("Solving FreeCell game using strategy 10 (Merged strat8+strat9: Tableau column preference + Heuristic-bucketed LRU cache)...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    // Use heuristic-bucketed LRU cache for efficient pruning
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let start_score = score_state(&game);
    println!("Starting score: {}", start_score);
    let mut visited: Vec<LruCache<PackedGameState, ()>> = (0..=start_score).map(|_| LruCache::new(lru_size)).collect();
    
    if dfs(&mut game, &mut path, &mut counter, &mut ancestors, &mut visited, None) {
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
