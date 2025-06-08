# activeContext.md

**Purpose:**  
Tracks the current work focus, recent changes, next steps, active decisions and considerations, important patterns and preferences, and key learnings and project insights. This file is updated frequently to reflect the evolving state of the project.

---

## Current Work Focus

- The project is a robust, reusable FreeCell game engine library crate (not a full game app).
- All core logic, move validation/execution, and state inspection APIs are implemented and tested.
- The engine is designed for integration with UIs, solvers, and other applications, with no direct user interface or I/O.
- Focus is on providing a clean, well-documented API and preparing for downstream integration.

## Recent Changes

- Completed modularization: `card`, `rules`, `tableau`, `freecells`, `foundations`, `game_state` modules.
- Implemented `GameState` struct to model the board, freecells, tableau, and foundations.
- Move validation and execution logic is complete.
- Added game state inspection API (win detection, available moves).
- 50+ tests now passing, covering all rule logic and state transitions.
- Documentation and code organization improved for clarity and reusability.

## Next Steps

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
