# systemPatterns.md

**Purpose:**  
Documents system architecture, key technical decisions, design patterns in use, component relationships, and critical implementation paths.

---

## System Architecture

- **Modular Rust Library Design:**  
  - All core game logic, rule validation, and game state management are separated for testability and maintainability.
  - The engine is implemented as a pure Rust library crate, fully decoupled from any user interface or I/O.
  - All rule logic is implemented as pure functions, making them easy to test, reuse, and integrate with external consumers (UIs, solvers, etc.).

- **Rules Engine:**  
  - Each FreeCell rule (tableau stacking, foundation stacking, free cell, empty tableau, foundation immutability) is implemented as a dedicated function.
  - Rules are validated independently, supporting both unit and integration testing.

- **Testing Patterns:**  
  - TDD (Test-Driven Development) is the primary workflow.
  - Parameterized tests using `rstest` provide comprehensive coverage for all rule functions.
  - Each rule is tested with both valid and invalid scenarios.

- **Game State Layer:**  
  - The `GameState` struct represents tableau, free cells, and foundations, and exposes all state and move APIs.
  - Move validation and execution are methods on `GameState`, leveraging the rules engine.
  - The API is designed for easy integration with UIs, solvers, and other applications.

## Key Technical Decisions

- **Pure Functions for Rules:**  
  - All rule logic is stateless and side-effect free.
  - This enables easy testing and seamless integration with UIs, solvers, and other consumers.

- **Library-First Architecture:**  
  - The engine is a library crate only, with no built-in UI or direct user interaction.
  - All APIs are designed for reusability and extensibility by downstream applications.

- **Idiomatic Rust:**  
  - Use of enums, pattern matching, and strong typing for clarity and safety.
  - Consistent use of Rust best practices (clippy, rustfmt, documentation).

- **Documentation-First:**  
  - All architectural and technical decisions are documented in the Memory Bank for session resilience.

## Component Relationships

- **Card**: Fundamental struct, used by all rule functions.
- **Rule Functions**: Operate on Card(s), used by tests and GameState.
- **GameState**: Aggregates tableau, free cells, and foundations, and uses rule functions for move validation and execution.
- **External Consumers**: UIs, solvers, and other applications interact with the engine solely through the public API.

## Critical Implementation Paths

- **Rule Validation**:  
  - All movement and placement rules are validated before any game state mutation.
  - Foundation is treated as immutable for moves out.

- **API Design**:  
  - All state, move, and inspection APIs are exposed for use by UIs and solvers.
  - No UI or I/O logic is present in the engine crate.

- **Testing**:  
  - All rules are covered by parameterized tests, ensuring correctness and preventing regressions.
