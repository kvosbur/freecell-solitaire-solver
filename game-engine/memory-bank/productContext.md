# productContext.md

**Purpose:**  
Describes why this project exists, the problems it solves, how it should work, and the user experience goals. This file provides essential product context for all contributors.

---

## Product Context

### Motivation

This project was created to provide a hands-on, engaging way to learn the Rust programming language by building a robust, reusable, and **pure** FreeCell game engine library. This library strictly focuses on implementing the core rules and mechanics of FreeCell. It is designed to be the authoritative source for FreeCell game logic, intended for integration into a variety of higher-level applications (such as terminal UIs, graphical UIs, and AI solvers) that will depend on this engine. This approach offers a practical context for exploring Rust's features, idioms, and best practices in a clearly defined domain.

### Problems Addressed

- Lack of interactive, real-world projects for learning Rust, specifically in the domain of core game logic.
- Limited availability of reusable, modular game engine libraries for classic card games like FreeCell that strictly adhere to game mechanics without mixing in application-specific concerns.
- Need for a codebase that demonstrates idiomatic Rust in a non-trivial, library-oriented application, with a clear separation of concerns.
- Ensuring a single, authoritative source for FreeCell rules and state management, preventing rule inconsistencies across different applications.

### Intended Workflow

- The `freecell-game-engine` library provides a clean, focused API for:
    - Game state management (initialization, current state)
    - Move validation (checking if a move is legal according to FreeCell rules)
    - Move execution (applying legal moves to update the game state)
    - Basic state inspection (e.g., win condition, available moves)
- Consumers (UIs, solvers, analysis tools, etc.) will integrate this engine library. They are responsible for:
    - User interaction, input, and presentation (for UIs)
    - Strategic decision-making and search algorithms (for solvers)
    - Data analysis and heuristic calculations (for analysis tools)
    - Any I/O operations (file system, network)
- The engine supports reproducible deals and compatibility with Microsoft FreeCell for any given game number, providing a consistent foundation for all downstream applications.

### User Experience Objectives

- **For Developers Consuming the Engine**:
    - **Ease of Use**: Provide a highly ergonomic, intuitive, and consistent API that is easy to integrate into any application (UI, solver, etc.).
    - **Discoverability**: Ensure the API is self-documenting, with clear, predictable method names and comprehensive rustdoc examples.
    - **Robustness**: Guarantee that all rules are correctly implemented and that error handling is clear, consistent, and easy to manage.
    - **Purity**: Maintain a strict separation of concerns, so consumers only get pure game logic without any unnecessary dependencies or features.
    - **Learning Tool**: Serve as an exemplary Rust project that developers can reference for best practices in library design, API development, and documentation.
- **For End-Users (indirectly)**:
    - Ensure a consistent, correct, and bug-free FreeCell experience across any application built on top of this engine.
