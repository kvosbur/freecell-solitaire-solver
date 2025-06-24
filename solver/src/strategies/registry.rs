use super::{SolverStrategy, StrategyError};
use std::collections::HashMap;
use std::sync::Arc;

pub struct StrategyRegistry {
    strategies: HashMap<String, Arc<dyn SolverStrategy>>,
}

impl StrategyRegistry {
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
        }
    }

    pub fn register_strategy(&mut self, strategy: Box<dyn SolverStrategy>) {
        let name = strategy.name().to_string();
        self.strategies.insert(name, Arc::from(strategy));
    }

    pub fn get_strategy(&self, name: &str) -> Result<Arc<dyn SolverStrategy>, StrategyError> {
        self.strategies
            .get(name)
            .cloned()
            .ok_or_else(|| StrategyError::NotFound(name.to_string()))
    }

    pub fn list_strategies(&self) -> Vec<(&str, &str)> {
        self.strategies
            .values()
            .map(|s| (s.name(), s.description()))
            .collect()
    }

    pub fn auto_discover() -> Self {
        let mut registry = Self::new();

        // Register all available strategies
        registry.register_strategy(Box::new(super::strat1::Strat1::new()));
        registry.register_strategy(Box::new(super::strat2::Strat2::new()));
        registry.register_strategy(Box::new(super::strat3::Strat3::new()));
        registry.register_strategy(Box::new(super::strat4::Strat4::new()));
        registry.register_strategy(Box::new(super::strat5::Strat5::new()));

        registry
    }
}

impl Default for StrategyRegistry {
    fn default() -> Self {
        Self::auto_discover()
    }
}
