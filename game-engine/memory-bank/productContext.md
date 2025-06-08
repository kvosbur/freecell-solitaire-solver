# productContext.md

**Purpose:**  
Describes why this project exists, the problems it solves, how it should work, and the user experience goals. This file provides essential product context for all contributors.

---

## Product Context

### Motivation

This project was created to provide a hands-on, engaging way to learn the Rust programming language by building a robust, reusable FreeCell game engine library. The library is designed to be integrated into a variety of applications, such as terminal UIs, graphical UIs, and AI solvers, offering a practical context for exploring Rust's features, idioms, and best practices.

### Problems Addressed

- Lack of interactive, real-world projects for learning Rust.
- Limited availability of reusable, modular game engine libraries for classic card games like FreeCell.
- Need for a codebase that demonstrates idiomatic Rust in a non-trivial, library-oriented application.

### Intended Workflow

- Application developers integrate the FreeCell engine library into their own UIs or solvers.
- The library provides a clear API for game state management, move validation/execution, and state inspection.
- Consumers (UIs, solvers, etc.) handle user interaction, input, and presentation, while the engine manages all game logic.
- The engine supports reproducible deals and compatibility with Microsoft FreeCell for solver and analysis use cases.

### User Experience Objectives

- Provide a robust, well-documented API for all core FreeCell actions and state queries.
- Ensure the engine is easy to integrate with a variety of frontends and tools.
- Guarantee compatibility with the original Microsoft FreeCell deals for any given game number.
- Deliver a codebase that is easy to read, understand, and extend, supporting the learning goals of contributors and the needs of downstream consumers.
