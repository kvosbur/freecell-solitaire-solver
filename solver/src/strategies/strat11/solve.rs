use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState, location::Location};
use freecell_game_engine::game_state::heuristics::score_state;
use freecell_game_engine::{card::{Card, Rank, Suit}, location::{FoundationLocation, TableauLocation}};
use lru::LruCache;
use std::collections::HashMap;
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

/// Gets the next expected rank for each suit based on what's already in foundations
fn get_next_expected_ranks(game: &GameState) -> HashMap<Suit, Rank> {
    let mut expected_ranks = HashMap::new();
    
    // Check each foundation pile (there are 4, one for each suit)
    for foundation_index in 0..4 {
        if let Ok(location) = FoundationLocation::new(foundation_index) {
            if let Ok(Some(top_card)) = game.foundations().get_card(location) {
                // If there's a card, the next expected rank is one higher
                let next_rank_value = (top_card.rank() as u8) + 1;
                if next_rank_value <= 13 {  // King is 13
                    if let Ok(next_rank) = Rank::try_from(next_rank_value) {
                        expected_ranks.insert(top_card.suit(), next_rank);
                    }
                }
            } else {
                // Empty foundation, so we need an Ace
                // We need to determine which suit this foundation is for
                // Since foundations don't inherently know their suit, we'll handle this
                // by checking if any card can be placed there
                for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
                    let ace = Card::new(Rank::Ace, suit);
                    if game.foundations().validate_card_placement(location, &ace).is_ok() {
                        expected_ranks.insert(suit, Rank::Ace);
                        break;
                    }
                }
            }
        }
    }
    
    // For any suits not yet started in foundations, they need Aces
    for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
        if !expected_ranks.contains_key(&suit) {
            expected_ranks.insert(suit, Rank::Ace);
        }
    }
    
    expected_ranks
}

/// Finds the lowest rank card that's not yet in the foundations for each tableau column
fn get_column_lowest_needed_ranks(game: &GameState) -> Vec<Option<u8>> {
    let next_expected = get_next_expected_ranks(game);
    let mut column_lowest_ranks = vec![None; 8]; // 8 tableau columns
    
    for column_index in 0..8 {
        if let Ok(column_cards) = game.tableau().get_column(column_index as usize) {
            let mut lowest_needed_rank = 14u8; // Higher than King (13)
            
            for card in column_cards {
                if let Some(&expected_rank) = next_expected.get(&card.suit()) {
                    if card.rank() as u8 >= expected_rank as u8 {
                        // This card is needed in foundations
                        lowest_needed_rank = lowest_needed_rank.min(card.rank() as u8);
                    }
                }
            }
            
            if lowest_needed_rank <= 13 {
                column_lowest_ranks[column_index as usize] = Some(lowest_needed_rank);
            }
        }
    }
    
    column_lowest_ranks
}

/// Sorts moves to prioritize columns with the lowest cards needed for foundations
/// Falls back to tableau column preference from previous move if no clear priority
fn sort_moves_by_lowest_needed_cards(moves: Vec<Move>, game: &GameState, previous_tableau_column: Option<u8>) -> Vec<Move> {
    let column_lowest_ranks = get_column_lowest_needed_ranks(game);
    
    let mut move_priorities: Vec<(Move, u8)> = moves.into_iter().map(|m| {
        let priority = if let Some(source_column) = get_tableau_column(&m.source) {
            let column_idx = source_column as usize;
            if column_idx < column_lowest_ranks.len() {
                if let Some(lowest_rank) = column_lowest_ranks[column_idx] {
                    // Lower rank = higher priority (lower number)
                    lowest_rank
                } else {
                    // No needed cards in this column, give it lower priority
                    20u8
                }
            } else {
                15u8 // Default for invalid column
            }
        } else {
            // Non-tableau moves (freecell, etc.) get medium priority
            10u8
        };
        (m, priority)
    }).collect();
    
    // Sort by priority (lower number = higher priority)
    move_priorities.sort_by_key(|(_, priority)| *priority);
    
    // If we have a tie in priorities, use the previous tableau column preference as tiebreaker
    if let Some(preferred_column) = previous_tableau_column {
        move_priorities.sort_by(|(move_a, priority_a), (move_b, priority_b)| {
            if priority_a == priority_b {
                let a_matches = get_tableau_column(&move_a.source) == Some(preferred_column);
                let b_matches = get_tableau_column(&move_b.source) == Some(preferred_column);
                match (a_matches, b_matches) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            } else {
                priority_a.cmp(priority_b)
            }
        });
    }
    
    move_priorities.into_iter().map(|(m, _)| m).collect()
}

/// Attempts to solve the given FreeCell game state using recursive DFS that combines:
/// 1. Prioritizing moves from columns with lowest cards needed for foundations
/// 2. Tableau column preference optimization from strategy 8 (as tiebreaker)
/// 3. Heuristic-bucketed LRU cache system from strategy 9
/// 4. Enhanced move selection based on game state scoring
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
    
    // Apply lowest-needed-cards prioritization with tableau column preference as tiebreaker
    let sorted_moves = sort_moves_by_lowest_needed_cards(moves, game, previous_tableau_column);
    
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
    println!("Solving FreeCell game using strategy 11 (Enhanced strat10 with lowest-needed-cards prioritization) with cancellation support...");
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
    println!("Solving FreeCell game using strategy 11 (Enhanced strat10 with lowest-needed-cards prioritization)...");
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
