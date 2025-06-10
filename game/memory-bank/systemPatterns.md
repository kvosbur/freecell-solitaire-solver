# System Patterns: Interactive Game Architecture

## Game Application Architecture

### Application Layer Structure
The game application follows a clear layered architecture:

```
User Interface Layer (CLI)
    ↓
Command Processing Layer
    ↓
Game Engine Integration Layer
    ↓
freecell-game-engine (shared library)
```

### Current Implementation Structure
```rust
fn main() {
    // 1. Initialize game engine
    let mut game_state = GameState::new();
    
    // 2. Display initial state
    // TODO: Implement display logic
    
    // 3. Game loop
    // TODO: Implement user input and move processing
    
    // 4. Handle game completion
    // TODO: Implement win/lose handling
}
```

## Interface Design Patterns

### Command Pattern (Planned)
User input will be parsed into command objects:
```rust
enum GameCommand {
    Move(String),      // e.g., "t1f" (tableau 1 to freecell)
    Help,
    Quit,
    NewGame,
    Restart,
}
```

### Display Pattern (Planned)
Game state visualization will follow consistent formatting:
- ASCII art representation of cards and game areas
- Clear separation between tableau, freecells, and foundations
- Status information (moves made, time elapsed, etc.)

### Input Validation Pattern
```rust
fn parse_move_command(input: &str) -> Result<Move, String> {
    // Parse user input into game engine Move enum
    // Validate syntax before attempting game engine validation
}
```

## Game Loop Architecture

### Main Game Loop Pattern
```rust
loop {
    // 1. Display current game state
    display_game_state(&game_state);
    
    // 2. Get user input
    let input = get_user_input();
    
    // 3. Process command
    match parse_command(&input) {
        Ok(GameCommand::Move(move_str)) => {
            // Parse and execute move
        },
        Ok(GameCommand::Quit) => break,
        // Handle other commands
    }
    
    // 4. Check win condition
    if game_state.is_game_won() {
        celebrate_win();
        break;
    }
}
```

### State Management Pattern
- Game state is owned by main loop
- Passed by reference to display functions
- Modified through game engine's validated move system
- No direct state manipulation outside game engine

## User Interface Patterns

### CLI Command Syntax (Planned)
Simple, memorable command format:
- `t1f` - Move from tableau column 1 to freecell
- `f2t3` - Move from freecell 2 to tableau column 3
- `t4fo` - Move from tableau column 4 to foundation
- `help` - Show available commands
- `quit` - Exit game

### Display Layout Pattern
```
FreeCells: [  ] [  ] [  ] [  ]    Foundations: [A♠] [  ] [  ] [  ]

Tableau:
  1     2     3     4     5     6     7     8
[K♥]  [Q♠]  [J♦]  [10♣] [9♥]  [8♠]  [7♦]  [6♣]
[Q♦]  [J♥]  [10♠] [9♦]  [8♣]  [7♥]  [6♠]  [5♦]
...
```

### Error Handling Pattern
```rust
match game_state.execute_move(&parsed_move) {
    Ok(()) => {
        // Move successful, continue
    },
    Err(error_msg) => {
        println!("Invalid move: {}", error_msg);
        // Continue game loop
    }
}
```

## Integration Patterns

### Game Engine Integration
- All game logic delegated to freecell-game-engine
- Application focuses purely on user interface
- Clean separation between presentation and business logic

### Dependency Pattern
```toml
[dependencies]
freecell-game-engine = { path = "../game-engine" }
```

### API Usage Pattern
```rust
use freecell_game_engine::{GameState, Move, Card, Suit};

// Initialize game
let mut game = GameState::new();

// Execute moves
let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
game.execute_move(&move_cmd)?;

// Check game status
if game.is_game_won() {
    // Handle win
}
```

## Error Handling Patterns

### Input Validation Chain
1. **Syntax Validation**: Check command format
2. **Semantic Validation**: Verify move makes sense
3. **Game Rule Validation**: Use game engine validation
4. **User Feedback**: Provide helpful error messages

### Graceful Degradation
- Invalid input doesn't crash the application
- Clear error messages guide user to correct input
- Game state remains consistent after errors

## Performance Patterns

### Efficient Display Updates
- Only redraw when game state changes
- Minimize terminal output for responsiveness
- Cache display strings when possible

### Memory Management
- Game state is stack-allocated when possible
- Minimal heap allocations during gameplay
- Reuse command parsing buffers

## Future Enhancement Patterns

### Extensible Command System
Design commands to be easily extensible:
```rust
trait GameCommand {
    fn execute(&self, game_state: &mut GameState) -> Result<(), String>;
    fn help_text(&self) -> &str;
}
```

### Pluggable Display System
Structure display to allow future TUI/GUI upgrades:
```rust
trait GameDisplay {
    fn show_game_state(&self, state: &GameState);
    fn show_message(&self, message: &str);
}
```

### Undo/Redo Pattern (Future)
```rust
struct GameHistory {
    states: Vec<GameState>,
    current_index: usize,
}
```

## Testing Patterns

### Unit Testing Strategy
- Test command parsing independently
- Test display formatting with known game states
- Mock user input for game loop testing

### Integration Testing
- Test full command execution flow
- Verify game engine integration
- Test error handling paths

## Code Organization Patterns

### Module Structure (Planned)
```
src/
├── main.rs           // Entry point and main game loop
├── display.rs        // Game state display logic
├── input.rs          // User input parsing
├── commands.rs       // Command definitions and parsing
└── game_loop.rs      // Game loop management
```

### Separation of Concerns
- **main.rs**: Application entry and coordination
- **display.rs**: Pure presentation logic
- **input.rs**: Input handling and parsing
- **commands.rs**: Command definitions and validation
- **game_loop.rs**: Game flow control
