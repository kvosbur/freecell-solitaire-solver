use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState, location::Location};
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

/// Attempts to solve the given FreeCell game state using recursive DFS with both
/// ancestor tracking for cycle detection and LRU cache for efficient pruning.
/// Enhanced with tableau column preference - prefers moves from the same column
/// as the previous move to encourage working within the same tableau column.
fn dfs(
    game: &mut GameState,
    path: &mut Vec<Move>,
    counter: &mut Counter,
    ancestors: &mut HashSet<PackedGameState>,
    visited: &mut LruCache<PackedGameState, ()>,
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
    if path.len() > 200 {
        // Limit the depth to prevent excessive recursion
        return false;
    }
    
    let packed = PackedGameState::from_game_state_canonical(game);
    
    // First check: Is this state in our current path? (Cycle detection)
    if ancestors.contains(&packed) {
        return false;
    }
    
    // Second check: Have we seen this state before in any path? (Pruning optimization)
    if visited.contains(&packed) {
        return false;
    }
    
    // Add to both tracking structures
    ancestors.insert(packed.clone());
    visited.put(packed.clone(), ());
    
    let moves = game.get_available_moves();
    // Sort moves to prefer moves from the same tableau column as the previous move
    let sorted_moves = sort_moves_by_column_preference(moves, previous_tableau_column);
    
    for m in sorted_moves {
        if game.execute_move(&m).is_ok() {
            path.push(m.clone());
            
            // Determine the new preferred column for the next iteration
            let next_preferred_column = get_tableau_column(&m.source);
            
            if dfs(game, path, counter, ancestors, visited, next_preferred_column) {
                // Remove from ancestors before returning success (visited stays for future pruning)
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
    // (visited cache keeps the state for future pruning)
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
    println!("Solving FreeCell game using strategy 8 (Enhanced strat7 with tableau column preference) with cancellation support...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: Some(cancel_flag.clone()),
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    // Use LRU cache for efficient pruning of previously visited states
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let mut visited = LruCache::new(lru_size);
    
    let result = dfs(&mut game_state, &mut path, &mut counter, &mut ancestors, &mut visited, None);
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
    println!("Solving FreeCell game using strategy 8 (Enhanced strat7 with tableau column preference)...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
        cancel_flag: None,
    };
    // Use HashSet to track only ancestor states (states in current path)
    let mut ancestors = HashSet::new();
    // Use LRU cache for efficient pruning of previously visited states
    let lru_size = NonZeroUsize::new(250_000_000).unwrap();
    let mut visited = LruCache::new(lru_size);
    
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
