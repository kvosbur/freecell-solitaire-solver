# Integration Patterns: Solver and Game-Engine

## Overview

The `solver` and `game-engine` components are tightly integrated to enable high-performance FreeCell solving. This document outlines the key patterns that govern their interaction.

## Core Integration Points

### 1. GameState as the Single Source of Truth

- **Pattern**: The `game-engine`'s `GameState` struct is the canonical representation of the game state. The `solver` never modifies the game state directly; instead, it uses the `game-engine`'s APIs to validate and execute moves.
- **Benefit**: This ensures that all game logic and rules are enforced by the `game-engine`, preventing the `solver` from making invalid moves.

### 2. Strategy Pattern for Solving Algorithms

- **Pattern**: The `solver` uses a strategy pattern to implement different solving algorithms. Each strategy is a separate module that implements a common trait. A central `StrategyRegistry` discovers and manages the available strategies.
- **Benefit**: This allows for easy extension of the solver with new algorithms without modifying the core solver logic.

### 3. Packed State for Performance

- **Pattern**: The `solver` uses a `packed_state` representation to minimize the memory footprint of game states during the search process. This packed state can be efficiently converted to and from the `game-engine`'s `GameState`.
- **Benefit**: This significantly reduces memory consumption, allowing the solver to explore a larger search space.

### 4. CLI-Driven Execution

- **Pattern**: The `solver` is designed to be driven from the command line. The `clap` crate is used to parse command-line arguments, allowing users to select strategies, set timeouts, and run benchmarks.
- **Benefit**: This makes the solver easy to automate and integrate into larger workflows.

## Workflow

1.  **Initialization**: The `solver` is initialized with a starting `GameState` from the `game-engine`.
2.  **Strategy Selection**: The user selects a solving strategy via the CLI.
3.  **State Packing**: The `solver` converts the initial `GameState` into its packed representation.
4.  **Search**: The selected strategy performs a search for a solution, using the `game-engine` to generate and validate moves.
5.  **Solution**: If a solution is found, it is returned as a sequence of `Move` structs from the `game-engine`.
