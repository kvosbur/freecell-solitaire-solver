use super::{SolverStrategy, SolverResult, SolverStats, StrategyConfig, StrategyError};
use freecell_game_engine::GameState;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Instant;

mod solve;

pub struct Strat4 {
    config: StrategyConfig,
}

impl Strat4 {
    pub fn new() -> Self {
        Self {
            config: StrategyConfig::default(),
        }
    }
}

impl SolverStrategy for Strat4 {
    fn name(&self) -> &'static str {
        "strat4"
    }
    
    fn description(&self) -> &'static str {
        "Strategy 4"
    }
    
    fn solve(&self, game_state: GameState, cancel_flag: Arc<AtomicBool>) -> SolverResult {
        let start_time = Instant::now();
        let solved = solve::solve_with_cancel(game_state, cancel_flag);
        let time_elapsed = start_time.elapsed();
        
        SolverResult {
            solved,
            moves: vec![], // TODO: Extract moves from solve function
            stats: SolverStats {
                states_explored: 0, // TODO: Extract from solve function
                time_elapsed,
                max_depth: self.config.max_depth.unwrap_or(200),
                cache_hits: None,
                cache_misses: None,
            },
        }
    }
    
    fn configure(&mut self, config: StrategyConfig) -> Result<(), StrategyError> {
        self.config = config;
        Ok(())
    }
}
