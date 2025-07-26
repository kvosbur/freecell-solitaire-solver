# Strategy 8: Enhanced Tableau Column Preference

## Overview

Strategy 8 builds on Strategy 7 by adding tableau column preference to improve solving efficiency. While Strategy 7 uses hybrid ancestor tracking with LRU cache for cycle detection and pruning, Strategy 8 enhances the move ordering by preferring moves from the same tableau column as the previous move.

## Key Features

1. **Hybrid Memory Management** (from Strategy 7):
   - Uses HashSet for ancestor tracking (cycle detection)
   - Uses LRU cache for visited state pruning
   - Memory efficient with performance benefits

2. **Tableau Column Preference** (new in Strategy 8):
   - Tracks which tableau column the previous card was moved from
   - Sorts available moves to prioritize moves from the same column
   - Encourages working within the same tableau column to potentially create longer sequences

## Algorithm Enhancement

The strategy modifies the DFS search by:

1. **Tracking Previous Column**: Each recursive call tracks the tableau column index of the source location from the previous move
2. **Move Sorting**: Before evaluating moves, they are sorted to prioritize:
   - First: Moves from the same tableau column as the previous move
   - Second: All other moves (tableau moves from different columns, freecell moves, foundation moves)
3. **Column Propagation**: When a move is executed, the source column (if from tableau) becomes the preferred column for the next iteration

## Implementation Details

- `get_tableau_column()`: Extracts tableau column index from a Location
- `sort_moves_by_column_preference()`: Reorders moves to prefer same-column moves
- Enhanced `dfs()` function: Takes an additional `previous_tableau_column` parameter
- Move ordering maintains all the cycle detection and pruning benefits of Strategy 7

## Expected Benefits

- **Improved Move Efficiency**: By working within the same tableau column, the solver may create longer valid sequences more quickly
- **Better Heuristic**: Tableau columns that have been recently modified are more likely to have further productive moves
- **Maintained Performance**: Retains all the memory and performance optimizations from Strategy 7

## Usage

Strategy 8 can be used exactly like Strategy 7, with the same function signatures:
- `solve(game_state)`: Standard solve function
- `solve_with_cancel(game_state, cancel_flag)`: Solve with cancellation support
