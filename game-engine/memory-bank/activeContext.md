# activeContext.md

**Purpose:**  
Tracks the current work focus, recent changes, next steps, active decisions and considerations, important patterns and preferences, and key learnings and project insights. This file is updated frequently to reflect the evolving state of the project.

---

## Current Work Focus

- The project is a robust, reusable FreeCell game engine library crate (not a full game app).
- All core logic, move validation/execution, and state inspection APIs are implemented and tested.
- The engine is designed for integration with UIs, solvers, and other applications, with no direct user interface or I/O.
- Focus is on providing a clean, well-documented API and preparing for downstream integration.
- **Recent focus:** Interface consistency refactor for `FreeCells`, `Foundations`, and `Tableau` to standardize method signatures and error handling across all core components.
- **Current Focus**: Comprehensive API review completed. The game engine is in an excellent state, demonstrating strong adherence to Rust best practices. A decision has been made to pursue breaking changes in the next major version to further refine the API, ensuring it remains a pure, focused game engine, deferring solver-specific or analysis-specific features to separate, higher-level crates.

## Recent Changes

- Refactored `FreeCells`, `Foundations`, and `Tableau` to use consistent method signatures for `place_card`, `remove_card`, and `get_card`, each with their own domain-specific error types.
- Updated helper methods for counting and emptiness checks to be consistent across all three components.
- Updated move execution and undo logic in `game_state/execution.rs` to use the new interfaces.
- Updated tests to match the new signatures and error handling.
- Completed modularization: `card`, `rules`, `tableau`, `freecells`, `foundations`, `game_state` modules.
- `game_state` split into: `mod.rs`, `error.rs`, `validation.rs`, `moves.rs`, `execution.rs`.
- `is_move_valid` now delegates to private helper methods for each move type.
- Unified error handling with a single `GameError` type and `Display` implementation for game state errors.
- Implemented `Default` for `GameState`.
- Added comprehensive documentation to all modules and methods.
- Move validation and execution logic is complete.
- Added game state inspection API (win detection, available moves).
- 50+ tests now passing, covering all rule logic and state transitions.
- Documentation and code organization improved for clarity and reusability.
- **New**: Completed a comprehensive API review of the `game-engine` crate. Identified areas for significant improvement by introducing breaking changes to achieve a cleaner, more focused, and more robust API, specifically by separating core game engine responsibilities from solver/analysis/UI-specific concerns.

## Next Steps

- **Prepare for Next Major Version (v0.2.0)**: Implement the refined API design focusing on a pure game engine.
    1.  **Enhanced Error System**: Implement a rich `GameError` that preserves full context from component-specific errors.
    2.  **Type-Safe Locations**: Introduce a validated `Location` struct for all game areas, ensuring type safety and preventing invalid indices.
    3.  **Clean Move System**: Redesign the `Move` struct to be type-safe and focused solely on game mechanics, removing solver-specific metadata.
    4.  **Focused GameState API**: Streamline the `GameState` API, ensuring all methods are core to game rules and mechanics, and provide consistent `Result`-based return types.
    5.  **Component Interface Refinement**: Ensure all component methods (`Tableau`, `FreeCells`, `Foundations`) return `Result` for all fallible operations, providing consistent error handling.
- **Documentation Update**: Thoroughly update all API documentation to reflect the new design and provide clear migration guides.
- **Testing**: Ensure comprehensive test coverage for all new and modified APIs.

## Active Decisions & Considerations

- Rust is the language of choice for both learning and implementation.
- The project is a library crate only, with no built-in user interface or direct user interaction.
- Emphasis on modular, idiomatic Rust code and clear documentation.
- TDD is the primary development workflow.
- Minimize dependencies unless they provide clear learning or usability benefits.
- **Interface Consistency**: All core components now follow a standardized interface pattern for core operations, with domain-specific error types for clarity and maintainability.
- **New Architectural Decision**: The `game-engine` crate will strictly adhere to core FreeCell game rules and mechanics. Features related to solver heuristics, game analysis, or UI-specific logic will be explicitly excluded from this crate and developed in separate, higher-level crates that depend on `game-engine`. This ensures a pure, focused, and highly reusable core library.

## Important Patterns & Preferences

- Modular code organization (separation of game logic from UI and input).
- Use of state and command patterns for game state management and user actions.
- Consistent use of Rust best practices (clippy, rustfmt, documentation).
- Parameterized testing with rstest for comprehensive rule validation.

## Learnings & Project Insights

- Comprehensive documentation at the outset provides clarity and direction.
- Defining architecture and patterns early helps guide future development and learning.
- TDD ensures correctness and confidence in rule logic.
- Rust's ecosystem (Cargo, clippy, rustfmt) supports maintainable and high-quality code.
- **Interface consistency across components greatly improves maintainability and integration.**
- **New Insight**: Maintaining a strict separation of concerns, especially between core game logic and higher-level application-specific features (like solvers or advanced analysis), is crucial for building a truly reusable and maintainable game engine. Breaking changes are justified when they lead to a significantly cleaner and more focused API.
