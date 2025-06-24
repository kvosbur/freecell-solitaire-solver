# productContext.md

**Purpose:**  
Describes why the solver component exists, the problems it solves, how it should work, and the user experience goals.

---

## Product Context

### Motivation

The solver component is designed to provide automated solutions for FreeCell games, enabling users and other applications to find optimal or near-optimal move sequences for any given deal. It leverages the shared game-engine library to ensure all solutions are valid and consistent with FreeCell rules.

### Problems Addressed

- Manual solving of FreeCell games can be time-consuming and error-prone.
- Need for a reliable, automated way to find solutions for analysis, learning, or automation.
- Requirement for a standardized solution format for integration with other workspace components (e.g., appAutomation).

### Intended Workflow

- The solver receives a game state (or deal number) as input.
- It uses the game-engine API to validate and manipulate game states.
- Solving algorithms explore possible move sequences to find a solution.
- The resulting move sequence is output in a standardized format for use by other tools or applications.

### User Experience Objectives

- Provide fast, reliable solutions for standard FreeCell deals.
- Ensure all solutions are valid according to the latest game-engine rules and interfaces.
- Output move sequences in a format that is easy to consume by downstream components.
- Deliver a codebase that is easy to extend with new solving strategies and heuristics.
