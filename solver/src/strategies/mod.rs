use freecell_game_engine::{GameState, action::Action};
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Duration;
use std::collections::HashMap;

// Core trait that all strategies must implement
pub trait SolverStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn solve(&self, game_state: GameState, cancel_flag: Arc<AtomicBool>) -> SolverResult;
    fn configure(&mut self, config: StrategyConfig) -> Result<(), StrategyError>;
}

// Result type returned by all strategies
#[derive(Debug, Clone)]
pub struct SolverResult {
    pub solved: bool,
    pub moves: Vec<Action>,
    pub stats: SolverStats,
}

// Statistics collected during solving
#[derive(Debug, Clone, Default)]
pub struct SolverStats {
    pub states_explored: u64,
    pub time_elapsed: Duration,
    pub max_depth: usize,
    pub cache_hits: Option<u64>,
    pub cache_misses: Option<u64>,
}

// Configuration for strategies
#[derive(Debug, Clone, Default)]
pub struct StrategyConfig {
    pub max_depth: Option<usize>,
    pub cache_size: Option<usize>,
    pub timeout_seconds: Option<u64>,
    pub custom_params: HashMap<String, String>,
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum StrategyError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Strategy not found: {0}")]
    NotFound(String),
    #[error("Solve failed: {0}")]
    SolveFailed(String),
}

// Re-export strategy modules
pub mod strat1;
pub mod strat2;
pub mod strat3;
pub mod strat4;
pub mod strat5;
pub mod registry;

pub use registry::StrategyRegistry;
