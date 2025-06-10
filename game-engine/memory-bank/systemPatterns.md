# System Patterns: Game Engine Library Architecture

## Library Architecture

### Pure Library Design
The game engine follows a strict library-only architecture:
- **No main.rs**: Pure library crate with no executable entry point
- **No I/O Dependencies**: No user interface, file system, or network operations
- **Pure Functions**: Game logic implemented as stateless, side-effect-free functions
- **API-First**: All functionality exposed through well-defined public interfaces

### Modular Component Structure
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
```rust
pub enum Move {
    TableauToFoundation { from_column: usize, to_pile: usize },
    TableauToFreecell { from_column: usize, to_cell: usize },
    TableauToTableau { from_column: usize, to_column: usize },
    FreecellToFoundation { from_cell: usize, to_pile: usize },
    FreecellToTableau { from_cell: usize, to_column: usize },
}
```

## Rule Engine Patterns

### Pure Function Rule Validation
Each FreeCell rule is implemented as an independent, testable function:
```rust
pub fn can_place_on_tableau(card: &Card, target_card: &Card) -> bool {
    // Alternating colors and descending rank validation
}

pub fn can_place_on_foundation(card: &Card, foundation_top: Option<&Card>) -> bool {
    // Same suit and ascending rank validation
}
```

### Validation Before Mutation
```rust
impl GameState {
    pub fn execute_move(&mut self, move_to_make: &Move) -> Result<(), String> {
        // 1. Validate move using rule functions
        if !self.is_move_valid(move_to_make) {
            return Err("Invalid move".to_string());
        }
        
        // 2. Execute move only after validation passes
        self.apply_move(move_to_make);
        Ok(())
    }
}
```

## API Design Patterns

### Result-Based Error Handling
```rust
pub fn execute_move(&mut self, move_to_make: &Move) -> Result<(), String> {
    // Returns Result for graceful error handling
}

pub fn get_available_moves(&self) -> Vec<Move> {
    // Returns all valid moves for current state
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

## Testing Patterns

### Test-Driven Development
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_tableau_placement() {
        let red_king = Card { rank: 13, suit: Suit::Hearts };
        let black_queen = Card { rank: 12, suit: Suit::Spades };
        assert!(can_place_on_tableau(&black_queen, &red_king));
    }
}
```

### Parameterized Testing
```rust
use rstest::rstest;

#[rstest]
#[case(Card { rank: 1, suit: Suit::Hearts }, None, true)]
#[case(Card { rank: 2, suit: Suit::Hearts }, Some(&Card { rank: 1, suit: Suit::Hearts }), true)]
#[case(Card { rank: 3, suit: Suit::Hearts }, Some(&Card { rank: 1, suit: Suit::Hearts }), false)]
fn test_foundation_placement(
    #[case] card: Card,
    #[case] foundation_top: Option<&Card>,
    #[case] expected: bool
) {
    assert_eq!(can_place_on_foundation(&card, foundation_top), expected);
}
```

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

### Comprehensive Validation
```rust
impl GameState {
    fn validate_move_indices(&self, move_to_make: &Move) -> Result<(), String> {
        // Validate all array indices before accessing
        // Return descriptive error messages
    }
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
