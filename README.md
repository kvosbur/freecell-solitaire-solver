# FreeCell Solitaire Solver

This is a Rust workspace containing a shared game engine and multiple applications that consume it.

## Project Structure

```
freecell-solitaire-solver/
├── Cargo.toml                 # Workspace root configuration
├── game-engine/               # Shared library crate
│   ├── Cargo.toml            # Library configuration
│   └── src/                  # Game engine source code
│       ├── lib.rs            # Library entry point
│       ├── card.rs           # Card types and logic
│       ├── game_state.rs     # Game state management
│       ├── tableau.rs        # Tableau logic
│       ├── freecells.rs      # FreeCells logic
│       ├── foundations.rs    # Foundations logic
│       └── rules.rs          # Game rules
├── solver/                   # Solver application
│   ├── Cargo.toml           # Solver configuration
│   └── src/main.rs          # Solver entry point
├── game/                     # Game application
│   ├── Cargo.toml           # Game configuration
│   └── src/main.rs          # Game entry point
└── appAutomation/           # Standalone automation app (separate)
```

## Building and Running

### Build Everything
```bash
cargo build --workspace
```

### Clean Build Artifacts
```bash
# Remove all build artifacts (equivalent to rimraf dist/ in TypeScript)
cargo clean

# Clean specific package only
cargo clean -p freecell-game-engine

# Clean release artifacts only
cargo clean --release

# Dry run - see what would be deleted without actually deleting
cargo clean --dry-run
```

### Run Individual Applications
```bash
# Run the solver
cargo run --bin solver

# Run the game
cargo run --bin game
```

### Run Tests
```bash
# Test the entire workspace
cargo test --workspace

# Test just the game engine
cargo test -p freecell-game-engine
```

## Using the Game Engine

Both the solver and game applications depend on the `freecell-game-engine` library. Here's how to use it:

### Basic Usage

```rust
use freecell_game_engine::{GameState, Card, Suit};

// Create a new game state
let mut game_state = GameState::new();

// Create cards
let card = Card { rank: 1, suit: Suit::Hearts }; // Ace of Hearts

// Access game components
let tableau = &game_state.tableau;
let freecells = &game_state.freecells;
let foundations = &game_state.foundations;

// Check game status
if game_state.is_game_won() {
    println!("Game won!");
}

// Get available moves
let moves = game_state.get_available_moves();

// Execute a move
use freecell_game_engine::Move;
let move_to_make = Move::TableauToFoundation { 
    from_column: 0, 
    to_pile: 0 
};
if let Ok(()) = game_state.execute_move(&move_to_make) {
    println!("Move executed successfully");
}
```

### Available Types

- `GameState`: Main game state containing tableau, freecells, and foundations
- `Card`: Represents a playing card with rank (1-13) and suit
- `Suit`: Enum for card suits (Hearts, Diamonds, Clubs, Spades)
- `Color`: Enum for card colors (Red, Black)
- `Move`: Enum for different types of moves
- `Tableau`: Manages the 8 tableau columns
- `FreeCells`: Manages the 4 free cells
- `Foundations`: Manages the 4 foundation piles

## Adding New Applications

To add a new application that uses the game engine:

1. Create a new directory (e.g., `my-app/`)
2. Add it to the workspace members in the root `Cargo.toml`
3. Create `my-app/Cargo.toml` with dependency on `freecell-game-engine`
4. Create `my-app/src/main.rs` and import the game engine

Example `my-app/Cargo.toml`:
```toml
[package]
name = "my-freecell-app"
version = "0.1.0"
edition = "2021"

[dependencies]
freecell-game-engine = { path = "../game-engine" }
```

## Development Tips

- The game engine is a library crate, so it doesn't have a `main.rs`
- All applications share the same game engine code, ensuring consistency
- Use `cargo check --workspace` for fast compilation checking
- Use `cargo clippy --workspace` for linting suggestions
- The workspace uses Rust 2021 edition for all crates
- **Single `.gitignore`**: The workspace uses one `.gitignore` file at the root level that covers all workspace members, since Cargo creates a single `target/` directory for the entire workspace

## Git Ignore Strategy

For Rust workspaces, it's best practice to use a single `.gitignore` file at the root level rather than individual files in each crate because:

- Cargo creates a single `target/` directory at the workspace root containing build artifacts for all members
- Avoids duplication of ignore rules across multiple files
- Simpler to maintain - one place to update ignore patterns
- Follows standard Rust workspace conventions
