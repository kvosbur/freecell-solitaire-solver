# Project Brief: FreeCell Solver Component

## Component Mission
Develop an automated solution finder for FreeCell games that leverages the shared game-engine library to validate game states and generate optimal move sequences.

## Component Focus
The `solver/` component is a Rust binary crate that:
- Integrates with the `game-engine` library for all game logic and validation
- Implements algorithms to solve FreeCell deals, finding optimal or near-optimal solutions
- Outputs move sequences in a standardized format for use by other components (e.g., appAutomation)

## Core Requirements

### Functional Requirements
- **Game State Validation**: Use game-engine APIs to validate and manipulate game states
- **Solving Algorithms**: Implement algorithms (e.g., BFS, A*, IDA*) to explore the game tree and find solutions
- **Solution Output**: Provide move sequences in a format compatible with other workspace components
- **Performance**: Efficiently handle complex game states and large search spaces

### Technical Requirements
- **Rust Binary Crate**: Standalone executable for solving FreeCell deals
- **Integration**: Use standardized interfaces from game-engine for all state manipulation
- **Code Quality**: Maintainable, well-documented, and testable code
- **Extensibility**: Support for multiple solving strategies and heuristics

## Success Criteria
1. Solver can find solutions for standard FreeCell deals using the game-engine API
2. Output format is compatible with downstream consumers (automation, UI)
3. Performance is acceptable for practical use
4. Code is maintainable and extensible for new strategies

## Current Status
- Skeleton structure in place (Cargo.toml, main.rs, initial modules)
- Configured to use game-engine as a dependency
- Algorithm implementation pending
- Will adopt new interface patterns from game-engine for all state operations

## Integration Considerations
- Must track and adapt to any breaking changes in game-engine interfaces
- Solution output format should be documented and standardized for integration with appAutomation and other tools
