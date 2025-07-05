# Performance Metrics

## Overview

This document outlines the key performance metrics for the solver and the targets for optimization.

## Key Metrics

- **Time to Solve**: The wall-clock time it takes for a strategy to find a solution.
- **Nodes Explored**: The number of game states (nodes) explored by a strategy during its search.
- **Memory Usage**: The peak memory usage of the solver during a run.

## Benchmarking

The `solver` includes a benchmarking harness that can be used to measure these metrics for each strategy. The benchmark works by taking a known solved game and "undoing" a certain number of moves. It then measures the performance of the strategy in finding a solution from that point.

To run the benchmark:

```bash
cargo run --bin solver -- --benchmark
```

## Optimization Targets

- **Time to Solve**: Reduce the time to solve for all strategies, with a focus on `strat5`.
- **Nodes Explored**: Minimize the number of nodes explored by improving heuristics and search algorithms.
- **Memory Usage**: Continue to optimize the `packed_state` representation and other data structures to reduce memory usage.
