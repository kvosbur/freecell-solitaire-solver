# System Patterns: Game Engine Library Architecture

## Library Architecture

### Pure Library Design
The game engine follows a strict library-only architecture:
- **No main.rs**: Pure library crate with no executable entry point
- **No I/O Dependencies**: No user interface, file system, or network operations
- **Pure Functions**: Game logic implemented as stateless, side-effect-free functions
- **API-First**: All functionality exposed through well-defined public interfaces

### Modular Component Structure

The `game_state` module is split into focused submodules for maintainability and clarity:

```
game_state/
  ├── mod.rs         // Main struct, core methods, docs, Default
  ├── error.rs       // GameError type, Display impl
  ├── validation.rs  // is_move_valid and private helpers per move type
  ├── moves.rs       // Move generation logic
  └── execution.rs   // Move execution and undo logic
```

- `is_move_valid` delegates to private helper methods for each move type, improving readability and testability.
- All error handling is unified under a single `GameError` type with a `Display` implementation.
- Comprehensive documentation is provided at the module, struct, and method level.

```rust
pub struct GameState {
    pub tableau: Tableau,      // 8 columns of cards
    pub freecells: FreeCells,  // 4 temporary storage cells
    pub foundations: Foundations, // 4 suit-based foundation piles
}
```

Each component is self-contained with clear responsibilities:
- **Tableau**: Manages 8 main playing columns and cascading rules
- **FreeCells**: Manages 4 temporary storage cells with occupancy rules
- **Foundations**: Manages 4 suit-based completion piles with sequence rules

## Core Data Patterns

### Card Representation
```rust
pub struct Card {
    pub rank: u8,    // 1-13 (Ace through King)
    pub suit: Suit,  // Hearts, Diamonds, Clubs, Spades
}

pub enum Suit {
    Hearts, Diamonds, Clubs, Spades
}

impl Card {
    pub fn color(&self) -> Color {
        // Red/Black color determination
    }
}
```

### Move System Architecture
The `Move` struct uses the type-safe `Location` enum for its source and destination, ensuring that all moves are between valid game areas. It includes a `card_count` field, though currently only single-card moves are implemented.

```rust
// Current Move struct
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub source: Location,
    pub destination: Location,
    pub card_count: u8, // Currently only 1 is supported
}
```

## Rule Engine Patterns

### Pure Function Rule Validation
Each component (`Tableau`, `Foundations`) provides a `validate_card_placement` method that encapsulates its specific placement rules. These functions are pure, testable, and return a `Result<(), Error>`.

```rust
// In Tableau
pub fn validate_card_placement(&self, column: usize, card: &Card) -> Result<(), TableauError> {
    // Alternating colors and descending rank validation
}

// In Foundations
pub fn validate_card_placement(&self, pile: usize, card: &Card) -> Result<(), FoundationError> {
    // Same suit and ascending rank validation
}
```

### Validation Before Mutation
The `GameState` orchestrates validation and execution. `execute_move` first calls `is_move_valid` to ensure correctness before mutating the state.

```rust
impl GameState {
    pub fn execute_move(&mut self, m: &Move) -> Result<(), GameError> {
        // 1. is_move_valid is called internally to ensure correctness
        self.is_move_valid(m)?;
        
        // 2. Private execute_* methods apply the move
        // ...
        Ok(())
    }
}
```

## API Design Patterns

### Result-Based API (Mostly Consistent)
Most fallible operations return `Result<T, Error>`, providing rich error context. However, some component methods still return `Option` where `Result` would be more consistent. This is a key area for refinement in v0.2.0.

```rust
// Example of current API
impl GameState {
    // Good: Consistent Result return
    pub fn execute_move(&mut self, m: &Move) -> Result<(), GameError> {
        // ...
    }

    // Good: Consistent Result return
    pub fn get_card(&self, location: Location) -> Result<Option<&Card>, GameError> {
        // ...
    }
}

// Inconsistency example in a component
impl Tableau {
    // Should be Result<Option<&Card>, TableauError> for consistency
    pub fn get_card(&self, column: usize) -> Result<Option<&Card>, TableauError> {
        // ...
    }
}
```

### Immutable State Inspection
```rust
impl GameState {
    pub fn is_game_won(&self) -> bool {
        // Non-mutating win condition check
    }
    
    pub fn get_tableau_column(&self, column: usize) -> Option<&Vec<Card>> {
        // Safe, immutable access to game state
    }
}
```

### Builder Pattern for Initialization
```rust
impl GameState {
    pub fn new() -> Self {
        // Creates properly initialized game state
    }
    
    pub fn new_with_deal(deal_number: u32) -> Self {
        // Creates game with specific Microsoft-compatible deal
    }
}
```

## Component Interaction Patterns

### Encapsulation and Delegation
```rust
impl GameState {
    fn validate_tableau_move(&self, from: usize, to: usize) -> bool {
        // Delegates to tableau component for validation
        self.tableau.can_move_sequence(from, to)
    }
    
    fn execute_tableau_move(&mut self, from: usize, to: usize) {
        // Delegates to tableau component for execution
        self.tableau.move_sequence(from, to);
    }
}
```

### Component Boundaries
- **GameState**: Orchestrates moves and maintains overall game state
- **Tableau**: Handles column-specific logic and cascading moves
- **FreeCells**: Manages temporary storage with occupancy rules
- **Foundations**: Manages completion piles with suit/rank rules

---

## Interface Consistency Pattern (2025-06)

### Motivation
To improve maintainability, predictability, and integration, the interfaces for `FreeCells`, `Foundations`, and `Tableau` were standardized. This ensures all core components expose a consistent set of methods for card placement, removal, and inspection.

### Standardized Method Signatures

All three components now provide:

```rust
fn place_card(&mut self, location: usize, card: Card) -> Result<(), ErrorType>;
fn remove_card(&mut self, location: usize) -> Result<Option<Card>, ErrorType>;
fn get_card(&self, location: usize) -> Option<&Card>;
```
- `location` is `cell_index`, `pile`, or `column` as appropriate.
- Each component retains its own domain-specific error type (`FreeCellError`, `FoundationError`, `TableauError`).

#### Helper Methods (Consistent Naming)
- `*_count()` for number of locations (cells, piles, columns)
- `empty_*_count()` for number of empty locations
- `is_*_empty(location)` for emptiness check

### Rationale for Domain-Specific Error Types

- **Type Safety & Clarity**: Each component's error enum reflects its unique rules and constraints.
- **Debuggability**: Errors are descriptive and context-specific.
- **Extensibility**: New error variants can be added per component as rules evolve.

### Impact

- **Move execution and undo logic** now use the new interfaces, handling `Result<Option<Card>, ErrorType>` for all removals.
- **Tests** updated to match new signatures and error handling.
- **Documentation** and memory banks updated to reflect this architectural improvement.

### Benefits

- Predictable, uniform API for all card containers
- Easier integration for downstream consumers (UI, solver, automation)
- Clear separation of concerns and error domains

**Note**: While the current interface consistency pattern is strong, the upcoming major version will further refine error handling and return types to ensure all fallible operations consistently return `Result<T, GameError>`, providing even richer context and type safety.

---

## Performance Patterns

### Efficient State Representation
- Cards are small structs (8 bytes each)
- Game state is compact and stack-allocatable
- Move validation is O(1) for most operations
- No unnecessary heap allocations during gameplay

### Lazy Evaluation
```rust
impl GameState {
    pub fn get_available_moves(&self) -> Vec<Move> {
        // Only compute moves when requested
        // Cache results if performance becomes critical
    }
}
```

## Error Handling Patterns

### Enhanced Error System
The `GameError` enum will be significantly enhanced to preserve full context from component-specific errors. This means `GameError` will wrap the original `FreeCellError`, `FoundationError`, or `TableauError`, along with additional context like the attempted `Move` and operation description. This provides rich, debuggable error information without losing the specificity of component-level errors.

```rust
// Proposed new GameError structure
#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    FreeCellError { 
        error: FreeCellError, 
        attempted_move: Option<Move>,
        operation: String,
    },
    FoundationError { 
        error: FoundationError, 
        attempted_move: Option<Move>,
        operation: String,
    },
    TableauError { 
        error: TableauError, 
        attempted_move: Option<Move>,
        operation: String,
    },
    InvalidMove { 
        move_cmd: Move, 
        reason: InvalidMoveReason 
    },
    // ... other game-level errors
}
```

### Graceful Degradation
- Invalid moves return errors without panicking
- Game state remains consistent after any operation
- All public methods handle edge cases gracefully

## Integration Patterns

### Consumer-Friendly API
```rust
// Simple initialization
let mut game = GameState::new();

// Clear move execution
let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
match game.execute_move(&move_cmd) {
    Ok(()) => println!("Move successful"),
    Err(msg) => println!("Invalid move: {}", msg),
}

// Easy state inspection
if game.is_game_won() {
    println!("Congratulations!");
}
```

### Extensibility Points
- Move enum can be extended for new move types
- Rule functions can be composed for complex validations
- Component traits allow for alternative implementations

## Documentation Patterns

### API Documentation
```rust
/// Executes a move if it's valid according to FreeCell rules.
/// 
/// # Arguments
/// * `move_to_make` - The move to attempt
/// 
/// # Returns
/// * `Ok(())` if the move was executed successfully
/// * `Err(String)` with a description if the move is invalid
/// 
/// # Examples
/// ```
/// let mut game = GameState::new();
/// let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
/// game.execute_move(&move_cmd)?;
/// ```
pub fn execute_move(&mut self, move_to_make: &Move) -> Result<(), String>
```

### Internal Documentation
- Complex algorithms documented with inline comments
- Design decisions explained in code comments
- Performance considerations noted where relevant
