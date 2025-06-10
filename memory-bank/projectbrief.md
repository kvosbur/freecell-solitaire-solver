# Project Brief: FreeCell Solitaire Solver Monorepo

## Core Mission
Build a comprehensive FreeCell solitaire solution ecosystem that can find optimal solutions to FreeCell games and automate entering them into mobile applications.

## Monorepo Architecture
This is a Rust workspace with multiple interconnected components designed around a shared library pattern:

### Component Overview
1. **game-engine/** - Shared library crate containing core FreeCell game logic
   - Foundation for all other components
   - Pure game rules and state management
   - No UI or I/O dependencies

2. **game/** - Interactive FreeCell game application
   - Uses game-engine for core logic
   - Provides user-friendly gameplay interface
   - Current development focus

3. **solver/** - Algorithm-based solution finder
   - Uses game-engine for game state validation
   - Implements solving algorithms
   - Outputs optimal move sequences

4. **appAutomation/** - Mobile app automation system
   - Standalone automation for mobile app interaction
   - Consumes solutions from solver
   - Interfaces with external mobile applications

## Key Requirements

### Functional Requirements
- **Shared Foundation**: Robust, reusable FreeCell game engine
- **Interactive Play**: User-friendly game interface
- **Automated Solving**: Algorithm to find optimal solutions
- **Mobile Integration**: Ability to input solutions into mobile apps
- **Consistency**: All components use same game rules and logic

### Technical Requirements
- **Language**: Rust (2021 edition)
- **Architecture**: Workspace with shared library pattern
- **Code Quality**: Clean, well-documented, testable code
- **Performance**: Efficient algorithms and state management
- **Modularity**: Clear separation of concerns between components

## Success Criteria
1. Game engine correctly implements all FreeCell rules
2. All components can integrate seamlessly with game engine
3. Solver can find solutions for standard FreeCell deals
4. Game provides enjoyable interactive experience
5. Automation successfully interfaces with target mobile apps
6. Workspace maintains clean dependency relationships

## Constraints
- Game engine must remain UI-agnostic library
- Each application should be focused and single-purpose
- Shared dependencies managed at workspace level
- Follow Rust best practices and workspace conventions
- Maintain compatibility with Microsoft FreeCell deal numbering
