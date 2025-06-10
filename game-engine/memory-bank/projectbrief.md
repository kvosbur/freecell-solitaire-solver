# Project Brief: FreeCell Game Engine Library

## Component Mission
Provide a robust, reusable FreeCell game engine library that serves as the foundation for all FreeCell applications in the workspace.

## Component Focus
The `game-engine/` component is a pure Rust library that:
- Implements all FreeCell game rules and logic
- Provides a clean, well-documented API for game state management
- Remains completely decoupled from user interface concerns
- Serves as the single source of truth for FreeCell gameplay

## Core Requirements

### Functional Requirements
- **Complete FreeCell Rules**: Implement all standard FreeCell gameplay rules
- **Game State Management**: Provide comprehensive game state tracking
- **Move Validation**: Validate all possible FreeCell moves according to rules
- **Move Execution**: Execute valid moves and update game state
- **Win Detection**: Automatically detect game completion
- **Microsoft Compatibility**: Support original Microsoft FreeCell deal numbering

### Technical Requirements
- **Pure Library**: No main.rs, no direct user interaction
- **Clean API**: Well-documented public interface for consumers
- **Idiomatic Rust**: Follow Rust best practices and patterns
- **Zero Dependencies**: Minimize external dependencies where possible
- **Comprehensive Testing**: Full test coverage for all game logic

## API Design Goals

### Core Types
- `GameState`: Central game state container
- `Card`: Individual playing card representation
- `Move`: Enumeration of all possible move types
- `Suit`: Card suit enumeration
- Component types: `Tableau`, `FreeCells`, `Foundations`

### Key Operations
- Game initialization and setup
- Move validation and execution
- Game state inspection
- Win condition checking
- Card dealing and shuffling

## Success Criteria
1. All FreeCell rules correctly implemented and tested
2. Clean, intuitive API that's easy for consumers to use
3. Comprehensive documentation with examples
4. Zero runtime panics in normal usage
5. Performance suitable for interactive gameplay
6. Compatible with Microsoft FreeCell deal generation

## Current Status
- ✅ Core data structures implemented (GameState, Card, Move)
- ✅ Component architecture established (Tableau, FreeCells, Foundations)
- ✅ Basic move system defined
- ✅ Library structure and API foundation complete
- 🔄 Move validation and execution logic needs completion
- ⏳ Win detection implementation
- ⏳ Microsoft deal compatibility
- ⏳ Comprehensive testing suite

## Library Design Principles

### Separation of Concerns
- Game logic only - no UI, no I/O, no user interaction
- Pure functions where possible
- Clear boundaries between components

### API Usability
- Intuitive method names and signatures
- Comprehensive error messages
- Examples in documentation
- Consistent patterns across the API

### Robustness
- All public methods handle edge cases gracefully
- Invalid operations return errors rather than panicking
- Game state remains consistent after any operation
- Comprehensive input validation

## Integration Considerations

### Consumer Applications
- **game/**: Interactive FreeCell gameplay
- **solver/**: Automated solution finding
- **appAutomation/**: Mobile app integration
- Future applications as needed

### API Stability
- Maintain backward compatibility when possible
- Document breaking changes clearly
- Version appropriately for semantic versioning
- Consider consumer needs when evolving API
