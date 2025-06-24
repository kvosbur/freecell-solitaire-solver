# techContext.md

**Purpose:**  
Documents the technologies used, development setup, technical constraints, dependencies, and tool usage patterns for the solver component.

---

## Technologies Used

- **Language:** Rust
- **Build Tool:** Cargo
- **Testing:** To be implemented (unit and integration tests planned)
- **Linting/Formatting:** clippy, rustfmt

## Development Setup

- Rust binary crate with all solving logic in `src/`
- Depends on the shared game-engine library for all game logic and state management
- Will use idiomatic Rust patterns and workspace conventions

## Technical Constraints

- All game state operations must use the standardized interfaces from game-engine (place_card, remove_card, get_card, etc.)
- Minimize dependencies unless they provide clear benefits for solving algorithms or output formatting
- Code must be maintainable, extensible, and pass clippy/rustfmt checks

## Dependencies

- **game-engine**: For all game logic, state validation, and move execution
- Standard Rust library
- (Planned) Additional crates for algorithmic efficiency or output formatting as needed

## Tool Usage Patterns

- **TDD Workflow (Planned):**  
  - Write failing tests for each solving strategy before implementation
  - Implement solving logic as pure functions where possible
  - Refactor and document as needed
- **Testing:**  
  - Add unit and integration tests to verify solver correctness and integration with game-engine
- **Documentation:**  
  - All technical and architectural decisions are documented in the Memory Bank

## Next Steps

- Implement initial solving algorithm using game-engine APIs
- Add tests for solver correctness and integration
- Expand documentation as the codebase evolves
