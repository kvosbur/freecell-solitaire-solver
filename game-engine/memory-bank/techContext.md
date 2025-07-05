# techContext.md

**Purpose:**  
Documents the technologies used, development setup, technical constraints, dependencies, and tool usage patterns for the project.

---

## Technologies Used

- **Language:** Rust
- **Build Tool:** Cargo
- **Testing:** `rstest` for parameterized and unit tests
- **Linting/Formatting:** clippy, rustfmt

## Development Setup

- Modular Rust project with all core logic in a library crate (`src/lib.rs`)
- All rule validation functions and tests are in the library crate for easy testing and reuse
- Tests are written using `#[cfg(test)]` and `rstest` for comprehensive coverage

## Technical Constraints

- Library crate only: no built-in UI or direct user interaction
- Minimize dependencies unless they provide clear learning or usability benefits
- Code must be idiomatic Rust and pass clippy/rustfmt checks
- **New Constraint**: The `freecell-game-engine` crate must remain pure, focusing solely on core FreeCell game rules and mechanics. Features related to solver heuristics, game analysis, or UI-specific logic are explicitly excluded and will reside in separate, higher-level crates.

## Dependencies

- `rstest` for parameterized testing (dev-dependency)
- Standard Rust library

## Tool Usage Patterns

- **TDD Workflow:**  
  - Write failing tests for each rule before implementation
  - Implement rule logic as pure functions
  - Refactor and document as needed
- **Testing:**  
  - Use `rstest` for parameterized tests to cover all rule scenarios
  - All rule functions are tested with both valid and invalid cases
- **Documentation:**  
  - All technical and architectural decisions are documented in the Memory Bank

## Current Project Structure

- `src/lib.rs`: Core game logic, rule validation functions, and tests
- `src/main.rs`: (Optional, for integration examples or demos)
- `memory-bank/`: Project documentation and context files

## Next Steps

- Expand and document the API for integration with UIs and solvers. This will involve implementing the refined API design for the next major version (v0.2.0) as detailed in `activeContext.md` and `progress.md`.
- Add deck creation, shuffling, and Microsoft-compatible deal logic.
- Add seed-based reproducible deals for testing/solvers.
- Continue using TDD and parameterized testing for new features.
- Expand documentation as the codebase evolves, especially for the upcoming API changes.
