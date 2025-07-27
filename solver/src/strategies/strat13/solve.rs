use crate::packed_state::PackedGameState;
use freecell_game_engine::{r#move::Move, GameState, location::Location};
use freecell_game_engine::game_state::heuristics::score_state;
use freecell_game_engine::{card::{Card, Rank, Suit}, location::{FoundationLocation, TableauLocation}};
use lru::LruCache;
use fxhash::{FxHashMap, FxHashSet, FxBuildHasher};
use std::num::NonZeroUsize;
use std::time::Instant;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SolverResult {
    pub solved: bool,
    pub solution_moves: Option<Vec<Move>>,
}

struct Counter {
    count: Arc<AtomicUsize>,
    start: Instant,
    cancel_flag: Option<Arc<AtomicBool>>,
}

#[derive(Clone)]
struct WorkItem {
    game_state: GameState,
    path: Vec<Move>,
    previous_tableau_column: Option<u8>,
    depth: usize,
}

struct SharedState {
    work_queue: Mutex<VecDeque<WorkItem>>,
    solution_found: AtomicBool,
    solution: Mutex<Option<Vec<Move>>>,
    global_visited: Mutex<Vec<LruCache<PackedGameState, (), FxBuildHasher>>>,
    counter: AtomicUsize,
    start_time: Instant,
}

/// Helper function to extract tableau column index from a location
fn get_tableau_column(location: &Location) -> Option<u8> {
    match location {
        Location::Tableau(tableau_loc) => Some(tableau_loc.index()),
        _ => None,
    }
}

/// Gets the next expected rank for each suit based on what's already in foundations
fn get_next_expected_ranks(game: &GameState) -> FxHashMap<Suit, Rank> {
    let mut expected_ranks = FxHashMap::default();
    
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

/// Worker thread function that processes work items from the shared queue
fn worker_thread(
    thread_id: usize,
    shared_state: Arc<SharedState>,
    cancel_flag: Option<Arc<AtomicBool>>,
    max_depth: usize,
) {
    let mut local_ancestors = FxHashSet::default();
    let mut local_visited = Vec::new();
    
    // Initialize local visited cache
    let lru_size = NonZeroUsize::new(100_000).unwrap();
    for _ in 0..=200 {  // Reasonable upper bound for scores
        local_visited.push(LruCache::with_hasher(lru_size, FxBuildHasher::default()));
    }
    
    loop {
        // Check if solution found or cancelled
        if shared_state.solution_found.load(Ordering::SeqCst) {
            break;
        }
        if let Some(ref flag) = cancel_flag {
            if flag.load(Ordering::SeqCst) {
                break;
            }
        }
        
        // Get work item from queue
        let work_item = {
            let mut queue = shared_state.work_queue.lock().unwrap();
            queue.pop_front()
        };
        
        let work_item = match work_item {
            Some(item) => item,
            None => {
                // No work available, sleep briefly and check again
                thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }
        };
        
        // Process the work item
        if let Some(solution) = process_work_item(
            work_item,
            &mut local_ancestors,
            &mut local_visited,
            &shared_state,
            max_depth,
        ) {
            // Found a solution!
            shared_state.solution_found.store(true, Ordering::SeqCst);
            let mut solution_lock = shared_state.solution.lock().unwrap();
            *solution_lock = Some(solution);
            break;
        }
    }
    
    // println!("Worker thread {} finished", thread_id);
}

/// Process a single work item, potentially generating new work items
fn process_work_item(
    mut work_item: WorkItem,
    local_ancestors: &mut FxHashSet<PackedGameState>,
    local_visited: &mut Vec<LruCache<PackedGameState, (), FxBuildHasher>>,
    shared_state: &Arc<SharedState>,
    max_depth: usize,
) -> Option<Vec<Move>> {
    let mut game = work_item.game_state;
    let mut path = work_item.path;
    
    // Limit recursion depth
    if work_item.depth > max_depth {
        return None;
    }
    
    // Check if won
    if game.is_won().unwrap_or(false) {
        return Some(path);
    }
    
    let score = score_state(&game);
    let packed = PackedGameState::from_game_state_canonical(&game);
    
    // Check local ancestors (cycle detection)
    if local_ancestors.contains(&packed) {
        return None;
    }
    
    // Check local visited states
    if (score as usize) < local_visited.len() && local_visited[score as usize].contains(&packed) {
        return None;
    }
    
    // Check global visited states (with lock)
    {
        let mut global_visited = shared_state.global_visited.lock().unwrap();
        if (score as usize) < global_visited.len() && global_visited[score as usize].contains(&packed) {
            return None;
        }
        global_visited[score as usize].put(packed.clone(), ());
    }
    
    // Add to local tracking
    local_ancestors.insert(packed.clone());
    if (score as usize) < local_visited.len() {
        local_visited[score as usize].put(packed.clone(), ());
    }
    
    // Get moves
    let moves = if score == 0 {
        let mut moves = Vec::new();
        game.get_tableau_to_foundation_moves(&mut moves);
        game.get_freecell_to_foundation_moves(&mut moves);
        if moves.is_empty() {
            println!("Thread: No moves available at winning state");
            return None;
        }
        moves
    } else {
        game.get_available_moves()
    };
    
    let sorted_moves = sort_moves_by_lowest_needed_cards(moves, &game, work_item.previous_tableau_column);
    
    // Process first few moves in this thread, add rest as work items for other threads
    let (process_here, add_to_queue) = if sorted_moves.len() > 3 && work_item.depth < max_depth / 2 {
        sorted_moves.split_at(2)
    } else {
        (sorted_moves.as_slice(), &[][..])
    };
    
    // Add work items for other threads
    if !add_to_queue.is_empty() {
        let mut queue = shared_state.work_queue.lock().unwrap();
        for m in add_to_queue {
            let mut new_game = game.clone();
            if new_game.execute_move(m).is_ok() {
                let mut new_path = path.clone();
                new_path.push(m.clone());
                let next_preferred_column = get_tableau_column(&m.source);
                
                queue.push_back(WorkItem {
                    game_state: new_game,
                    path: new_path,
                    previous_tableau_column: next_preferred_column,
                    depth: work_item.depth + 1,
                });
            }
        }
    }
    
    // Process moves in this thread
    for m in process_here {
        if shared_state.solution_found.load(Ordering::SeqCst) {
            break;
        }
        
        if game.execute_move(m).is_ok() {
            path.push(m.clone());
            let next_preferred_column = get_tableau_column(&m.source);
            
            // Recursively process this move
            let new_work_item = WorkItem {
                game_state: game.clone(),
                path: path.clone(),
                previous_tableau_column: next_preferred_column,
                depth: work_item.depth + 1,
            };
            
            if let Some(solution) = process_work_item(
                new_work_item,
                local_ancestors,
                local_visited,
                shared_state,
                max_depth,
            ) {
                local_ancestors.remove(&packed);
                return Some(solution);
            }
            
            path.pop();
            game.undo_move(m);
        }
    }
    
    // Remove from local ancestors when backtracking
    local_ancestors.remove(&packed);
    
    // Update counter
    let count = shared_state.counter.fetch_add(1, Ordering::SeqCst);
    if count % 100000 == 0 {
        // println!(
        //     "Checked {} game states across all threads, time:{:?}, current score: {}",
        //     count,
        //     shared_state.start_time.elapsed(),
        //     score
        // );
    }
    
    None
}

pub fn solve_with_cancel(
    game_state: GameState,
    cancel_flag: Arc<AtomicBool>,
) -> SolverResult {
    // println!("Solving FreeCell game using strategy 13 (Multi-threaded strat12) with cancellation support...");
    
    let start_score = score_state(&game_state);
    // println!("Starting score: {}", start_score);
    
    // Initialize shared state
    let lru_size = NonZeroUsize::new(1_000_000).unwrap();
    let mut global_visited = Vec::new();
    for _ in 0..=(start_score as usize) {
        global_visited.push(LruCache::with_hasher(lru_size, FxBuildHasher::default()));
    }
    
    let shared_state = Arc::new(SharedState {
        work_queue: Mutex::new(VecDeque::new()),
        solution_found: AtomicBool::new(false),
        solution: Mutex::new(None),
        global_visited: Mutex::new(global_visited),
        counter: AtomicUsize::new(0),
        start_time: Instant::now(),
    });
    
    // Add initial work item
    {
        let mut queue = shared_state.work_queue.lock().unwrap();
        queue.push_back(WorkItem {
            game_state: game_state.clone(),
            path: Vec::new(),
            previous_tableau_column: None,
            depth: 0,
        });
    }
    
    // Spawn worker threads
    let num_threads = num_cpus::get().min(8); // Limit to 8 threads max
    // println!("Spawning {} worker threads", num_threads);
    
    let mut handles = Vec::new();
    for i in 0..num_threads {
        let shared_state_clone = Arc::clone(&shared_state);
        let cancel_flag_clone = Arc::clone(&cancel_flag);
        
        let handle = thread::spawn(move || {
            worker_thread(i, shared_state_clone, Some(cancel_flag_clone), 1000);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = shared_state.counter.load(Ordering::SeqCst);
    let elapsed = shared_state.start_time.elapsed();
    
    if shared_state.solution_found.load(Ordering::SeqCst) {
        let solution = shared_state.solution.lock().unwrap().clone();
        if let Some(moves) = solution {
            // println!(
            //     "Solution found! {} moves in {:?} after checking {} states",
            //     moves.len(),
            //     elapsed,
            //     final_count
            // );
            return SolverResult {
                solved: true,
                solution_moves: Some(moves),
            };
        }
    }
    
    // println!(
    //     "No solution found. Checked {} states in {:?}",
    //     final_count,
    //     elapsed
    // );
    
    SolverResult {
        solved: false,
        solution_moves: None,
    }
}

pub fn solve(game_state: GameState) {
    // println!("Solving FreeCell game using strategy 13 (Multi-threaded strat12)...");
    
    let start_score = score_state(&game_state);
    // println!("Starting score: {}", start_score);
    
    // Initialize shared state
    let lru_size = NonZeroUsize::new(5_000_000).unwrap();
    let mut global_visited = Vec::new();
    for _ in 0..=(start_score as usize) {
        global_visited.push(LruCache::with_hasher(lru_size, FxBuildHasher::default()));
    }
    
    let shared_state = Arc::new(SharedState {
        work_queue: Mutex::new(VecDeque::new()),
        solution_found: AtomicBool::new(false),
        solution: Mutex::new(None),
        global_visited: Mutex::new(global_visited),
        counter: AtomicUsize::new(0),
        start_time: Instant::now(),
    });
    
    // Add initial work item
    {
        let mut queue = shared_state.work_queue.lock().unwrap();
        queue.push_back(WorkItem {
            game_state: game_state.clone(),
            path: Vec::new(),
            previous_tableau_column: None,
            depth: 0,
        });
    }
    
    // Spawn worker threads
    let num_threads = num_cpus::get().min(8); // Limit to 8 threads max
    // println!("Spawning {} worker threads", num_threads);
    
    let mut handles = Vec::new();
    for i in 0..num_threads {
        let shared_state_clone = Arc::clone(&shared_state);
        
        let handle = thread::spawn(move || {
            worker_thread(i, shared_state_clone, None, 1000);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = shared_state.counter.load(Ordering::SeqCst);
    let elapsed = shared_state.start_time.elapsed();
    
    if shared_state.solution_found.load(Ordering::SeqCst) {
        let solution = shared_state.solution.lock().unwrap().clone();
        if let Some(moves) = solution {
            // println!(
            //     "Solution found! {} moves in {:?} after checking {} states",
            //     moves.len(),
            //     elapsed,
            //     final_count
            // );
            // Optionally print moves
            // for m in moves {
            //     println!("{:?}", m);
            // }
        }
    } else {
        // println!("No solution found.");
    }
    
    // println!(
    //     "Checked {} states total in {:?}",
    //     final_count,
    //     elapsed
    // );
}
