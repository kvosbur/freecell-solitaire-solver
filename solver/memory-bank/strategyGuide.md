# Solver Strategy Guide

## Overview

The solver includes multiple strategies for finding solutions to FreeCell games. Each strategy has different performance characteristics and is suitable for different types of problems.

## Available Strategies

### `strat1`

- **Description**: A simple, unoptimized search algorithm.
- **Best For**: Educational purposes and as a baseline for comparison.

### `strat2`

- **Description**: An improved search algorithm with basic optimizations.
- **Best For**: Simple to moderately complex deals.

### `strat3`

- **Description**: A more advanced search algorithm with better heuristics.
- **Best For**: A good general-purpose strategy.

### `strat4`

- **Description**: A highly optimized search algorithm.
- **Best For**: Complex deals that require a large search space.

### `strat5`

- **Description**: The most advanced and performant strategy, using a combination of techniques.
- **Best For**: The default strategy for most use cases.

## Selecting a Strategy

The desired strategy can be selected using the `--strategy` command-line argument:

```bash
cargo run --bin solver -- --strategy strat5
