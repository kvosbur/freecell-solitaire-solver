# Testing Strategy

## Overview

This project employs a multi-layered testing strategy to ensure the correctness and performance of the `game-engine` and `solver` components.

## Layers of Testing

### 1. Unit Tests

- **Location**: Within each component's `src` directory.
- **Purpose**: To test individual functions and modules in isolation.
- **Examples**:
    - `game-engine`: Testing card validation, move generation, and game state transitions.
    - `solver`: Testing the logic of individual solving strategies.

### 2. Integration Tests

- **Location**: In the `tests` directory of the `game-engine` crate.
- **Purpose**: To test the interaction between the `game-engine` and `solver`.
- **Examples**:
    - Verifying that the `solver` can correctly solve a known game state using the `game-engine`.
    - Ensuring that the `solver` does not make invalid moves.

### 3. Benchmarking

- **Location**: In the `solver` component's `main.rs`.
- **Purpose**: To measure and compare the performance of different solving strategies.
- **Workflow**:
    1.  A known game state and its solution are loaded.
    2.  A subset of the solution moves are undone to create a starting position.
    3.  The selected strategy is timed to see how long it takes to find a solution.
    4.  The process is repeated with an increasing number of undone moves until the strategy fails.

## Running Tests

- **All tests**: `cargo test --workspace`
- **Unit tests for a specific component**: `cargo test -p <component-name>`
- **Benchmarks**: `cargo run --bin solver -- --benchmark`
