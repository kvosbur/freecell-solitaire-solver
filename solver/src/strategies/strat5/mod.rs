use super::{SolverStrategy, SolverResult, SolverStats, StrategyConfig, StrategyError};
use freecell_game_engine::GameState;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Instant;

mod solve;

pub struct Strat5 {
    config: StrategyConfig,
}

impl Strat5 {
    pub fn new() -> Self {
        Self {
            config: StrategyConfig {
                cache_size: Some(250_000_000), // Default LRU cache size
                max_depth: Some(200),
                ..Default::default()
            },
        }
    }
}

impl SolverStrategy for Strat5 {
    fn name(&self) -> &'static str {
        "strat5"
    }
    
    fn description(&self) -> &'static str {
        "strat4 except set a cap on cache size so that ram isn't overrun. Allowed program to run for a long time, but still didn't get any further."
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
                cache_hits: Some(0), // TODO: Extract from solve function
                cache_misses: Some(0), // TODO: Extract from solve function
            },
        }
    }
    
    fn configure(&mut self, config: StrategyConfig) -> Result<(), StrategyError> {
        if let Some(cache_size) = config.cache_size {
            if cache_size == 0 {
                return Err(StrategyError::InvalidConfig("cache_size must be > 0".to_string()));
            }
        }
        self.config = config;
        Ok(())
    }
}
