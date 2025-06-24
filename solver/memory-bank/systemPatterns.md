# systemPatterns.md

**Purpose:**  
Documents the system architecture, key technical decisions, design patterns, and critical implementation paths for the solver component.

---

## System Architecture

### Integration with game-engine

- The solver is a Rust binary crate that depends on the shared game-engine library for all game logic, state validation, and move execution.
- All state manipulations and move validations are performed using the standardized interfaces from game-engine (place_card, remove_card, get_card, etc.).
- The solver does not reimplement any game rules; it delegates all rule logic to game-engine.

### Solving Algorithm Patterns

- The core of the solver is a search algorithm (e.g., BFS, A*, IDA*) that explores possible move sequences from a given game state to a win condition.
- Each node in the search tree represents a unique game state, validated and manipulated using game-engine APIs.
- The solver tracks visited states to avoid cycles and redundant computation.
- Heuristics may be used to prioritize promising branches (for A* or similar algorithms).

### Output and Integration Patterns

- Solutions are output as sequences of moves, using the Move type from game-engine.
- Output format is designed for compatibility with other workspace components (e.g., appAutomation).
- The solver may support multiple output formats (e.g., JSON, plain text) as needed.

## Key Technical Decisions

- Use only game-engine APIs for all game logic to ensure consistency and maintainability.
- Design the solver to be extensible, allowing for new algorithms and heuristics to be added.
- Maintain clear separation between search logic, state management, and output formatting.

## Critical Implementation Paths

1. Parse input (deal number or game state).
2. Initialize GameState using game-engine.
3. Run solving algorithm to find a sequence of valid moves to the win condition.
4. Output the solution in the required format.

## Patterns for Future Expansion

- Support for multiple solving strategies (user-selectable).
- Pluggable heuristics for performance tuning.
- Integration hooks for testing and benchmarking.
