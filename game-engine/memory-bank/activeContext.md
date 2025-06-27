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

## Next Steps

- Propagate interface consistency to all consumers (e.g., solver, UI) and update documentation.
- Add deck creation and shuffling logic.
- Implement the standard FreeCell deal algorithm (Microsoft-compatible).
- Add seed-based reproducible deals for testing/solvers.
- Expand and document the API for integration with UIs and solvers.
- Continue updating documentation as the codebase evolves.

## Active Decisions & Considerations

- Rust is the language of choice for both learning and implementation.
- The project is a library crate only, with no built-in user interface or direct user interaction.
- Emphasis on modular, idiomatic Rust code and clear documentation.
- TDD is the primary development workflow.
- Minimize dependencies unless they provide clear learning or usability benefits.
- **Interface Consistency**: All core components now follow a standardized interface pattern for core operations, with domain-specific error types for clarity and maintainability.

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
