# projectbrief.md

**Purpose:**  
This is the foundation document for the project Memory Bank. It defines the core requirements, goals, and scope of the project. All other context files build upon the information established here.

---

## Project Brief

This project is a reusable, modular FreeCell game engine library written in Rust. The primary goal is to provide a robust, idiomatic, and well-tested core for FreeCell gameplay that can be integrated into various applications, such as terminal UIs, graphical UIs, and AI solvers. The project also serves as a learning platform for Rust development.

### Core Requirements
- Implement all rules and gameplay logic for FreeCell as a pure Rust library crate.
- Expose a clear, well-documented API for game state management, move validation/execution, and state inspection.
- Ensure the engine is fully decoupled from any user interface or input/output concerns.
- Provide compatibility with the original Microsoft FreeCell deals for any given game number (using the same algorithm/seed logic).
- Focus on code clarity, idiomatic Rust, and maintainability to support both learning and reusability.

### Goals
- Gain hands-on experience with Rust, including its syntax, ownership model, and ecosystem.
- Build a functional, robust FreeCell game engine that can be reused by UIs, solvers, and other applications.
- Ensure compatibility with the original Microsoft FreeCell deals for any given game number.
- Document the development process and key learnings throughout the project.

### Scope
- The project is a library crate only; it does not include a user interface or direct user interaction.
- The engine is designed for integration with terminal UIs, graphical UIs, AI solvers, and other consumers.
- The codebase will serve as a reference for Rust programming patterns and best practices.
